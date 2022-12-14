// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateDeliveryStream`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_delivery_stream`](crate::client::Client::create_delivery_stream).
            ///
            /// See [`crate::client::fluent_builders::CreateDeliveryStream`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct CreateDeliveryStream {
    _private: ()
}
impl CreateDeliveryStream {
    /// Creates a new builder-style object to manufacture [`CreateDeliveryStreamInput`](crate::input::CreateDeliveryStreamInput).
    pub fn builder() -> crate::input::create_delivery_stream_input::Builder {
        crate::input::create_delivery_stream_input::Builder::default()
    }
    /// Creates a new `CreateDeliveryStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateDeliveryStream {
                type Output = std::result::Result<crate::output::CreateDeliveryStreamOutput, crate::error::CreateDeliveryStreamError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_delivery_stream_error(response)
                     } else {
                        crate::operation_deser::parse_create_delivery_stream_response(response)
                     }
                }
            }

/// Operation shape for `DeleteDeliveryStream`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_delivery_stream`](crate::client::Client::delete_delivery_stream).
            ///
            /// See [`crate::client::fluent_builders::DeleteDeliveryStream`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct DeleteDeliveryStream {
    _private: ()
}
impl DeleteDeliveryStream {
    /// Creates a new builder-style object to manufacture [`DeleteDeliveryStreamInput`](crate::input::DeleteDeliveryStreamInput).
    pub fn builder() -> crate::input::delete_delivery_stream_input::Builder {
        crate::input::delete_delivery_stream_input::Builder::default()
    }
    /// Creates a new `DeleteDeliveryStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteDeliveryStream {
                type Output = std::result::Result<crate::output::DeleteDeliveryStreamOutput, crate::error::DeleteDeliveryStreamError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_delivery_stream_error(response)
                     } else {
                        crate::operation_deser::parse_delete_delivery_stream_response(response)
                     }
                }
            }

/// Operation shape for `DescribeDeliveryStream`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_delivery_stream`](crate::client::Client::describe_delivery_stream).
            ///
            /// See [`crate::client::fluent_builders::DescribeDeliveryStream`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct DescribeDeliveryStream {
    _private: ()
}
impl DescribeDeliveryStream {
    /// Creates a new builder-style object to manufacture [`DescribeDeliveryStreamInput`](crate::input::DescribeDeliveryStreamInput).
    pub fn builder() -> crate::input::describe_delivery_stream_input::Builder {
        crate::input::describe_delivery_stream_input::Builder::default()
    }
    /// Creates a new `DescribeDeliveryStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeDeliveryStream {
                type Output = std::result::Result<crate::output::DescribeDeliveryStreamOutput, crate::error::DescribeDeliveryStreamError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_delivery_stream_error(response)
                     } else {
                        crate::operation_deser::parse_describe_delivery_stream_response(response)
                     }
                }
            }

/// Operation shape for `ListDeliveryStreams`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_delivery_streams`](crate::client::Client::list_delivery_streams).
            ///
            /// See [`crate::client::fluent_builders::ListDeliveryStreams`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct ListDeliveryStreams {
    _private: ()
}
impl ListDeliveryStreams {
    /// Creates a new builder-style object to manufacture [`ListDeliveryStreamsInput`](crate::input::ListDeliveryStreamsInput).
    pub fn builder() -> crate::input::list_delivery_streams_input::Builder {
        crate::input::list_delivery_streams_input::Builder::default()
    }
    /// Creates a new `ListDeliveryStreams` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListDeliveryStreams {
                type Output = std::result::Result<crate::output::ListDeliveryStreamsOutput, crate::error::ListDeliveryStreamsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_delivery_streams_error(response)
                     } else {
                        crate::operation_deser::parse_list_delivery_streams_response(response)
                     }
                }
            }

/// Operation shape for `ListTagsForDeliveryStream`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_tags_for_delivery_stream`](crate::client::Client::list_tags_for_delivery_stream).
            ///
            /// See [`crate::client::fluent_builders::ListTagsForDeliveryStream`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct ListTagsForDeliveryStream {
    _private: ()
}
impl ListTagsForDeliveryStream {
    /// Creates a new builder-style object to manufacture [`ListTagsForDeliveryStreamInput`](crate::input::ListTagsForDeliveryStreamInput).
    pub fn builder() -> crate::input::list_tags_for_delivery_stream_input::Builder {
        crate::input::list_tags_for_delivery_stream_input::Builder::default()
    }
    /// Creates a new `ListTagsForDeliveryStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForDeliveryStream {
                type Output = std::result::Result<crate::output::ListTagsForDeliveryStreamOutput, crate::error::ListTagsForDeliveryStreamError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_tags_for_delivery_stream_error(response)
                     } else {
                        crate::operation_deser::parse_list_tags_for_delivery_stream_response(response)
                     }
                }
            }

/// Operation shape for `PutRecord`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_record`](crate::client::Client::put_record).
            ///
            /// See [`crate::client::fluent_builders::PutRecord`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct PutRecord {
    _private: ()
}
impl PutRecord {
    /// Creates a new builder-style object to manufacture [`PutRecordInput`](crate::input::PutRecordInput).
    pub fn builder() -> crate::input::put_record_input::Builder {
        crate::input::put_record_input::Builder::default()
    }
    /// Creates a new `PutRecord` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutRecord {
                type Output = std::result::Result<crate::output::PutRecordOutput, crate::error::PutRecordError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_record_error(response)
                     } else {
                        crate::operation_deser::parse_put_record_response(response)
                     }
                }
            }

/// Operation shape for `PutRecordBatch`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_record_batch`](crate::client::Client::put_record_batch).
            ///
            /// See [`crate::client::fluent_builders::PutRecordBatch`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct PutRecordBatch {
    _private: ()
}
impl PutRecordBatch {
    /// Creates a new builder-style object to manufacture [`PutRecordBatchInput`](crate::input::PutRecordBatchInput).
    pub fn builder() -> crate::input::put_record_batch_input::Builder {
        crate::input::put_record_batch_input::Builder::default()
    }
    /// Creates a new `PutRecordBatch` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutRecordBatch {
                type Output = std::result::Result<crate::output::PutRecordBatchOutput, crate::error::PutRecordBatchError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_record_batch_error(response)
                     } else {
                        crate::operation_deser::parse_put_record_batch_response(response)
                     }
                }
            }

/// Operation shape for `StartDeliveryStreamEncryption`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`start_delivery_stream_encryption`](crate::client::Client::start_delivery_stream_encryption).
            ///
            /// See [`crate::client::fluent_builders::StartDeliveryStreamEncryption`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct StartDeliveryStreamEncryption {
    _private: ()
}
impl StartDeliveryStreamEncryption {
    /// Creates a new builder-style object to manufacture [`StartDeliveryStreamEncryptionInput`](crate::input::StartDeliveryStreamEncryptionInput).
    pub fn builder() -> crate::input::start_delivery_stream_encryption_input::Builder {
        crate::input::start_delivery_stream_encryption_input::Builder::default()
    }
    /// Creates a new `StartDeliveryStreamEncryption` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartDeliveryStreamEncryption {
                type Output = std::result::Result<crate::output::StartDeliveryStreamEncryptionOutput, crate::error::StartDeliveryStreamEncryptionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_start_delivery_stream_encryption_error(response)
                     } else {
                        crate::operation_deser::parse_start_delivery_stream_encryption_response(response)
                     }
                }
            }

/// Operation shape for `StopDeliveryStreamEncryption`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`stop_delivery_stream_encryption`](crate::client::Client::stop_delivery_stream_encryption).
            ///
            /// See [`crate::client::fluent_builders::StopDeliveryStreamEncryption`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct StopDeliveryStreamEncryption {
    _private: ()
}
impl StopDeliveryStreamEncryption {
    /// Creates a new builder-style object to manufacture [`StopDeliveryStreamEncryptionInput`](crate::input::StopDeliveryStreamEncryptionInput).
    pub fn builder() -> crate::input::stop_delivery_stream_encryption_input::Builder {
        crate::input::stop_delivery_stream_encryption_input::Builder::default()
    }
    /// Creates a new `StopDeliveryStreamEncryption` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopDeliveryStreamEncryption {
                type Output = std::result::Result<crate::output::StopDeliveryStreamEncryptionOutput, crate::error::StopDeliveryStreamEncryptionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_stop_delivery_stream_encryption_error(response)
                     } else {
                        crate::operation_deser::parse_stop_delivery_stream_encryption_response(response)
                     }
                }
            }

/// Operation shape for `TagDeliveryStream`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`tag_delivery_stream`](crate::client::Client::tag_delivery_stream).
            ///
            /// See [`crate::client::fluent_builders::TagDeliveryStream`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct TagDeliveryStream {
    _private: ()
}
impl TagDeliveryStream {
    /// Creates a new builder-style object to manufacture [`TagDeliveryStreamInput`](crate::input::TagDeliveryStreamInput).
    pub fn builder() -> crate::input::tag_delivery_stream_input::Builder {
        crate::input::tag_delivery_stream_input::Builder::default()
    }
    /// Creates a new `TagDeliveryStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagDeliveryStream {
                type Output = std::result::Result<crate::output::TagDeliveryStreamOutput, crate::error::TagDeliveryStreamError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_tag_delivery_stream_error(response)
                     } else {
                        crate::operation_deser::parse_tag_delivery_stream_response(response)
                     }
                }
            }

/// Operation shape for `UntagDeliveryStream`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`untag_delivery_stream`](crate::client::Client::untag_delivery_stream).
            ///
            /// See [`crate::client::fluent_builders::UntagDeliveryStream`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct UntagDeliveryStream {
    _private: ()
}
impl UntagDeliveryStream {
    /// Creates a new builder-style object to manufacture [`UntagDeliveryStreamInput`](crate::input::UntagDeliveryStreamInput).
    pub fn builder() -> crate::input::untag_delivery_stream_input::Builder {
        crate::input::untag_delivery_stream_input::Builder::default()
    }
    /// Creates a new `UntagDeliveryStream` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagDeliveryStream {
                type Output = std::result::Result<crate::output::UntagDeliveryStreamOutput, crate::error::UntagDeliveryStreamError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_untag_delivery_stream_error(response)
                     } else {
                        crate::operation_deser::parse_untag_delivery_stream_response(response)
                     }
                }
            }

/// Operation shape for `UpdateDestination`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_destination`](crate::client::Client::update_destination).
            ///
            /// See [`crate::client::fluent_builders::UpdateDestination`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct UpdateDestination {
    _private: ()
}
impl UpdateDestination {
    /// Creates a new builder-style object to manufacture [`UpdateDestinationInput`](crate::input::UpdateDestinationInput).
    pub fn builder() -> crate::input::update_destination_input::Builder {
        crate::input::update_destination_input::Builder::default()
    }
    /// Creates a new `UpdateDestination` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateDestination {
                type Output = std::result::Result<crate::output::UpdateDestinationOutput, crate::error::UpdateDestinationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_destination_error(response)
                     } else {
                        crate::operation_deser::parse_update_destination_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

