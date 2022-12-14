// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(Debug)]
pub struct AudioStreamErrorMarshaller;

impl AudioStreamErrorMarshaller {
    pub fn new() -> Self {
        AudioStreamErrorMarshaller
    }
}
impl aws_smithy_eventstream::frame::MarshallMessage for AudioStreamErrorMarshaller {
    type Input = crate::error::AudioStreamError;
    fn marshall(
        &self,
        _input: Self::Input,
    ) -> std::result::Result<
        aws_smithy_eventstream::frame::Message,
        aws_smithy_eventstream::error::Error,
    > {
        let mut headers = Vec::new();
        headers.push(aws_smithy_eventstream::frame::Header::new(
            ":message-type",
            aws_smithy_eventstream::frame::HeaderValue::String("exception".into()),
        ));
        let payload = Vec::new();
        Ok(aws_smithy_eventstream::frame::Message::new_from_parts(
            headers, payload,
        ))
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct AudioStreamMarshaller;

impl AudioStreamMarshaller {
    pub fn new() -> Self {
        AudioStreamMarshaller
    }
}
impl aws_smithy_eventstream::frame::MarshallMessage for AudioStreamMarshaller {
    type Input = crate::model::AudioStream;
    fn marshall(
        &self,
        input: Self::Input,
    ) -> std::result::Result<
        aws_smithy_eventstream::frame::Message,
        aws_smithy_eventstream::error::Error,
    > {
        let mut headers = Vec::new();
        headers.push(aws_smithy_eventstream::frame::Header::new(
            ":message-type",
            aws_smithy_eventstream::frame::HeaderValue::String("event".into()),
        ));
        let payload = match input {
            Self::Input::AudioEvent(inner) =>  {
                headers.push(aws_smithy_eventstream::frame::Header::new(":event-type", aws_smithy_eventstream::frame::HeaderValue::String("AudioEvent".into())));
                headers.push(aws_smithy_eventstream::frame::Header::new(":content-type", aws_smithy_eventstream::frame::HeaderValue::String("application/octet-stream".into())));
                if let Some(inner_payload) = inner.audio_chunk {
                    inner_payload.into_inner()
                }
                 else  {
                    Vec::new()
                }
            }
            Self::Input::Unknown => return Err(
                                            aws_smithy_eventstream::error::Error::marshalling("Cannot serialize `AudioStream::Unknown` for the request. The `Unknown` variant is intended for responses only. It occurs when an outdated client is used after a new enum variant was added on the server side.".to_owned())
                                        )
        }
        ;
        Ok(aws_smithy_eventstream::frame::Message::new_from_parts(
            headers, payload,
        ))
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct MedicalTranscriptResultStreamUnmarshaller;

impl MedicalTranscriptResultStreamUnmarshaller {
    pub fn new() -> Self {
        MedicalTranscriptResultStreamUnmarshaller
    }
}
impl aws_smithy_eventstream::frame::UnmarshallMessage
    for MedicalTranscriptResultStreamUnmarshaller
{
    type Output = crate::model::MedicalTranscriptResultStream;
    type Error = crate::error::MedicalTranscriptResultStreamError;
    fn unmarshall(
        &self,
        message: &aws_smithy_eventstream::frame::Message,
    ) -> std::result::Result<
        aws_smithy_eventstream::frame::UnmarshalledMessage<Self::Output, Self::Error>,
        aws_smithy_eventstream::error::Error,
    > {
        let response_headers = aws_smithy_eventstream::smithy::parse_response_headers(message)?;
        match response_headers.message_type.as_str() {
            "event" => match response_headers.smithy_type.as_str() {
                "TranscriptEvent" => {
                    let parsed =
                            crate::json_deser::deser_structure_crate_model_medical_transcript_event_payload(&message.payload()[..])
                                            .map_err(|err| {
                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall TranscriptEvent: {}", err))
                                            })?
                        ;
                    Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                        crate::model::MedicalTranscriptResultStream::TranscriptEvent(parsed),
                    ))
                }
                _unknown_variant => Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                    crate::model::MedicalTranscriptResultStream::Unknown,
                )),
            },
            "exception" => {
                let generic =
                    match crate::json_deser::parse_event_stream_generic_error(message.payload()) {
                        Ok(generic) => generic,
                        Err(err) => {
                            return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                crate::error::MedicalTranscriptResultStreamError::unhandled(err),
                            ))
                        }
                    };
                match response_headers.smithy_type.as_str() {
                    "BadRequestException" => {
                        let mut builder = crate::error::BadRequestException::builder();
                        builder = crate::json_deser::deser_structure_crate_error_bad_request_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall BadRequestException: {}", err))
                                                            })?;
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::error::MedicalTranscriptResultStreamError::new(
                                                                crate::error::MedicalTranscriptResultStreamErrorKind::BadRequestException(builder.build()),
                                                                generic,
                                                            )
                                                        ));
                    }
                    "LimitExceededException" => {
                        let mut builder = crate::error::LimitExceededException::builder();
                        builder = crate::json_deser::deser_structure_crate_error_limit_exceeded_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall LimitExceededException: {}", err))
                                                            })?;
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::error::MedicalTranscriptResultStreamError::new(
                                                                crate::error::MedicalTranscriptResultStreamErrorKind::LimitExceededException(builder.build()),
                                                                generic,
                                                            )
                                                        ));
                    }
                    "InternalFailureException" => {
                        let mut builder = crate::error::InternalFailureException::builder();
                        builder = crate::json_deser::deser_structure_crate_error_internal_failure_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall InternalFailureException: {}", err))
                                                            })?;
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::error::MedicalTranscriptResultStreamError::new(
                                                                crate::error::MedicalTranscriptResultStreamErrorKind::InternalFailureException(builder.build()),
                                                                generic,
                                                            )
                                                        ));
                    }
                    "ConflictException" => {
                        let mut builder = crate::error::ConflictException::builder();
                        builder = crate::json_deser::deser_structure_crate_error_conflict_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall ConflictException: {}", err))
                                                            })?;
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::error::MedicalTranscriptResultStreamError::new(
                                                                crate::error::MedicalTranscriptResultStreamErrorKind::ConflictException(builder.build()),
                                                                generic,
                                                            )
                                                        ));
                    }
                    "ServiceUnavailableException" => {
                        let mut builder = crate::error::ServiceUnavailableException::builder();
                        builder = crate::json_deser::deser_structure_crate_error_service_unavailable_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall ServiceUnavailableException: {}", err))
                                                            })?;
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::error::MedicalTranscriptResultStreamError::new(
                                                                crate::error::MedicalTranscriptResultStreamErrorKind::ServiceUnavailableException(builder.build()),
                                                                generic,
                                                            )
                                                        ));
                    }
                    _ => {}
                }
                Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                    crate::error::MedicalTranscriptResultStreamError::generic(generic),
                ))
            }
            value => {
                return Err(aws_smithy_eventstream::error::Error::unmarshalling(
                    format!("unrecognized :message-type: {}", value),
                ));
            }
        }
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct TranscriptResultStreamUnmarshaller;

impl TranscriptResultStreamUnmarshaller {
    pub fn new() -> Self {
        TranscriptResultStreamUnmarshaller
    }
}
impl aws_smithy_eventstream::frame::UnmarshallMessage for TranscriptResultStreamUnmarshaller {
    type Output = crate::model::TranscriptResultStream;
    type Error = crate::error::TranscriptResultStreamError;
    fn unmarshall(
        &self,
        message: &aws_smithy_eventstream::frame::Message,
    ) -> std::result::Result<
        aws_smithy_eventstream::frame::UnmarshalledMessage<Self::Output, Self::Error>,
        aws_smithy_eventstream::error::Error,
    > {
        let response_headers = aws_smithy_eventstream::smithy::parse_response_headers(message)?;
        match response_headers.message_type.as_str() {
            "event" => {
                match response_headers.smithy_type.as_str() {
                    "TranscriptEvent" => {
                        let parsed =
                            crate::json_deser::deser_structure_crate_model_transcript_event_payload(&message.payload()[..])
                                            .map_err(|err| {
                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall TranscriptEvent: {}", err))
                                            })?
                        ;
                        Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                            crate::model::TranscriptResultStream::TranscriptEvent(parsed),
                        ))
                    }
                    _unknown_variant => {
                        Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                            crate::model::TranscriptResultStream::Unknown,
                        ))
                    }
                }
            }
            "exception" => {
                let generic =
                    match crate::json_deser::parse_event_stream_generic_error(message.payload()) {
                        Ok(generic) => generic,
                        Err(err) => {
                            return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                crate::error::TranscriptResultStreamError::unhandled(err),
                            ))
                        }
                    };
                match response_headers.smithy_type.as_str() {
                    "BadRequestException" => {
                        let mut builder = crate::error::BadRequestException::builder();
                        builder = crate::json_deser::deser_structure_crate_error_bad_request_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall BadRequestException: {}", err))
                                                            })?;
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                            crate::error::TranscriptResultStreamError::new(
                                crate::error::TranscriptResultStreamErrorKind::BadRequestException(
                                    builder.build(),
                                ),
                                generic,
                            ),
                        ));
                    }
                    "LimitExceededException" => {
                        let mut builder = crate::error::LimitExceededException::builder();
                        builder = crate::json_deser::deser_structure_crate_error_limit_exceeded_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall LimitExceededException: {}", err))
                                                            })?;
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::error::TranscriptResultStreamError::new(
                                                                crate::error::TranscriptResultStreamErrorKind::LimitExceededException(builder.build()),
                                                                generic,
                                                            )
                                                        ));
                    }
                    "InternalFailureException" => {
                        let mut builder = crate::error::InternalFailureException::builder();
                        builder = crate::json_deser::deser_structure_crate_error_internal_failure_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall InternalFailureException: {}", err))
                                                            })?;
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::error::TranscriptResultStreamError::new(
                                                                crate::error::TranscriptResultStreamErrorKind::InternalFailureException(builder.build()),
                                                                generic,
                                                            )
                                                        ));
                    }
                    "ConflictException" => {
                        let mut builder = crate::error::ConflictException::builder();
                        builder = crate::json_deser::deser_structure_crate_error_conflict_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall ConflictException: {}", err))
                                                            })?;
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                            crate::error::TranscriptResultStreamError::new(
                                crate::error::TranscriptResultStreamErrorKind::ConflictException(
                                    builder.build(),
                                ),
                                generic,
                            ),
                        ));
                    }
                    "ServiceUnavailableException" => {
                        let mut builder = crate::error::ServiceUnavailableException::builder();
                        builder = crate::json_deser::deser_structure_crate_error_service_unavailable_exception_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::unmarshalling(format!("failed to unmarshall ServiceUnavailableException: {}", err))
                                                            })?;
                        return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                                                            crate::error::TranscriptResultStreamError::new(
                                                                crate::error::TranscriptResultStreamErrorKind::ServiceUnavailableException(builder.build()),
                                                                generic,
                                                            )
                                                        ));
                    }
                    _ => {}
                }
                Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                    crate::error::TranscriptResultStreamError::generic(generic),
                ))
            }
            value => {
                return Err(aws_smithy_eventstream::error::Error::unmarshalling(
                    format!("unrecognized :message-type: {}", value),
                ));
            }
        }
    }
}
