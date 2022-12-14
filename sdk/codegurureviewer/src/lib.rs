#![allow(deprecated)]#![allow(clippy::module_inception)]#![allow(clippy::upper_case_acronyms)]#![allow(clippy::large_enum_variant)]#![allow(clippy::wrong_self_convention)]#![allow(clippy::should_implement_trait)]#![allow(clippy::blacklisted_name)]#![allow(clippy::vec_init_then_push)]#![allow(clippy::type_complexity)]#![allow(clippy::needless_return)]#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>This section provides documentation for the Amazon CodeGuru Reviewer API operations. CodeGuru Reviewer is a
//! service that uses program analysis and machine learning to detect potential defects that
//! are difficult for developers to find and recommends fixes in your Java and Python
//! code.</p>
//! 
//! <p>By proactively detecting and providing recommendations for addressing code defects and
//! implementing best practices, CodeGuru Reviewer improves the overall quality and maintainability of
//! your code base during the code review stage. For more information about CodeGuru Reviewer, see the
//! <i>
//! <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-ug/welcome.html">Amazon CodeGuru Reviewer User Guide</a>.</i>
//! </p>
//! 
//! <p>To improve the security of your CodeGuru Reviewer API calls, you can establish a private connection
//! between your VPC and CodeGuru Reviewer by creating an <i>interface VPC endpoint</i>. For
//! more information, see <a href="https://docs.aws.amazon.com/codeguru/latest/reviewer-ug/vpc-interface-endpoints.html">CodeGuru Reviewer and interface VPC
//! endpoints (Amazon Web Services PrivateLink)</a> in the <i>Amazon CodeGuru Reviewer User
//! Guide</i>.</p>
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
static API_METADATA: aws_http::user_agent::ApiMetadata = aws_http::user_agent::ApiMetadata::new("codegurureviewer", PKG_VERSION);
pub use aws_types::app_name::AppName;
#[doc(inline)]pub use client::Client;
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::region::Region;
pub use aws_types::Credentials;

