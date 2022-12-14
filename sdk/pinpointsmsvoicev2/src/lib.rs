#![allow(deprecated)]#![allow(clippy::module_inception)]#![allow(clippy::upper_case_acronyms)]#![allow(clippy::large_enum_variant)]#![allow(clippy::wrong_self_convention)]#![allow(clippy::should_implement_trait)]#![allow(clippy::blacklisted_name)]#![allow(clippy::vec_init_then_push)]#![allow(clippy::type_complexity)]#![allow(clippy::needless_return)]#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>Welcome to the <i>Amazon Pinpoint SMS and Voice, version 2 API Reference</i>.
//! This guide provides information about Amazon Pinpoint SMS and Voice, version 2 API
//! resources, including supported HTTP methods, parameters, and schemas.</p>
//! <p>Amazon Pinpoint is an Amazon Web Services service that you can use to engage with
//! your recipients across multiple messaging channels. The Amazon Pinpoint SMS and
//! Voice, version 2 API provides programmatic access to options that are unique to the SMS
//! and voice channels and supplements the resources provided by the Amazon Pinpoint
//! API.</p>
//! <p>If you're new to Amazon Pinpoint, it's also helpful to review the <a href="https://docs.aws.amazon.com/pinpoint/latest/developerguide/welcome.html">
//! Amazon Pinpoint Developer Guide</a>. The <i>Amazon Pinpoint
//! Developer Guide</i> provides tutorials, code samples, and procedures that
//! demonstrate how to use Amazon Pinpoint features programmatically and how to integrate
//! Amazon Pinpoint functionality into mobile apps and other types of applications.
//! The guide also provides key information, such as Amazon Pinpoint integration with
//! other Amazon Web Services services, and the quotas that apply to use of the
//! service.</p>
//! 
//! # Crate Organization
//! 
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//! 
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//! 
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//! 
//! The other modules within this crate are not required for normal usage.


// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
pub mod client;
/// Configuration for the service.
pub mod config;
/// All error types that operations can return.
pub mod error;
mod error_meta;
mod idempotency_token;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
/// Generated accessors for nested fields
pub mod lens;
pub mod middleware;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Paginators for the service
pub mod paginator;
/// Crate version number.
                    pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_types::DateTime;
    pub use aws_smithy_http::result::SdkError;
}
static API_METADATA: aws_http::user_agent::ApiMetadata = aws_http::user_agent::ApiMetadata::new("pinpointsmsvoicev2", PKG_VERSION);
pub use aws_types::app_name::AppName;
#[doc(inline)]pub use client::Client;
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::region::Region;
pub use aws_types::Credentials;

