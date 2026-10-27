//! Runtime ports for time, entropy and environment access.
//!
//! Command and library code receives these ports instead of reading the
//! process clock, the OS random generator or the environment directly. Real
//! implementations are only wired at the process boundary
//! ([`SystemClock`], [`OsEntropy`], [`ProcessEnv`]); deterministic
//! implementations ([`FixedClock`], [`SeededEntropy`], [`MapEnv`]) are used by
//! tests and reproducibility checks.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fmt::Debug;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use jiff::Timestamp;

use crate::error::{Error, Result};

/// Environment variable that freezes generated artifact timestamps.
pub const SOURCE_DATE_EPOCH_ENV: &str = "SOURCE_DATE_EPOCH";

/// Source of wall-clock and monotonic time.
pub trait Clock: Debug + Send + Sync {
    /// Current UTC instant.
    fn now_utc(&self) -> Timestamp;

    /// Monotonic instant used only for elapsed-time measurements.
    fn now_monotonic(&self) -> Instant;
}

/// Real process clock. Use only at the process boundary.
#[derive(Debug, Default, Clone, Copy)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_utc(&self) -> Timestamp {
        Timestamp::now()
    }

    fn now_monotonic(&self) -> Instant {
        Instant::now()
    }
}

/// Frozen clock with explicit advancement for deterministic tests.
#[derive(Debug, Clone)]
pub struct FixedClock {
    utc: Arc<Mutex<Timestamp>>,
    monotonic: Arc<Mutex<Instant>>,
}

impl FixedClock {
    /// Create a clock pinned to `utc`; monotonic time starts at zero elapsed.
    pub fn new(utc: Timestamp) -> Self {
        Self {
            utc: Arc::new(Mutex::new(utc)),
            monotonic: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// Create a clock pinned to a Unix timestamp in seconds.
    pub fn from_unix_seconds(seconds: i64) -> Result<Self> {
        let utc = Timestamp::from_second(seconds).map_err(|error| {
            Error::Config(format!(
                "Fixed clock is outside the supported UTC range: {error}"
            ))
        })?;
        Ok(Self::new(utc))
    }

    /// Advance both the wall clock and the monotonic instant.
    pub fn advance(&self, by: Duration) {
        let mut utc = self.utc.lock().unwrap_or_else(PoisonError::into_inner);
        if let Ok(advanced) = utc.checked_add(by) {
            *utc = advanced;
        }
        let mut monotonic = self
            .monotonic
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if let Some(advanced) = monotonic.checked_add(by) {
            *monotonic = advanced;
        }
    }
}

impl Clock for FixedClock {
    fn now_utc(&self) -> Timestamp {
        *self.utc.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn now_monotonic(&self) -> Instant {
        *self
            .monotonic
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }
}

/// Parse a `SOURCE_DATE_EPOCH` value into a UTC timestamp.
pub fn source_date_epoch_timestamp(raw: &str) -> Result<Timestamp> {
    let seconds = raw.parse::<i64>().map_err(|error| {
        Error::Config(format!("SOURCE_DATE_EPOCH must be Unix seconds: {error}"))
    })?;
    Timestamp::from_second(seconds).map_err(|error| {
        Error::Config(format!(
            "SOURCE_DATE_EPOCH is outside the supported UTC range: {error}"
        ))
    })
}

/// Read `SOURCE_DATE_EPOCH` through an injected environment.
pub fn source_date_epoch_from_env(env: &dyn Env) -> Result<Option<Timestamp>> {
    env.var(SOURCE_DATE_EPOCH_ENV)
        .map(|raw| source_date_epoch_timestamp(&raw))
        .transpose()
}

/// Source of cryptographic randomness.
pub trait Entropy: Debug + Send + Sync {
    /// Fill `destination` with random bytes.
    fn fill_bytes(&self, destination: &mut [u8]) -> Result<()>;
}

/// OS-backed entropy. Use only at the process boundary.
#[derive(Debug, Default, Clone, Copy)]
pub struct OsEntropy;

impl Entropy for OsEntropy {
    fn fill_bytes(&self, destination: &mut [u8]) -> Result<()> {
        getrandom::fill(destination)
            .map_err(|_| Error::General("Failed to gather random bytes".to_string()))
    }
}

/// Deterministic, seedable entropy used by tests and reproducibility checks.
#[derive(Debug, Clone)]
pub struct SeededEntropy {
    state: Arc<AtomicU64>,
}

impl SeededEntropy {
    /// Create a generator whose output depends only on `seed` and call order.
    pub fn new(seed: u64) -> Self {
        Self {
            state: Arc::new(AtomicU64::new(seed)),
        }
    }
}

impl Entropy for SeededEntropy {
    fn fill_bytes(&self, destination: &mut [u8]) -> Result<()> {
        for chunk in destination.chunks_mut(8) {
            let mut value = self
                .state
                .fetch_add(1, Ordering::SeqCst)
                .wrapping_add(0x9e37_79b9_7f4a_7c15);
            value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            value ^= value >> 31;
            for (slot, byte) in chunk.iter_mut().zip(value.to_le_bytes()) {
                *slot = byte;
            }
        }
        Ok(())
    }
}

/// Source of process environment values.
pub trait Env: Debug + Send + Sync {
    /// Return a variable as an OS string.
    fn var_os(&self, key: &str) -> Option<OsString>;

    /// Return a variable as UTF-8 when it is valid.
    fn var(&self, key: &str) -> Option<String> {
        self.var_os(key)?.into_string().ok()
    }

    /// Return every variable, sorted by key for deterministic iteration.
    fn vars_os(&self) -> Vec<(OsString, OsString)>;
}

/// Real process environment. Use only at the process boundary.
#[derive(Debug, Default, Clone, Copy)]
pub struct ProcessEnv;

impl Env for ProcessEnv {
    fn var_os(&self, key: &str) -> Option<OsString> {
        std::env::var_os(key)
    }

    fn vars_os(&self) -> Vec<(OsString, OsString)> {
        let mut variables: Vec<_> = std::env::vars_os().collect();
        variables.sort_by(|left, right| left.0.cmp(&right.0));
        variables
    }
}

/// In-memory environment for deterministic tests.
#[derive(Debug, Default, Clone)]
pub struct MapEnv {
    variables: BTreeMap<OsString, OsString>,
}

impl MapEnv {
    /// Build an environment from key/value pairs.
    pub fn new<I, K, V>(variables: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<OsString>,
        V: Into<OsString>,
    {
        Self {
            variables: variables
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        }
    }
}

impl Env for MapEnv {
    fn var_os(&self, key: &str) -> Option<OsString> {
        self.variables.get(&OsString::from(key)).cloned()
    }

    fn vars_os(&self) -> Vec<(OsString, OsString)> {
        self.variables
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_clock_advances_wall_and_monotonic_time() {
        let clock = FixedClock::from_unix_seconds(1_700_000_000).expect("valid timestamp");
        let before = clock.now_monotonic();
        clock.advance(Duration::from_millis(25));

        assert_eq!(clock.now_utc().as_second(), 1_700_000_000);
        assert_eq!(clock.now_utc().subsec_nanosecond(), 25_000_000);
        assert_eq!(
            clock.now_monotonic().duration_since(before),
            Duration::from_millis(25)
        );
    }

    #[test]
    fn seeded_entropy_is_reproducible_and_changes_with_seed() {
        let first = SeededEntropy::new(7);
        let second = SeededEntropy::new(7);
        let other = SeededEntropy::new(8);
        let mut left = [0_u8; 32];
        let mut right = [0_u8; 32];
        let mut different = [0_u8; 32];

        first.fill_bytes(&mut left).expect("fill");
        second.fill_bytes(&mut right).expect("fill");
        other.fill_bytes(&mut different).expect("fill");

        assert_eq!(left, right);
        assert_ne!(left, different);
        assert_ne!(left, [0_u8; 32]);
    }

    #[test]
    fn map_env_reads_are_sorted_and_typed() {
        let env = MapEnv::new([("B", "2"), ("A", "1")]);
        assert_eq!(env.var("A").as_deref(), Some("1"));
        assert_eq!(env.var("missing"), None);
        assert_eq!(
            env.vars_os(),
            vec![("A".into(), "1".into()), ("B".into(), "2".into())]
        );
    }

    #[test]
    fn source_date_epoch_requires_valid_seconds() {
        assert_eq!(
            source_date_epoch_timestamp("0").expect("valid").as_second(),
            0
        );
        assert!(source_date_epoch_timestamp("not-a-number").is_err());
        assert!(source_date_epoch_timestamp("99999999999999999999").is_err());
    }

    #[test]
    fn source_date_epoch_from_env_is_optional() {
        let unset = MapEnv::default();
        assert_eq!(
            source_date_epoch_from_env(&unset).expect("unset is valid"),
            None
        );

        let set = MapEnv::new([(SOURCE_DATE_EPOCH_ENV, "1700000000")]);
        let timestamp = source_date_epoch_from_env(&set)
            .expect("valid epoch")
            .expect("present");
        assert_eq!(timestamp.as_second(), 1_700_000_000);
    }
}
