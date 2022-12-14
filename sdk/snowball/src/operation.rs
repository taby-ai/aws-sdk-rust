// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CancelCluster`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`cancel_cluster`](crate::client::Client::cancel_cluster).
            ///
            /// See [`crate::client::fluent_builders::CancelCluster`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct CancelCluster {
    _private: ()
}
impl CancelCluster {
    /// Creates a new builder-style object to manufacture [`CancelClusterInput`](crate::input::CancelClusterInput).
    pub fn builder() -> crate::input::cancel_cluster_input::Builder {
        crate::input::cancel_cluster_input::Builder::default()
    }
    /// Creates a new `CancelCluster` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CancelCluster {
                type Output = std::result::Result<crate::output::CancelClusterOutput, crate::error::CancelClusterError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_cancel_cluster_error(response)
                     } else {
                        crate::operation_deser::parse_cancel_cluster_response(response)
                     }
                }
            }

/// Operation shape for `CancelJob`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`cancel_job`](crate::client::Client::cancel_job).
            ///
            /// See [`crate::client::fluent_builders::CancelJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct CancelJob {
    _private: ()
}
impl CancelJob {
    /// Creates a new builder-style object to manufacture [`CancelJobInput`](crate::input::CancelJobInput).
    pub fn builder() -> crate::input::cancel_job_input::Builder {
        crate::input::cancel_job_input::Builder::default()
    }
    /// Creates a new `CancelJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CancelJob {
                type Output = std::result::Result<crate::output::CancelJobOutput, crate::error::CancelJobError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_cancel_job_error(response)
                     } else {
                        crate::operation_deser::parse_cancel_job_response(response)
                     }
                }
            }

/// Operation shape for `CreateAddress`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_address`](crate::client::Client::create_address).
            ///
            /// See [`crate::client::fluent_builders::CreateAddress`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct CreateAddress {
    _private: ()
}
impl CreateAddress {
    /// Creates a new builder-style object to manufacture [`CreateAddressInput`](crate::input::CreateAddressInput).
    pub fn builder() -> crate::input::create_address_input::Builder {
        crate::input::create_address_input::Builder::default()
    }
    /// Creates a new `CreateAddress` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateAddress {
                type Output = std::result::Result<crate::output::CreateAddressOutput, crate::error::CreateAddressError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_address_error(response)
                     } else {
                        crate::operation_deser::parse_create_address_response(response)
                     }
                }
            }

/// Operation shape for `CreateCluster`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_cluster`](crate::client::Client::create_cluster).
            ///
            /// See [`crate::client::fluent_builders::CreateCluster`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct CreateCluster {
    _private: ()
}
impl CreateCluster {
    /// Creates a new builder-style object to manufacture [`CreateClusterInput`](crate::input::CreateClusterInput).
    pub fn builder() -> crate::input::create_cluster_input::Builder {
        crate::input::create_cluster_input::Builder::default()
    }
    /// Creates a new `CreateCluster` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateCluster {
                type Output = std::result::Result<crate::output::CreateClusterOutput, crate::error::CreateClusterError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_cluster_error(response)
                     } else {
                        crate::operation_deser::parse_create_cluster_response(response)
                     }
                }
            }

/// Operation shape for `CreateJob`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_job`](crate::client::Client::create_job).
            ///
            /// See [`crate::client::fluent_builders::CreateJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct CreateJob {
    _private: ()
}
impl CreateJob {
    /// Creates a new builder-style object to manufacture [`CreateJobInput`](crate::input::CreateJobInput).
    pub fn builder() -> crate::input::create_job_input::Builder {
        crate::input::create_job_input::Builder::default()
    }
    /// Creates a new `CreateJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateJob {
                type Output = std::result::Result<crate::output::CreateJobOutput, crate::error::CreateJobError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_job_error(response)
                     } else {
                        crate::operation_deser::parse_create_job_response(response)
                     }
                }
            }

/// Operation shape for `CreateLongTermPricing`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_long_term_pricing`](crate::client::Client::create_long_term_pricing).
            ///
            /// See [`crate::client::fluent_builders::CreateLongTermPricing`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct CreateLongTermPricing {
    _private: ()
}
impl CreateLongTermPricing {
    /// Creates a new builder-style object to manufacture [`CreateLongTermPricingInput`](crate::input::CreateLongTermPricingInput).
    pub fn builder() -> crate::input::create_long_term_pricing_input::Builder {
        crate::input::create_long_term_pricing_input::Builder::default()
    }
    /// Creates a new `CreateLongTermPricing` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateLongTermPricing {
                type Output = std::result::Result<crate::output::CreateLongTermPricingOutput, crate::error::CreateLongTermPricingError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_long_term_pricing_error(response)
                     } else {
                        crate::operation_deser::parse_create_long_term_pricing_response(response)
                     }
                }
            }

/// Operation shape for `CreateReturnShippingLabel`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_return_shipping_label`](crate::client::Client::create_return_shipping_label).
            ///
            /// See [`crate::client::fluent_builders::CreateReturnShippingLabel`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct CreateReturnShippingLabel {
    _private: ()
}
impl CreateReturnShippingLabel {
    /// Creates a new builder-style object to manufacture [`CreateReturnShippingLabelInput`](crate::input::CreateReturnShippingLabelInput).
    pub fn builder() -> crate::input::create_return_shipping_label_input::Builder {
        crate::input::create_return_shipping_label_input::Builder::default()
    }
    /// Creates a new `CreateReturnShippingLabel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateReturnShippingLabel {
                type Output = std::result::Result<crate::output::CreateReturnShippingLabelOutput, crate::error::CreateReturnShippingLabelError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_return_shipping_label_error(response)
                     } else {
                        crate::operation_deser::parse_create_return_shipping_label_response(response)
                     }
                }
            }

/// Operation shape for `DescribeAddress`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_address`](crate::client::Client::describe_address).
            ///
            /// See [`crate::client::fluent_builders::DescribeAddress`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct DescribeAddress {
    _private: ()
}
impl DescribeAddress {
    /// Creates a new builder-style object to manufacture [`DescribeAddressInput`](crate::input::DescribeAddressInput).
    pub fn builder() -> crate::input::describe_address_input::Builder {
        crate::input::describe_address_input::Builder::default()
    }
    /// Creates a new `DescribeAddress` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeAddress {
                type Output = std::result::Result<crate::output::DescribeAddressOutput, crate::error::DescribeAddressError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_address_error(response)
                     } else {
                        crate::operation_deser::parse_describe_address_response(response)
                     }
                }
            }

/// Operation shape for `DescribeAddresses`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_addresses`](crate::client::Client::describe_addresses).
            ///
            /// See [`crate::client::fluent_builders::DescribeAddresses`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct DescribeAddresses {
    _private: ()
}
impl DescribeAddresses {
    /// Creates a new builder-style object to manufacture [`DescribeAddressesInput`](crate::input::DescribeAddressesInput).
    pub fn builder() -> crate::input::describe_addresses_input::Builder {
        crate::input::describe_addresses_input::Builder::default()
    }
    /// Creates a new `DescribeAddresses` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeAddresses {
                type Output = std::result::Result<crate::output::DescribeAddressesOutput, crate::error::DescribeAddressesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_addresses_error(response)
                     } else {
                        crate::operation_deser::parse_describe_addresses_response(response)
                     }
                }
            }

/// Operation shape for `DescribeCluster`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_cluster`](crate::client::Client::describe_cluster).
            ///
            /// See [`crate::client::fluent_builders::DescribeCluster`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct DescribeCluster {
    _private: ()
}
impl DescribeCluster {
    /// Creates a new builder-style object to manufacture [`DescribeClusterInput`](crate::input::DescribeClusterInput).
    pub fn builder() -> crate::input::describe_cluster_input::Builder {
        crate::input::describe_cluster_input::Builder::default()
    }
    /// Creates a new `DescribeCluster` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeCluster {
                type Output = std::result::Result<crate::output::DescribeClusterOutput, crate::error::DescribeClusterError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_cluster_error(response)
                     } else {
                        crate::operation_deser::parse_describe_cluster_response(response)
                     }
                }
            }

/// Operation shape for `DescribeJob`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_job`](crate::client::Client::describe_job).
            ///
            /// See [`crate::client::fluent_builders::DescribeJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct DescribeJob {
    _private: ()
}
impl DescribeJob {
    /// Creates a new builder-style object to manufacture [`DescribeJobInput`](crate::input::DescribeJobInput).
    pub fn builder() -> crate::input::describe_job_input::Builder {
        crate::input::describe_job_input::Builder::default()
    }
    /// Creates a new `DescribeJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeJob {
                type Output = std::result::Result<crate::output::DescribeJobOutput, crate::error::DescribeJobError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_job_error(response)
                     } else {
                        crate::operation_deser::parse_describe_job_response(response)
                     }
                }
            }

/// Operation shape for `DescribeReturnShippingLabel`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_return_shipping_label`](crate::client::Client::describe_return_shipping_label).
            ///
            /// See [`crate::client::fluent_builders::DescribeReturnShippingLabel`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct DescribeReturnShippingLabel {
    _private: ()
}
impl DescribeReturnShippingLabel {
    /// Creates a new builder-style object to manufacture [`DescribeReturnShippingLabelInput`](crate::input::DescribeReturnShippingLabelInput).
    pub fn builder() -> crate::input::describe_return_shipping_label_input::Builder {
        crate::input::describe_return_shipping_label_input::Builder::default()
    }
    /// Creates a new `DescribeReturnShippingLabel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeReturnShippingLabel {
                type Output = std::result::Result<crate::output::DescribeReturnShippingLabelOutput, crate::error::DescribeReturnShippingLabelError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_return_shipping_label_error(response)
                     } else {
                        crate::operation_deser::parse_describe_return_shipping_label_response(response)
                     }
                }
            }

/// Operation shape for `GetJobManifest`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_job_manifest`](crate::client::Client::get_job_manifest).
            ///
            /// See [`crate::client::fluent_builders::GetJobManifest`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct GetJobManifest {
    _private: ()
}
impl GetJobManifest {
    /// Creates a new builder-style object to manufacture [`GetJobManifestInput`](crate::input::GetJobManifestInput).
    pub fn builder() -> crate::input::get_job_manifest_input::Builder {
        crate::input::get_job_manifest_input::Builder::default()
    }
    /// Creates a new `GetJobManifest` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetJobManifest {
                type Output = std::result::Result<crate::output::GetJobManifestOutput, crate::error::GetJobManifestError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_job_manifest_error(response)
                     } else {
                        crate::operation_deser::parse_get_job_manifest_response(response)
                     }
                }
            }

/// Operation shape for `GetJobUnlockCode`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_job_unlock_code`](crate::client::Client::get_job_unlock_code).
            ///
            /// See [`crate::client::fluent_builders::GetJobUnlockCode`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct GetJobUnlockCode {
    _private: ()
}
impl GetJobUnlockCode {
    /// Creates a new builder-style object to manufacture [`GetJobUnlockCodeInput`](crate::input::GetJobUnlockCodeInput).
    pub fn builder() -> crate::input::get_job_unlock_code_input::Builder {
        crate::input::get_job_unlock_code_input::Builder::default()
    }
    /// Creates a new `GetJobUnlockCode` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetJobUnlockCode {
                type Output = std::result::Result<crate::output::GetJobUnlockCodeOutput, crate::error::GetJobUnlockCodeError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_job_unlock_code_error(response)
                     } else {
                        crate::operation_deser::parse_get_job_unlock_code_response(response)
                     }
                }
            }

/// Operation shape for `GetSnowballUsage`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_snowball_usage`](crate::client::Client::get_snowball_usage).
            ///
            /// See [`crate::client::fluent_builders::GetSnowballUsage`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct GetSnowballUsage {
    _private: ()
}
impl GetSnowballUsage {
    /// Creates a new builder-style object to manufacture [`GetSnowballUsageInput`](crate::input::GetSnowballUsageInput).
    pub fn builder() -> crate::input::get_snowball_usage_input::Builder {
        crate::input::get_snowball_usage_input::Builder::default()
    }
    /// Creates a new `GetSnowballUsage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetSnowballUsage {
                type Output = std::result::Result<crate::output::GetSnowballUsageOutput, crate::error::GetSnowballUsageError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_snowball_usage_error(response)
                     } else {
                        crate::operation_deser::parse_get_snowball_usage_response(response)
                     }
                }
            }

/// Operation shape for `GetSoftwareUpdates`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_software_updates`](crate::client::Client::get_software_updates).
            ///
            /// See [`crate::client::fluent_builders::GetSoftwareUpdates`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct GetSoftwareUpdates {
    _private: ()
}
impl GetSoftwareUpdates {
    /// Creates a new builder-style object to manufacture [`GetSoftwareUpdatesInput`](crate::input::GetSoftwareUpdatesInput).
    pub fn builder() -> crate::input::get_software_updates_input::Builder {
        crate::input::get_software_updates_input::Builder::default()
    }
    /// Creates a new `GetSoftwareUpdates` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetSoftwareUpdates {
                type Output = std::result::Result<crate::output::GetSoftwareUpdatesOutput, crate::error::GetSoftwareUpdatesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_software_updates_error(response)
                     } else {
                        crate::operation_deser::parse_get_software_updates_response(response)
                     }
                }
            }

/// Operation shape for `ListClusterJobs`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_cluster_jobs`](crate::client::Client::list_cluster_jobs).
            ///
            /// See [`crate::client::fluent_builders::ListClusterJobs`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct ListClusterJobs {
    _private: ()
}
impl ListClusterJobs {
    /// Creates a new builder-style object to manufacture [`ListClusterJobsInput`](crate::input::ListClusterJobsInput).
    pub fn builder() -> crate::input::list_cluster_jobs_input::Builder {
        crate::input::list_cluster_jobs_input::Builder::default()
    }
    /// Creates a new `ListClusterJobs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListClusterJobs {
                type Output = std::result::Result<crate::output::ListClusterJobsOutput, crate::error::ListClusterJobsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_cluster_jobs_error(response)
                     } else {
                        crate::operation_deser::parse_list_cluster_jobs_response(response)
                     }
                }
            }

/// Operation shape for `ListClusters`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_clusters`](crate::client::Client::list_clusters).
            ///
            /// See [`crate::client::fluent_builders::ListClusters`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct ListClusters {
    _private: ()
}
impl ListClusters {
    /// Creates a new builder-style object to manufacture [`ListClustersInput`](crate::input::ListClustersInput).
    pub fn builder() -> crate::input::list_clusters_input::Builder {
        crate::input::list_clusters_input::Builder::default()
    }
    /// Creates a new `ListClusters` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListClusters {
                type Output = std::result::Result<crate::output::ListClustersOutput, crate::error::ListClustersError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_clusters_error(response)
                     } else {
                        crate::operation_deser::parse_list_clusters_response(response)
                     }
                }
            }

/// Operation shape for `ListCompatibleImages`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_compatible_images`](crate::client::Client::list_compatible_images).
            ///
            /// See [`crate::client::fluent_builders::ListCompatibleImages`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct ListCompatibleImages {
    _private: ()
}
impl ListCompatibleImages {
    /// Creates a new builder-style object to manufacture [`ListCompatibleImagesInput`](crate::input::ListCompatibleImagesInput).
    pub fn builder() -> crate::input::list_compatible_images_input::Builder {
        crate::input::list_compatible_images_input::Builder::default()
    }
    /// Creates a new `ListCompatibleImages` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListCompatibleImages {
                type Output = std::result::Result<crate::output::ListCompatibleImagesOutput, crate::error::ListCompatibleImagesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_compatible_images_error(response)
                     } else {
                        crate::operation_deser::parse_list_compatible_images_response(response)
                     }
                }
            }

/// Operation shape for `ListJobs`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_jobs`](crate::client::Client::list_jobs).
            ///
            /// See [`crate::client::fluent_builders::ListJobs`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct ListJobs {
    _private: ()
}
impl ListJobs {
    /// Creates a new builder-style object to manufacture [`ListJobsInput`](crate::input::ListJobsInput).
    pub fn builder() -> crate::input::list_jobs_input::Builder {
        crate::input::list_jobs_input::Builder::default()
    }
    /// Creates a new `ListJobs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListJobs {
                type Output = std::result::Result<crate::output::ListJobsOutput, crate::error::ListJobsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_jobs_error(response)
                     } else {
                        crate::operation_deser::parse_list_jobs_response(response)
                     }
                }
            }

/// Operation shape for `ListLongTermPricing`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_long_term_pricing`](crate::client::Client::list_long_term_pricing).
            ///
            /// See [`crate::client::fluent_builders::ListLongTermPricing`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct ListLongTermPricing {
    _private: ()
}
impl ListLongTermPricing {
    /// Creates a new builder-style object to manufacture [`ListLongTermPricingInput`](crate::input::ListLongTermPricingInput).
    pub fn builder() -> crate::input::list_long_term_pricing_input::Builder {
        crate::input::list_long_term_pricing_input::Builder::default()
    }
    /// Creates a new `ListLongTermPricing` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListLongTermPricing {
                type Output = std::result::Result<crate::output::ListLongTermPricingOutput, crate::error::ListLongTermPricingError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_long_term_pricing_error(response)
                     } else {
                        crate::operation_deser::parse_list_long_term_pricing_response(response)
                     }
                }
            }

/// Operation shape for `UpdateCluster`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_cluster`](crate::client::Client::update_cluster).
            ///
            /// See [`crate::client::fluent_builders::UpdateCluster`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct UpdateCluster {
    _private: ()
}
impl UpdateCluster {
    /// Creates a new builder-style object to manufacture [`UpdateClusterInput`](crate::input::UpdateClusterInput).
    pub fn builder() -> crate::input::update_cluster_input::Builder {
        crate::input::update_cluster_input::Builder::default()
    }
    /// Creates a new `UpdateCluster` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateCluster {
                type Output = std::result::Result<crate::output::UpdateClusterOutput, crate::error::UpdateClusterError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_cluster_error(response)
                     } else {
                        crate::operation_deser::parse_update_cluster_response(response)
                     }
                }
            }

/// Operation shape for `UpdateJob`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_job`](crate::client::Client::update_job).
            ///
            /// See [`crate::client::fluent_builders::UpdateJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct UpdateJob {
    _private: ()
}
impl UpdateJob {
    /// Creates a new builder-style object to manufacture [`UpdateJobInput`](crate::input::UpdateJobInput).
    pub fn builder() -> crate::input::update_job_input::Builder {
        crate::input::update_job_input::Builder::default()
    }
    /// Creates a new `UpdateJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateJob {
                type Output = std::result::Result<crate::output::UpdateJobOutput, crate::error::UpdateJobError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_job_error(response)
                     } else {
                        crate::operation_deser::parse_update_job_response(response)
                     }
                }
            }

/// Operation shape for `UpdateJobShipmentState`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_job_shipment_state`](crate::client::Client::update_job_shipment_state).
            ///
            /// See [`crate::client::fluent_builders::UpdateJobShipmentState`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct UpdateJobShipmentState {
    _private: ()
}
impl UpdateJobShipmentState {
    /// Creates a new builder-style object to manufacture [`UpdateJobShipmentStateInput`](crate::input::UpdateJobShipmentStateInput).
    pub fn builder() -> crate::input::update_job_shipment_state_input::Builder {
        crate::input::update_job_shipment_state_input::Builder::default()
    }
    /// Creates a new `UpdateJobShipmentState` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateJobShipmentState {
                type Output = std::result::Result<crate::output::UpdateJobShipmentStateOutput, crate::error::UpdateJobShipmentStateError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_job_shipment_state_error(response)
                     } else {
                        crate::operation_deser::parse_update_job_shipment_state_response(response)
                     }
                }
            }

/// Operation shape for `UpdateLongTermPricing`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_long_term_pricing`](crate::client::Client::update_long_term_pricing).
            ///
            /// See [`crate::client::fluent_builders::UpdateLongTermPricing`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct UpdateLongTermPricing {
    _private: ()
}
impl UpdateLongTermPricing {
    /// Creates a new builder-style object to manufacture [`UpdateLongTermPricingInput`](crate::input::UpdateLongTermPricingInput).
    pub fn builder() -> crate::input::update_long_term_pricing_input::Builder {
        crate::input::update_long_term_pricing_input::Builder::default()
    }
    /// Creates a new `UpdateLongTermPricing` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateLongTermPricing {
                type Output = std::result::Result<crate::output::UpdateLongTermPricingOutput, crate::error::UpdateLongTermPricingError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_long_term_pricing_error(response)
                     } else {
                        crate::operation_deser::parse_update_long_term_pricing_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

