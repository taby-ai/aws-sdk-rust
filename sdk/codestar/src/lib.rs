#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_return)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <fullname>AWS CodeStar</fullname>
//! <p>This is the API reference for AWS CodeStar. This reference provides descriptions of the
//! operations and data types for the AWS CodeStar API along with usage examples.</p>
//! <p>You can use the AWS CodeStar API to work with:</p>
//! <p>Projects and their resources, by calling the following:</p>
//! <ul>
//! <li>
//! <p>
//! <code>DeleteProject</code>, which deletes a project.</p>
//! </li>
//! <li>
//! <p>
//! <code>DescribeProject</code>, which lists the attributes of a project.</p>
//! </li>
//! <li>
//! <p>
//! <code>ListProjects</code>, which lists all projects associated with your AWS
//! account.</p>
//! </li>
//! <li>
//! <p>
//! <code>ListResources</code>, which lists the resources associated with a
//! project.</p>
//! </li>
//! <li>
//! <p>
//! <code>ListTagsForProject</code>, which lists the tags associated with a
//! project.</p>
//! </li>
//! <li>
//! <p>
//! <code>TagProject</code>, which adds tags to a project.</p>
//! </li>
//! <li>
//! <p>
//! <code>UntagProject</code>, which removes tags from a project.</p>
//! </li>
//! <li>
//! <p>
//! <code>UpdateProject</code>, which updates the attributes of a project.</p>
//! </li>
//! </ul>
//! <p>Teams and team members, by calling the following:</p>
//! <ul>
//! <li>
//! <p>
//! <code>AssociateTeamMember</code>, which adds an IAM user to the team for a
//! project.</p>
//! </li>
//! <li>
//! <p>
//! <code>DisassociateTeamMember</code>, which removes an IAM user from the team for a
//! project.</p>
//! </li>
//! <li>
//! <p>
//! <code>ListTeamMembers</code>, which lists all the IAM users in the team for a
//! project, including their roles and attributes.</p>
//! </li>
//! <li>
//! <p>
//! <code>UpdateTeamMember</code>, which updates a team member's attributes in a
//! project.</p>
//! </li>
//! </ul>
//! <p>Users, by calling the following:</p>
//! <ul>
//! <li>
//! <p>
//! <code>CreateUserProfile</code>, which creates a user profile that contains data
//! associated with the user across all projects.</p>
//! </li>
//! <li>
//! <p>
//! <code>DeleteUserProfile</code>, which deletes all user profile information across
//! all projects.</p>
//! </li>
//! <li>
//! <p>
//! <code>DescribeUserProfile</code>, which describes the profile of a user.</p>
//! </li>
//! <li>
//! <p>
//! <code>ListUserProfiles</code>, which lists all user profiles.</p>
//! </li>
//! <li>
//! <p>
//! <code>UpdateUserProfile</code>, which updates the profile for a user.</p>
//! </li>
//! </ul>
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

#[doc(inline)]
pub use config::Config;

/// Client and fluent builders for calling the service.
pub mod client;

/// Configuration for the service.
pub mod config;

/// Endpoint resolution functionality
pub mod endpoint;

/// All error types that operations can return. Documentation on these types is copied from the model.
pub mod error;

mod error_meta;

/// Input structures for operations. Documentation on these types is copied from the model.
pub mod input;

/// Data structures used by operation inputs/outputs. Documentation on these types is copied from the model.
pub mod model;

/// All operations that this crate can perform.
pub mod operation;

/// Output structures for operations. Documentation on these types is copied from the model.
pub mod output;

/// Data primitives referenced by other data types.
pub mod types;

pub mod middleware;

mod no_credentials;

mod operation_deser;

mod operation_ser;

mod json_deser;

mod json_ser;

/// Endpoints standard library functions
mod endpoint_lib;

mod json_errors;

/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("codestar", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
