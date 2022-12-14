#![allow(deprecated)]#![allow(clippy::module_inception)]#![allow(clippy::upper_case_acronyms)]#![allow(clippy::large_enum_variant)]#![allow(clippy::wrong_self_convention)]#![allow(clippy::should_implement_trait)]#![allow(clippy::blacklisted_name)]#![allow(clippy::vec_init_then_push)]#![allow(clippy::type_complexity)]#![allow(clippy::needless_return)]#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>Amazon Web Services Support</fullname>
//! <p>The <i>Amazon Web Services Support API Reference</i> is intended for programmers who need detailed
//! information about the Amazon Web Services Support operations and data types. You can use the API to manage
//! your support cases programmatically. The Amazon Web Services Support API uses HTTP methods that return
//! results in JSON format.</p>
//! <note>
//! <ul>
//! <li>
//! <p>You must have a Business, Enterprise On-Ramp, or Enterprise Support plan to use the Amazon Web Services Support
//! API. </p>
//! </li>
//! <li>
//! <p>If you call the Amazon Web Services Support API from an account that does not have a
//! Business, Enterprise On-Ramp, or Enterprise Support plan, the
//! <code>SubscriptionRequiredException</code> error message appears. For
//! information about changing your support plan, see <a href="http://aws.amazon.com/premiumsupport/">Amazon Web Services Support</a>.</p>
//! </li>
//! </ul>
//! </note>
//! <p>The Amazon Web Services Support service also exposes a set of <a href="http://aws.amazon.com/premiumsupport/trustedadvisor/">Trusted Advisor</a> features. You can
//! retrieve a list of checks and their descriptions, get check results, specify checks to
//! refresh, and get the refresh status of checks.</p>
//! <p>The following list describes the Amazon Web Services Support case management operations:</p>
//! <ul>
//! <li>
//! <p> Service names, issue categories, and available severity levels  - The
//! <a>DescribeServices</a> and <a>DescribeSeverityLevels</a> operations return Amazon Web Services service names,
//! service codes, service categories, and problem severity levels. You use these
//! values when you call the <a>CreateCase</a> operation.</p>
//! </li>
//! <li>
//! <p> Case creation, case details, and case resolution - The <a>CreateCase</a>, <a>DescribeCases</a>, <a>DescribeAttachment</a>, and <a>ResolveCase</a> operations
//! create Amazon Web Services Support cases, retrieve information about cases, and resolve cases.</p>
//! </li>
//! <li>
//! <p> Case communication - The <a>DescribeCommunications</a>,
//! <a>AddCommunicationToCase</a>, and <a>AddAttachmentsToSet</a> operations retrieve and add communications
//! and attachments to Amazon Web Services Support cases.</p>
//! </li>
//! </ul>
//! <p>The following list describes the operations available from the Amazon Web Services Support service for
//! Trusted Advisor:</p>
//! <ul>
//! <li>
//! <p>
//! <a>DescribeTrustedAdvisorChecks</a> returns the list of checks that
//! run against your Amazon Web Services resources.</p>
//! </li>
//! <li>
//! <p>Using the <code>checkId</code> for a specific check returned by <a>DescribeTrustedAdvisorChecks</a>, you can call <a>DescribeTrustedAdvisorCheckResult</a> to obtain the results for the
//! check that you specified.</p>
//! </li>
//! <li>
//! <p>
//! <a>DescribeTrustedAdvisorCheckSummaries</a> returns summarized
//! results for one or more Trusted Advisor checks.</p>
//! </li>
//! <li>
//! <p>
//! <a>RefreshTrustedAdvisorCheck</a> requests that Trusted Advisor rerun a
//! specified check.</p>
//! </li>
//! <li>
//! <p>
//! <a>DescribeTrustedAdvisorCheckRefreshStatuses</a> reports the refresh
//! status of one or more checks.</p>
//! </li>
//! </ul>
//! <p>For authentication of requests, Amazon Web Services Support uses <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 Signing
//! Process</a>.</p>
//! <p>See <a href="https://docs.aws.amazon.com/awssupport/latest/user/Welcome.html">About the
//! Amazon Web Services Support API</a> in the <i>Amazon Web Services Support User Guide</i> for
//! information about how to use this service to create and manage your support cases, and
//! how to call Trusted Advisor for results of checks on your resources.</p>
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
    pub use aws_smithy_types::Blob;
    pub use aws_smithy_http::result::SdkError;
}
static API_METADATA: aws_http::user_agent::ApiMetadata = aws_http::user_agent::ApiMetadata::new("support", PKG_VERSION);
pub use aws_types::app_name::AppName;
#[doc(inline)]pub use client::Client;
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::region::Region;
pub use aws_types::Credentials;

