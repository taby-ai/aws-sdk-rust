// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `StartMedicalStreamTranscription`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`start_medical_stream_transcription`](crate::client::Client::start_medical_stream_transcription).
            ///
            /// See [`crate::client::fluent_builders::StartMedicalStreamTranscription`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct StartMedicalStreamTranscription {
    _private: ()
}
impl StartMedicalStreamTranscription {
    /// Creates a new builder-style object to manufacture [`StartMedicalStreamTranscriptionInput`](crate::input::StartMedicalStreamTranscriptionInput).
    pub fn builder() -> crate::input::start_medical_stream_transcription_input::Builder {
        crate::input::start_medical_stream_transcription_input::Builder::default()
    }
    /// Creates a new `StartMedicalStreamTranscription` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseHttpResponse for StartMedicalStreamTranscription {
                type Output = std::result::Result<crate::output::StartMedicalStreamTranscriptionOutput, crate::error::StartMedicalStreamTranscriptionError>;
                fn parse_unloaded(&self, response: &mut aws_smithy_http::operation::Response) -> Option<Self::Output> {
                    // This is an error, defer to the non-streaming parser
                    if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
                        return None;
                    }
                    Some(crate::operation_deser::parse_start_medical_stream_transcription(response))
                }
                fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                    // if streaming, we only hit this case if its an error
                    crate::operation_deser::parse_start_medical_stream_transcription_error(response)
                }
            }

/// Operation shape for `StartStreamTranscription`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`start_stream_transcription`](crate::client::Client::start_stream_transcription).
            ///
            /// See [`crate::client::fluent_builders::StartStreamTranscription`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct StartStreamTranscription {
    _private: ()
}
impl StartStreamTranscription {
    /// Creates a new builder-style object to manufacture [`StartStreamTranscriptionInput`](crate::input::StartStreamTranscriptionInput).
    pub fn builder() -> crate::input::start_stream_transcription_input::Builder {
        crate::input::start_stream_transcription_input::Builder::default()
    }
    /// Creates a new `StartStreamTranscription` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseHttpResponse for StartStreamTranscription {
                type Output = std::result::Result<crate::output::StartStreamTranscriptionOutput, crate::error::StartStreamTranscriptionError>;
                fn parse_unloaded(&self, response: &mut aws_smithy_http::operation::Response) -> Option<Self::Output> {
                    // This is an error, defer to the non-streaming parser
                    if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
                        return None;
                    }
                    Some(crate::operation_deser::parse_start_stream_transcription(response))
                }
                fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                    // if streaming, we only hit this case if its an error
                    crate::operation_deser::parse_start_stream_transcription_error(response)
                }
            }

/// Operation customization and supporting types
pub mod customize;

