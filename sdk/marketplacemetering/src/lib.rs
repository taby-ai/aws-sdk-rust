#![allow(deprecated)]#![allow(clippy::module_inception)]#![allow(clippy::upper_case_acronyms)]#![allow(clippy::large_enum_variant)]#![allow(clippy::wrong_self_convention)]#![allow(clippy::should_implement_trait)]#![allow(clippy::blacklisted_name)]#![allow(clippy::vec_init_then_push)]#![allow(clippy::type_complexity)]#![allow(clippy::needless_return)]#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>AWS Marketplace Metering Service</fullname>
//! <p>This reference provides descriptions of the low-level AWS Marketplace Metering Service
//! API.</p>
//! <p>AWS Marketplace sellers can use this API to submit usage data for custom usage
//! dimensions.</p>
//! <p>For information on the permissions you need to use this API, see <a href="https://docs.aws.amazon.com/marketplace/latest/userguide/iam-user-policy-for-aws-marketplace-actions.html">AWS Marketplace metering and entitlement API permissions</a> in the
//! <i>AWS Marketplace Seller Guide.</i>
//! </p>
//! <p>
//! <b>Submitting Metering Records</b>
//! </p>
//! <ul>
//! <li>
//! <p>
//! <i>MeterUsage</i> - Submits the metering record for an AWS
//! Marketplace product. <code>MeterUsage</code> is called from an EC2 instance or a
//! container running on EKS or ECS.</p>
//! </li>
//! <li>
//! <p>
//! <i>BatchMeterUsage</i> - Submits the metering record for a set of
//! customers. <code>BatchMeterUsage</code> is called from a software-as-a-service
//! (SaaS) application.</p>
//! </li>
//! </ul>
//! <p>
//! <b>Accepting New Customers</b>
//! </p>
//! <ul>
//! <li>
//! <p>
//! <i>ResolveCustomer</i> - Called by a SaaS application during the
//! registration process. When a buyer visits your website during the registration
//! process, the buyer submits a Registration Token through the browser. The
//! Registration Token is resolved through this API to obtain a
//! <code>CustomerIdentifier</code>
//! 
//! along with the <code>CustomerAWSAccountId</code> and
//! <code>ProductCode</code>.</p>
//! </li>
//! </ul>
//! <p>
//! <b>Entitlement and Metering for Paid Container Products</b>
//! </p>
//! <ul>
//! <li>
//! <p>Paid container software products sold through AWS Marketplace must integrate
//! with the AWS Marketplace Metering Service and call the
//! <code>RegisterUsage</code> operation for software entitlement and metering.
//! Free and BYOL products for Amazon ECS or Amazon EKS aren't required to call
//! <code>RegisterUsage</code>, but you can do so if you want to receive usage
//! data in your seller reports. For more information on using the
//! <code>RegisterUsage</code> operation, see <a href="https://docs.aws.amazon.com/marketplace/latest/userguide/container-based-products.html">Container-Based Products</a>. </p>
//! </li>
//! </ul>
//! <p>
//! <code>BatchMeterUsage</code> API calls are captured by AWS CloudTrail. You can use
//! Cloudtrail to verify that the SaaS metering records that you sent are accurate by
//! searching for records with the <code>eventName</code> of <code>BatchMeterUsage</code>.
//! You can also use CloudTrail to audit records over time. For more information, see the
//! <i>
//! <a href="http://docs.aws.amazon.com/awscloudtrail/latest/userguide/cloudtrail-concepts.html">AWS CloudTrail User Guide</a>.</i>
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
/// Crate version number.
                    pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Re-exported types from supporting crates.
pub mod types {
    pub use aws_smithy_types::DateTime;
    pub use aws_smithy_http::result::SdkError;
}
static API_METADATA: aws_http::user_agent::ApiMetadata = aws_http::user_agent::ApiMetadata::new("marketplacemetering", PKG_VERSION);
pub use aws_types::app_name::AppName;
#[doc(inline)]pub use client::Client;
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::region::Region;
pub use aws_types::Credentials;

