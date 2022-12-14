#![allow(deprecated)]#![allow(clippy::module_inception)]#![allow(clippy::upper_case_acronyms)]#![allow(clippy::large_enum_variant)]#![allow(clippy::wrong_self_convention)]#![allow(clippy::should_implement_trait)]#![allow(clippy::blacklisted_name)]#![allow(clippy::vec_init_then_push)]#![allow(clippy::type_complexity)]#![allow(clippy::needless_return)]#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>Storage Gateway Service</fullname>
//! 
//! <p>Storage Gateway is the service that connects an on-premises software appliance
//! with cloud-based storage to provide seamless and secure integration between an
//! organization's on-premises IT environment and the Amazon Web Services storage
//! infrastructure. The service enables you to securely upload data to the Amazon Web Services Cloud for cost effective backup and rapid disaster recovery.</p>
//! 
//! <p>Use the following links to get started using the <i>Storage Gateway
//! Service API Reference</i>:</p>
//! 
//! <ul>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/AWSStorageGatewayAPI.html#AWSStorageGatewayHTTPRequestsHeaders">Storage Gateway required request headers</a>: Describes the required
//! headers that you must send with every POST request to Storage Gateway.</p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/AWSStorageGatewayAPI.html#AWSStorageGatewaySigningRequests">Signing requests</a>: Storage Gateway requires that you authenticate
//! every request you send; this topic describes how sign such a request.</p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/AWSStorageGatewayAPI.html#APIErrorResponses">Error responses</a>: Provides reference information about Storage Gateway errors.</p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_Operations.html">Operations in Storage Gateway</a>: Contains detailed descriptions of all Storage Gateway operations, their request parameters, response elements, possible errors, and
//! examples of requests and responses.</p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/general/latest/gr/sg.html">Storage Gateway
//! endpoints and quotas</a>: Provides a list of each Amazon Web Services Region
//! and the endpoints available for use with Storage Gateway.</p>
//! </li>
//! </ul>
//! 
//! <note>
//! <p>Storage Gateway resource IDs are in uppercase. When you use these resource IDs
//! with the Amazon EC2 API, EC2 expects resource IDs in lowercase. You must change
//! your resource ID to lowercase to use it with the EC2 API. For example, in Storage
//! Gateway the ID for a volume might be <code>vol-AA22BB012345DAF670</code>. When you use
//! this ID with the EC2 API, you must change it to <code>vol-aa22bb012345daf670</code>.
//! Otherwise, the EC2 API might not behave as expected.</p>
//! </note>
//! 
//! <important>
//! <p>IDs for Storage Gateway volumes and Amazon EBS snapshots created from gateway
//! volumes are changing to a longer format. Starting in December 2016, all new volumes and
//! snapshots will be created with a 17-character string. Starting in April 2016, you will
//! be able to use these longer IDs so you can test your systems with the new format. For
//! more information, see <a href="http://aws.amazon.com/ec2/faqs/#longer-ids">Longer EC2 and
//! EBS resource IDs</a>.</p>
//! 
//! <p>For example, a volume Amazon Resource Name (ARN) with the longer volume ID format
//! looks like the following:</p>
//! 
//! <p>
//! <code>arn:aws:storagegateway:us-west-2:111122223333:gateway/sgw-12A3456B/volume/vol-1122AABBCCDDEEFFG</code>.</p>
//! 
//! <p>A snapshot ID with the longer ID format looks like the following:
//! <code>snap-78e226633445566ee</code>.</p>
//! 
//! <p>For more information, see <a href="http://forums.aws.amazon.com/ann.jspa?annID=3557">Announcement:
//! Heads-up – Longer Storage Gateway volume and snapshot IDs coming in
//! 2016</a>.</p>
//! </important>
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
static API_METADATA: aws_http::user_agent::ApiMetadata = aws_http::user_agent::ApiMetadata::new("storagegateway", PKG_VERSION);
pub use aws_types::app_name::AppName;
#[doc(inline)]pub use client::Client;
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::region::Region;
pub use aws_types::Credentials;

