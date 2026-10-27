//! Canonical timestamps for generated archives and exports.

use jiff::Timestamp;
use jiff::tz::TimeZone;
use rc_core::{Error, ProcessEnv, Result, source_date_epoch_from_env};
use zip::DateTime;

const MIN_ZIP_YEAR: i16 = 1980;
const MAX_ZIP_YEAR: i16 = 2107;

/// Return the ZIP timestamp used for every generated entry.
///
/// Without `SOURCE_DATE_EPOCH` the ZIP epoch (1980-01-01) is used, which is the
/// same default the `zip` crate applies. With it, the UTC date and time are
/// honored; values outside the representable ZIP range are clamped so an
/// export never fails only because a reproducible-build timestamp predates the
/// format.
pub(super) fn canonical_zip_datetime() -> Result<DateTime> {
    match source_date_epoch_from_env(&ProcessEnv)? {
        Some(timestamp) => zip_datetime(timestamp),
        None => Ok(DateTime::default()),
    }
}

fn zip_datetime(timestamp: Timestamp) -> Result<DateTime> {
    let zoned = timestamp.to_zoned(TimeZone::UTC);
    let date = zoned.date();
    let time = zoned.time();

    if date.year() < MIN_ZIP_YEAR {
        return Ok(DateTime::default());
    }
    if date.year() > MAX_ZIP_YEAR {
        return Ok(DateTime::from_date_and_time(
            u16::try_from(MAX_ZIP_YEAR).unwrap_or_default(),
            12,
            31,
            23,
            59,
            58,
        )
        .unwrap_or_else(|_| DateTime::default()));
    }

    let year = u16::try_from(date.year()).map_err(|_| {
        Error::Config("SOURCE_DATE_EPOCH produced an invalid archive year".to_string())
    })?;
    let month = u8::try_from(date.month()).map_err(|_| {
        Error::Config("SOURCE_DATE_EPOCH produced an invalid archive month".to_string())
    })?;
    let day = u8::try_from(date.day()).map_err(|_| {
        Error::Config("SOURCE_DATE_EPOCH produced an invalid archive day".to_string())
    })?;
    let hour = u8::try_from(time.hour()).map_err(|_| {
        Error::Config("SOURCE_DATE_EPOCH produced an invalid archive hour".to_string())
    })?;
    let minute = u8::try_from(time.minute()).map_err(|_| {
        Error::Config("SOURCE_DATE_EPOCH produced an invalid archive minute".to_string())
    })?;
    let second = u8::try_from(time.second()).map_err(|_| {
        Error::Config("SOURCE_DATE_EPOCH produced an invalid archive second".to_string())
    })?;

    DateTime::from_date_and_time(year, month, day, hour, minute, second).map_err(|error| {
        Error::Config(format!(
            "SOURCE_DATE_EPOCH is not a valid ZIP time: {error}"
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zip_datetime_uses_the_reproducible_utc_timestamp() {
        let timestamp = Timestamp::from_second(1_700_000_000).expect("valid timestamp");
        let datetime = zip_datetime(timestamp).expect("valid zip time");

        assert_eq!(datetime.year(), 2023);
        assert_eq!(datetime.month(), 11);
        assert_eq!(datetime.day(), 14);
        assert_eq!(datetime.hour(), 22);
        assert_eq!(datetime.minute(), 13);
        assert_eq!(datetime.second(), 20);
    }

    #[test]
    fn zip_datetime_clamps_outside_the_zip_calendar_range() {
        let before = Timestamp::from_second(0).expect("epoch");
        assert_eq!(zip_datetime(before).expect("clamped"), DateTime::default());

        let after = Timestamp::from_second(7_258_118_400).expect("year 2200");
        let clamped = zip_datetime(after).expect("clamped");
        assert_eq!(clamped.year(), 2107);
        assert_eq!(clamped.month(), 12);
        assert_eq!(clamped.day(), 31);
    }
}
