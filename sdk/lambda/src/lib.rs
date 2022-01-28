#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>Lambda</fullname>
//! <p>
//! <b>Overview</b>
//! </p>
//! <p>Lambda is a compute service that lets you run code without provisioning or managing servers.
//! Lambda runs your code on a high-availability compute infrastructure and performs all of the
//! administration of the compute resources, including server and operating system maintenance, capacity provisioning
//! and automatic scaling, code monitoring and logging. With Lambda, you can run code for virtually any
//! type of application or backend service. For more information about the Lambda service, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/welcome.html">What is Lambda</a> in the <b>Lambda Developer Guide</b>.</p>
//! <p>The <i>Lambda API Reference</i> provides information about
//! each of the API methods, including details about the parameters in each API request and
//! response. </p>
//! <p></p>
//! <p>You can use Software Development Kits (SDKs), Integrated Development Environment (IDE) Toolkits, and command
//! line tools to access the API. For installation instructions, see <a href="http://aws.amazon.com/tools/">Tools for
//! Amazon Web Services</a>. </p>
//! <p>For a list of Region-specific endpoints that Lambda supports,
//! see <a href="https://docs.aws.amazon.com/general/latest/gr/lambda-service.html/">Lambda
//! endpoints and quotas </a> in the <i>Amazon Web Services General Reference.</i>. </p>   
//! <p>When making the API calls, you will need to
//! authenticate your request by providing a signature. Lambda supports signature version 4. For more information,
//! see <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 signing process</a> in the
//! <i>Amazon Web Services General Reference.</i>. </p>
//! <p>
//! <b>CA certificates</b>
//! </p>
//!
//! <p>Because Amazon Web Services SDKs use the CA certificates from your computer, changes to the certificates on the Amazon Web Services servers
//! can cause connection failures when you attempt to use an SDK. You can prevent these failures by keeping your
//! computer's CA certificates and operating system up-to-date. If you encounter this issue in a corporate
//! environment and do not manage your own computer, you might need to ask an administrator to assist with the
//! update process. The following list shows minimum operating system and Java versions:</p>
//! <ul>
//! <li>
//! <p>Microsoft Windows versions that have updates from January 2005 or later installed contain at least one
//! of the required CAs in their trust list. </p>
//! </li>
//! <li>
//! <p>Mac OS X 10.4 with Java for Mac OS X 10.4 Release 5 (February 2007), Mac OS X 10.5 (October 2007), and
//! later versions contain at least one of the required CAs in their trust list. </p>
//! </li>
//! <li>
//! <p>Red Hat Enterprise Linux 5 (March 2007), 6, and 7 and CentOS 5, 6, and 7 all contain at least one of the
//! required CAs in their default trusted CA list. </p>
//! </li>
//! <li>
//! <p>Java 1.4.2_12 (May 2006), 5 Update 2 (March 2005), and all later versions, including Java 6 (December
//! 2006), 7, and 8, contain at least one of the required CAs in their default trusted CA list. </p>
//! </li>
//! </ul>
//! <p>When accessing the Lambda management console or Lambda API endpoints, whether through browsers or
//! programmatically, you will need to ensure your client machines support any of the following CAs: </p>
//! <ul>
//! <li>
//! <p>Amazon Root CA 1</p>
//! </li>
//! <li>
//! <p>Starfield Services Root Certificate Authority - G2</p>
//! </li>
//! <li>
//! <p>Starfield Class 2 Certification Authority</p>
//! </li>
//! </ul>
//! <p>Root certificates from the first two authorities are available from <a href="https://www.amazontrust.com/repository/">Amazon trust services</a>, but keeping your computer
//! up-to-date is the more straightforward solution. To learn more about ACM-provided certificates, see <a href="http://aws.amazon.com/certificate-manager/faqs/#certificates">Amazon Web Services Certificate Manager FAQs.</a>
//! </p>
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
//!
//! # Examples
//! Examples can be found [here](https://github.com/awslabs/aws-sdk-rust/tree/main/examples/lambda).

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
pub mod client;
/// Configuration for the service.
pub mod config;
/// Errors that can occur when calling the service.
pub mod error;
mod error_meta;
mod http_serde;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
/// Generated accessors for nested fields
mod lens;
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
    pub use aws_smithy_http::byte_stream::AggregatedBytes;
    pub use aws_smithy_http::byte_stream::ByteStream;
    pub use aws_smithy_http::result::SdkError;
    pub use aws_smithy_types::Blob;
    pub use aws_smithy_types::DateTime;
}
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("lambda", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
