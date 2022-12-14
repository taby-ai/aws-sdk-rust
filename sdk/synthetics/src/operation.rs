// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AssociateResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`associate_resource`](crate::client::Client::associate_resource).
            ///
            /// See [`crate::client::fluent_builders::AssociateResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct AssociateResource {
    _private: ()
}
impl AssociateResource {
    /// Creates a new builder-style object to manufacture [`AssociateResourceInput`](crate::input::AssociateResourceInput).
    pub fn builder() -> crate::input::associate_resource_input::Builder {
        crate::input::associate_resource_input::Builder::default()
    }
    /// Creates a new `AssociateResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AssociateResource {
                type Output = std::result::Result<crate::output::AssociateResourceOutput, crate::error::AssociateResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_associate_resource_error(response)
                     } else {
                        crate::operation_deser::parse_associate_resource_response(response)
                     }
                }
            }

/// Operation shape for `CreateCanary`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_canary`](crate::client::Client::create_canary).
            ///
            /// See [`crate::client::fluent_builders::CreateCanary`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct CreateCanary {
    _private: ()
}
impl CreateCanary {
    /// Creates a new builder-style object to manufacture [`CreateCanaryInput`](crate::input::CreateCanaryInput).
    pub fn builder() -> crate::input::create_canary_input::Builder {
        crate::input::create_canary_input::Builder::default()
    }
    /// Creates a new `CreateCanary` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateCanary {
                type Output = std::result::Result<crate::output::CreateCanaryOutput, crate::error::CreateCanaryError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_canary_error(response)
                     } else {
                        crate::operation_deser::parse_create_canary_response(response)
                     }
                }
            }

/// Operation shape for `CreateGroup`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_group`](crate::client::Client::create_group).
            ///
            /// See [`crate::client::fluent_builders::CreateGroup`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct CreateGroup {
    _private: ()
}
impl CreateGroup {
    /// Creates a new builder-style object to manufacture [`CreateGroupInput`](crate::input::CreateGroupInput).
    pub fn builder() -> crate::input::create_group_input::Builder {
        crate::input::create_group_input::Builder::default()
    }
    /// Creates a new `CreateGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateGroup {
                type Output = std::result::Result<crate::output::CreateGroupOutput, crate::error::CreateGroupError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_group_error(response)
                     } else {
                        crate::operation_deser::parse_create_group_response(response)
                     }
                }
            }

/// Operation shape for `DeleteCanary`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_canary`](crate::client::Client::delete_canary).
            ///
            /// See [`crate::client::fluent_builders::DeleteCanary`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct DeleteCanary {
    _private: ()
}
impl DeleteCanary {
    /// Creates a new builder-style object to manufacture [`DeleteCanaryInput`](crate::input::DeleteCanaryInput).
    pub fn builder() -> crate::input::delete_canary_input::Builder {
        crate::input::delete_canary_input::Builder::default()
    }
    /// Creates a new `DeleteCanary` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteCanary {
                type Output = std::result::Result<crate::output::DeleteCanaryOutput, crate::error::DeleteCanaryError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_canary_error(response)
                     } else {
                        crate::operation_deser::parse_delete_canary_response(response)
                     }
                }
            }

/// Operation shape for `DeleteGroup`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_group`](crate::client::Client::delete_group).
            ///
            /// See [`crate::client::fluent_builders::DeleteGroup`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct DeleteGroup {
    _private: ()
}
impl DeleteGroup {
    /// Creates a new builder-style object to manufacture [`DeleteGroupInput`](crate::input::DeleteGroupInput).
    pub fn builder() -> crate::input::delete_group_input::Builder {
        crate::input::delete_group_input::Builder::default()
    }
    /// Creates a new `DeleteGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteGroup {
                type Output = std::result::Result<crate::output::DeleteGroupOutput, crate::error::DeleteGroupError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_group_error(response)
                     } else {
                        crate::operation_deser::parse_delete_group_response(response)
                     }
                }
            }

/// Operation shape for `DescribeCanaries`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_canaries`](crate::client::Client::describe_canaries).
            ///
            /// See [`crate::client::fluent_builders::DescribeCanaries`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct DescribeCanaries {
    _private: ()
}
impl DescribeCanaries {
    /// Creates a new builder-style object to manufacture [`DescribeCanariesInput`](crate::input::DescribeCanariesInput).
    pub fn builder() -> crate::input::describe_canaries_input::Builder {
        crate::input::describe_canaries_input::Builder::default()
    }
    /// Creates a new `DescribeCanaries` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeCanaries {
                type Output = std::result::Result<crate::output::DescribeCanariesOutput, crate::error::DescribeCanariesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_canaries_error(response)
                     } else {
                        crate::operation_deser::parse_describe_canaries_response(response)
                     }
                }
            }

/// Operation shape for `DescribeCanariesLastRun`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_canaries_last_run`](crate::client::Client::describe_canaries_last_run).
            ///
            /// See [`crate::client::fluent_builders::DescribeCanariesLastRun`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct DescribeCanariesLastRun {
    _private: ()
}
impl DescribeCanariesLastRun {
    /// Creates a new builder-style object to manufacture [`DescribeCanariesLastRunInput`](crate::input::DescribeCanariesLastRunInput).
    pub fn builder() -> crate::input::describe_canaries_last_run_input::Builder {
        crate::input::describe_canaries_last_run_input::Builder::default()
    }
    /// Creates a new `DescribeCanariesLastRun` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeCanariesLastRun {
                type Output = std::result::Result<crate::output::DescribeCanariesLastRunOutput, crate::error::DescribeCanariesLastRunError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_canaries_last_run_error(response)
                     } else {
                        crate::operation_deser::parse_describe_canaries_last_run_response(response)
                     }
                }
            }

/// Operation shape for `DescribeRuntimeVersions`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_runtime_versions`](crate::client::Client::describe_runtime_versions).
            ///
            /// See [`crate::client::fluent_builders::DescribeRuntimeVersions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct DescribeRuntimeVersions {
    _private: ()
}
impl DescribeRuntimeVersions {
    /// Creates a new builder-style object to manufacture [`DescribeRuntimeVersionsInput`](crate::input::DescribeRuntimeVersionsInput).
    pub fn builder() -> crate::input::describe_runtime_versions_input::Builder {
        crate::input::describe_runtime_versions_input::Builder::default()
    }
    /// Creates a new `DescribeRuntimeVersions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeRuntimeVersions {
                type Output = std::result::Result<crate::output::DescribeRuntimeVersionsOutput, crate::error::DescribeRuntimeVersionsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_runtime_versions_error(response)
                     } else {
                        crate::operation_deser::parse_describe_runtime_versions_response(response)
                     }
                }
            }

/// Operation shape for `DisassociateResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`disassociate_resource`](crate::client::Client::disassociate_resource).
            ///
            /// See [`crate::client::fluent_builders::DisassociateResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct DisassociateResource {
    _private: ()
}
impl DisassociateResource {
    /// Creates a new builder-style object to manufacture [`DisassociateResourceInput`](crate::input::DisassociateResourceInput).
    pub fn builder() -> crate::input::disassociate_resource_input::Builder {
        crate::input::disassociate_resource_input::Builder::default()
    }
    /// Creates a new `DisassociateResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DisassociateResource {
                type Output = std::result::Result<crate::output::DisassociateResourceOutput, crate::error::DisassociateResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_disassociate_resource_error(response)
                     } else {
                        crate::operation_deser::parse_disassociate_resource_response(response)
                     }
                }
            }

/// Operation shape for `GetCanary`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_canary`](crate::client::Client::get_canary).
            ///
            /// See [`crate::client::fluent_builders::GetCanary`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct GetCanary {
    _private: ()
}
impl GetCanary {
    /// Creates a new builder-style object to manufacture [`GetCanaryInput`](crate::input::GetCanaryInput).
    pub fn builder() -> crate::input::get_canary_input::Builder {
        crate::input::get_canary_input::Builder::default()
    }
    /// Creates a new `GetCanary` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetCanary {
                type Output = std::result::Result<crate::output::GetCanaryOutput, crate::error::GetCanaryError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_canary_error(response)
                     } else {
                        crate::operation_deser::parse_get_canary_response(response)
                     }
                }
            }

/// Operation shape for `GetCanaryRuns`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_canary_runs`](crate::client::Client::get_canary_runs).
            ///
            /// See [`crate::client::fluent_builders::GetCanaryRuns`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct GetCanaryRuns {
    _private: ()
}
impl GetCanaryRuns {
    /// Creates a new builder-style object to manufacture [`GetCanaryRunsInput`](crate::input::GetCanaryRunsInput).
    pub fn builder() -> crate::input::get_canary_runs_input::Builder {
        crate::input::get_canary_runs_input::Builder::default()
    }
    /// Creates a new `GetCanaryRuns` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetCanaryRuns {
                type Output = std::result::Result<crate::output::GetCanaryRunsOutput, crate::error::GetCanaryRunsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_canary_runs_error(response)
                     } else {
                        crate::operation_deser::parse_get_canary_runs_response(response)
                     }
                }
            }

/// Operation shape for `GetGroup`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_group`](crate::client::Client::get_group).
            ///
            /// See [`crate::client::fluent_builders::GetGroup`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct GetGroup {
    _private: ()
}
impl GetGroup {
    /// Creates a new builder-style object to manufacture [`GetGroupInput`](crate::input::GetGroupInput).
    pub fn builder() -> crate::input::get_group_input::Builder {
        crate::input::get_group_input::Builder::default()
    }
    /// Creates a new `GetGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetGroup {
                type Output = std::result::Result<crate::output::GetGroupOutput, crate::error::GetGroupError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_group_error(response)
                     } else {
                        crate::operation_deser::parse_get_group_response(response)
                     }
                }
            }

/// Operation shape for `ListAssociatedGroups`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_associated_groups`](crate::client::Client::list_associated_groups).
            ///
            /// See [`crate::client::fluent_builders::ListAssociatedGroups`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct ListAssociatedGroups {
    _private: ()
}
impl ListAssociatedGroups {
    /// Creates a new builder-style object to manufacture [`ListAssociatedGroupsInput`](crate::input::ListAssociatedGroupsInput).
    pub fn builder() -> crate::input::list_associated_groups_input::Builder {
        crate::input::list_associated_groups_input::Builder::default()
    }
    /// Creates a new `ListAssociatedGroups` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAssociatedGroups {
                type Output = std::result::Result<crate::output::ListAssociatedGroupsOutput, crate::error::ListAssociatedGroupsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_associated_groups_error(response)
                     } else {
                        crate::operation_deser::parse_list_associated_groups_response(response)
                     }
                }
            }

/// Operation shape for `ListGroupResources`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_group_resources`](crate::client::Client::list_group_resources).
            ///
            /// See [`crate::client::fluent_builders::ListGroupResources`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct ListGroupResources {
    _private: ()
}
impl ListGroupResources {
    /// Creates a new builder-style object to manufacture [`ListGroupResourcesInput`](crate::input::ListGroupResourcesInput).
    pub fn builder() -> crate::input::list_group_resources_input::Builder {
        crate::input::list_group_resources_input::Builder::default()
    }
    /// Creates a new `ListGroupResources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListGroupResources {
                type Output = std::result::Result<crate::output::ListGroupResourcesOutput, crate::error::ListGroupResourcesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_group_resources_error(response)
                     } else {
                        crate::operation_deser::parse_list_group_resources_response(response)
                     }
                }
            }

/// Operation shape for `ListGroups`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_groups`](crate::client::Client::list_groups).
            ///
            /// See [`crate::client::fluent_builders::ListGroups`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct ListGroups {
    _private: ()
}
impl ListGroups {
    /// Creates a new builder-style object to manufacture [`ListGroupsInput`](crate::input::ListGroupsInput).
    pub fn builder() -> crate::input::list_groups_input::Builder {
        crate::input::list_groups_input::Builder::default()
    }
    /// Creates a new `ListGroups` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListGroups {
                type Output = std::result::Result<crate::output::ListGroupsOutput, crate::error::ListGroupsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_groups_error(response)
                     } else {
                        crate::operation_deser::parse_list_groups_response(response)
                     }
                }
            }

/// Operation shape for `ListTagsForResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
            ///
            /// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct ListTagsForResource {
    _private: ()
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
                type Output = std::result::Result<crate::output::ListTagsForResourceOutput, crate::error::ListTagsForResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_tags_for_resource_error(response)
                     } else {
                        crate::operation_deser::parse_list_tags_for_resource_response(response)
                     }
                }
            }

/// Operation shape for `StartCanary`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`start_canary`](crate::client::Client::start_canary).
            ///
            /// See [`crate::client::fluent_builders::StartCanary`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct StartCanary {
    _private: ()
}
impl StartCanary {
    /// Creates a new builder-style object to manufacture [`StartCanaryInput`](crate::input::StartCanaryInput).
    pub fn builder() -> crate::input::start_canary_input::Builder {
        crate::input::start_canary_input::Builder::default()
    }
    /// Creates a new `StartCanary` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartCanary {
                type Output = std::result::Result<crate::output::StartCanaryOutput, crate::error::StartCanaryError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_start_canary_error(response)
                     } else {
                        crate::operation_deser::parse_start_canary_response(response)
                     }
                }
            }

/// Operation shape for `StopCanary`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`stop_canary`](crate::client::Client::stop_canary).
            ///
            /// See [`crate::client::fluent_builders::StopCanary`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct StopCanary {
    _private: ()
}
impl StopCanary {
    /// Creates a new builder-style object to manufacture [`StopCanaryInput`](crate::input::StopCanaryInput).
    pub fn builder() -> crate::input::stop_canary_input::Builder {
        crate::input::stop_canary_input::Builder::default()
    }
    /// Creates a new `StopCanary` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopCanary {
                type Output = std::result::Result<crate::output::StopCanaryOutput, crate::error::StopCanaryError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_stop_canary_error(response)
                     } else {
                        crate::operation_deser::parse_stop_canary_response(response)
                     }
                }
            }

/// Operation shape for `TagResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`tag_resource`](crate::client::Client::tag_resource).
            ///
            /// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct TagResource {
    _private: ()
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
                type Output = std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_tag_resource_error(response)
                     } else {
                        crate::operation_deser::parse_tag_resource_response(response)
                     }
                }
            }

/// Operation shape for `UntagResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`untag_resource`](crate::client::Client::untag_resource).
            ///
            /// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct UntagResource {
    _private: ()
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
                type Output = std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_untag_resource_error(response)
                     } else {
                        crate::operation_deser::parse_untag_resource_response(response)
                     }
                }
            }

/// Operation shape for `UpdateCanary`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_canary`](crate::client::Client::update_canary).
            ///
            /// See [`crate::client::fluent_builders::UpdateCanary`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct UpdateCanary {
    _private: ()
}
impl UpdateCanary {
    /// Creates a new builder-style object to manufacture [`UpdateCanaryInput`](crate::input::UpdateCanaryInput).
    pub fn builder() -> crate::input::update_canary_input::Builder {
        crate::input::update_canary_input::Builder::default()
    }
    /// Creates a new `UpdateCanary` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateCanary {
                type Output = std::result::Result<crate::output::UpdateCanaryOutput, crate::error::UpdateCanaryError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_canary_error(response)
                     } else {
                        crate::operation_deser::parse_update_canary_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

