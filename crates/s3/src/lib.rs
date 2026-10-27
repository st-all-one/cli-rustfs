//! rc-s3: S3 SDK adapter for rc CLI client
//!
//! This crate provides the implementation of the ObjectStore trait
//! using the aws-sdk-s3 crate. It is the only crate that directly
//! depends on the AWS SDK.

#![cfg_attr(
    test,
    allow(
        clippy::expect_used,
        clippy::expect_fun_call,
        clippy::unwrap_used,
        clippy::panic,
        clippy::indexing_slicing,
        clippy::large_stack_arrays,
        let_underscore_drop,
        reason = "unit-test fixtures fail fast with actionable diagnostics"
    )
)]

pub mod admin;
pub mod client;
mod lifecycle_xml;
pub mod multipart;
mod ops;
mod select;
mod watch;

pub use admin::AdminClient;
pub use client::{DeleteRequestOptions, S3Client};
pub use multipart::{MultipartConfig, UploadState};
