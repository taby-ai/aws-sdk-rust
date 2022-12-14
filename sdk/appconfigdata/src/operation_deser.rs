// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]pub fn parse_get_latest_configuration_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetLatestConfigurationOutput, crate::error::GetLatestConfigurationError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::GetLatestConfigurationError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetLatestConfigurationError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::GetLatestConfigurationError { meta: generic, kind: crate::error::GetLatestConfigurationErrorKind::BadRequestException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetLatestConfigurationError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::GetLatestConfigurationError { meta: generic, kind: crate::error::GetLatestConfigurationErrorKind::InternalServerException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetLatestConfigurationError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::GetLatestConfigurationError { meta: generic, kind: crate::error::GetLatestConfigurationErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetLatestConfigurationError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::GetLatestConfigurationError { meta: generic, kind: crate::error::GetLatestConfigurationErrorKind::ThrottlingException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetLatestConfigurationError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::GetLatestConfigurationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_get_latest_configuration_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetLatestConfigurationOutput, crate::error::GetLatestConfigurationError> {
    Ok({
        #[allow(unused_mut)]let mut output = crate::output::get_latest_configuration_output::Builder::default();
        let _ = response;
        output = output.set_configuration(
            crate::http_serde::deser_payload_get_latest_configuration_get_latest_configuration_output_configuration(response.body().as_ref())?
        );
        output = output.set_content_type(
            crate::http_serde::deser_header_get_latest_configuration_get_latest_configuration_output_content_type(response.headers())
                                    .map_err(|_|crate::error::GetLatestConfigurationError::unhandled("Failed to parse ContentType from header `Content-Type"))?
        );
        output = output.set_next_poll_configuration_token(
            crate::http_serde::deser_header_get_latest_configuration_get_latest_configuration_output_next_poll_configuration_token(response.headers())
                                    .map_err(|_|crate::error::GetLatestConfigurationError::unhandled("Failed to parse NextPollConfigurationToken from header `Next-Poll-Configuration-Token"))?
        );
        output = output.set_next_poll_interval_in_seconds(
            crate::http_serde::deser_header_get_latest_configuration_get_latest_configuration_output_next_poll_interval_in_seconds(response.headers())
                                    .map_err(|_|crate::error::GetLatestConfigurationError::unhandled("Failed to parse NextPollIntervalInSeconds from header `Next-Poll-Interval-In-Seconds"))?
        );
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_start_configuration_session_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StartConfigurationSessionOutput, crate::error::StartConfigurationSessionError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::StartConfigurationSessionError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::StartConfigurationSessionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::StartConfigurationSessionError { meta: generic, kind: crate::error::StartConfigurationSessionErrorKind::BadRequestException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartConfigurationSessionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::StartConfigurationSessionError { meta: generic, kind: crate::error::StartConfigurationSessionErrorKind::InternalServerException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartConfigurationSessionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::StartConfigurationSessionError { meta: generic, kind: crate::error::StartConfigurationSessionErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartConfigurationSessionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::StartConfigurationSessionError { meta: generic, kind: crate::error::StartConfigurationSessionErrorKind::ThrottlingException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartConfigurationSessionError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::StartConfigurationSessionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_start_configuration_session_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StartConfigurationSessionOutput, crate::error::StartConfigurationSessionError> {
    Ok({
        #[allow(unused_mut)]let mut output = crate::output::start_configuration_session_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_start_configuration_session(response.body().as_ref(), output).map_err(crate::error::StartConfigurationSessionError::unhandled)?;
        output.build()
    })
}

