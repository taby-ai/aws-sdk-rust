#![allow(deprecated)]#![allow(clippy::module_inception)]#![allow(clippy::upper_case_acronyms)]#![allow(clippy::large_enum_variant)]#![allow(clippy::wrong_self_convention)]#![allow(clippy::should_implement_trait)]#![allow(clippy::blacklisted_name)]#![allow(clippy::vec_init_then_push)]#![allow(clippy::type_complexity)]#![allow(clippy::needless_return)]#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>Security Token Service</fullname>
//! <p>Security Token Service (STS) enables you to request temporary, limited-privilege 
//! credentials for Identity and Access Management (IAM) users or for users that you 
//! authenticate (federated users). This guide provides descriptions of the STS API. For 
//! more information about using this service, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp.html">Temporary Security Credentials</a>.</p>
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
/// Input structures for operations.
pub mod input;
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
mod query_ser;
mod rest_xml_wrapped_errors;
mod xml_deser;
/// Crate version number.
                    pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_types::DateTime;
    pub use aws_smithy_http::result::SdkError;
}
static API_METADATA: aws_http::user_agent::ApiMetadata = aws_http::user_agent::ApiMetadata::new("sts", PKG_VERSION);
pub use aws_types::app_name::AppName;
#[doc(inline)]pub use client::Client;
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::region::Region;
pub use aws_types::Credentials;

