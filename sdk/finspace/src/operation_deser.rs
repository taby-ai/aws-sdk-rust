// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]pub fn parse_create_environment_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateEnvironmentOutput, crate::error::CreateEnvironmentError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::CreateEnvironmentError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateEnvironmentError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::CreateEnvironmentError { meta: generic, kind: crate::error::CreateEnvironmentErrorKind::AccessDeniedException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::CreateEnvironmentError { meta: generic, kind: crate::error::CreateEnvironmentErrorKind::InternalServerException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "LimitExceededException" => crate::error::CreateEnvironmentError { meta: generic, kind: crate::error::CreateEnvironmentErrorKind::LimitExceededException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ServiceQuotaExceededException" => crate::error::CreateEnvironmentError { meta: generic, kind: crate::error::CreateEnvironmentErrorKind::ServiceQuotaExceededException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::service_quota_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_service_quota_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::CreateEnvironmentError { meta: generic, kind: crate::error::CreateEnvironmentErrorKind::ThrottlingException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::CreateEnvironmentError { meta: generic, kind: crate::error::CreateEnvironmentErrorKind::ValidationException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::CreateEnvironmentError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_create_environment_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateEnvironmentOutput, crate::error::CreateEnvironmentError> {
    Ok({
        #[allow(unused_mut)]let mut output = crate::output::create_environment_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_create_environment(response.body().as_ref(), output).map_err(crate::error::CreateEnvironmentError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_delete_environment_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteEnvironmentOutput, crate::error::DeleteEnvironmentError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::DeleteEnvironmentError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteEnvironmentError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::DeleteEnvironmentError { meta: generic, kind: crate::error::DeleteEnvironmentErrorKind::AccessDeniedException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::DeleteEnvironmentError { meta: generic, kind: crate::error::DeleteEnvironmentErrorKind::InternalServerException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::DeleteEnvironmentError { meta: generic, kind: crate::error::DeleteEnvironmentErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::DeleteEnvironmentError { meta: generic, kind: crate::error::DeleteEnvironmentErrorKind::ThrottlingException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::DeleteEnvironmentError { meta: generic, kind: crate::error::DeleteEnvironmentErrorKind::ValidationException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::DeleteEnvironmentError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_delete_environment_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteEnvironmentOutput, crate::error::DeleteEnvironmentError> {
    Ok({
        #[allow(unused_mut)]let mut output = crate::output::delete_environment_output::Builder::default();
        let _ = response;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_get_environment_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetEnvironmentOutput, crate::error::GetEnvironmentError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::GetEnvironmentError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetEnvironmentError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::GetEnvironmentError { meta: generic, kind: crate::error::GetEnvironmentErrorKind::AccessDeniedException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::GetEnvironmentError { meta: generic, kind: crate::error::GetEnvironmentErrorKind::InternalServerException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::GetEnvironmentError { meta: generic, kind: crate::error::GetEnvironmentErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::GetEnvironmentError { meta: generic, kind: crate::error::GetEnvironmentErrorKind::ValidationException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::GetEnvironmentError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_get_environment_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetEnvironmentOutput, crate::error::GetEnvironmentError> {
    Ok({
        #[allow(unused_mut)]let mut output = crate::output::get_environment_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_get_environment(response.body().as_ref(), output).map_err(crate::error::GetEnvironmentError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_list_environments_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListEnvironmentsOutput, crate::error::ListEnvironmentsError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::ListEnvironmentsError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListEnvironmentsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::ListEnvironmentsError { meta: generic, kind: crate::error::ListEnvironmentsErrorKind::InternalServerException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListEnvironmentsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::ListEnvironmentsError { meta: generic, kind: crate::error::ListEnvironmentsErrorKind::ValidationException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListEnvironmentsError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::ListEnvironmentsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_list_environments_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListEnvironmentsOutput, crate::error::ListEnvironmentsError> {
    Ok({
        #[allow(unused_mut)]let mut output = crate::output::list_environments_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_list_environments(response.body().as_ref(), output).map_err(crate::error::ListEnvironmentsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_list_tags_for_resource_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListTagsForResourceOutput, crate::error::ListTagsForResourceError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::ListTagsForResourceError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListTagsForResourceError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::ListTagsForResourceError { meta: generic, kind: crate::error::ListTagsForResourceErrorKind::InternalServerException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListTagsForResourceError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InvalidRequestException" => crate::error::ListTagsForResourceError { meta: generic, kind: crate::error::ListTagsForResourceErrorKind::InvalidRequestException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListTagsForResourceError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::ListTagsForResourceError { meta: generic, kind: crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListTagsForResourceError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::ListTagsForResourceError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_list_tags_for_resource_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListTagsForResourceOutput, crate::error::ListTagsForResourceError> {
    Ok({
        #[allow(unused_mut)]let mut output = crate::output::list_tags_for_resource_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_list_tags_for_resource(response.body().as_ref(), output).map_err(crate::error::ListTagsForResourceError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_tag_resource_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::TagResourceError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::TagResourceError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::TagResourceError { meta: generic, kind: crate::error::TagResourceErrorKind::InternalServerException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::TagResourceError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InvalidRequestException" => crate::error::TagResourceError { meta: generic, kind: crate::error::TagResourceErrorKind::InvalidRequestException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::TagResourceError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::TagResourceError { meta: generic, kind: crate::error::TagResourceErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::TagResourceError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::TagResourceError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_tag_resource_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError> {
    Ok({
        #[allow(unused_mut)]let mut output = crate::output::tag_resource_output::Builder::default();
        let _ = response;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_untag_resource_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::UntagResourceError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UntagResourceError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::UntagResourceError { meta: generic, kind: crate::error::UntagResourceErrorKind::InternalServerException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UntagResourceError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InvalidRequestException" => crate::error::UntagResourceError { meta: generic, kind: crate::error::UntagResourceErrorKind::InvalidRequestException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UntagResourceError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::UntagResourceError { meta: generic, kind: crate::error::UntagResourceErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UntagResourceError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::UntagResourceError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_untag_resource_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError> {
    Ok({
        #[allow(unused_mut)]let mut output = crate::output::untag_resource_output::Builder::default();
        let _ = response;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_update_environment_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateEnvironmentOutput, crate::error::UpdateEnvironmentError> {
    let generic = crate::json_deser::parse_http_generic_error(response).map_err(crate::error::UpdateEnvironmentError::unhandled)?;
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdateEnvironmentError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::UpdateEnvironmentError { meta: generic, kind: crate::error::UpdateEnvironmentErrorKind::AccessDeniedException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "InternalServerException" => crate::error::UpdateEnvironmentError { meta: generic, kind: crate::error::UpdateEnvironmentErrorKind::InternalServerException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ResourceNotFoundException" => crate::error::UpdateEnvironmentError { meta: generic, kind: crate::error::UpdateEnvironmentErrorKind::ResourceNotFoundException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ThrottlingException" => crate::error::UpdateEnvironmentError { meta: generic, kind: crate::error::UpdateEnvironmentErrorKind::ThrottlingException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        "ValidationException" => crate::error::UpdateEnvironmentError { meta: generic, kind: crate::error::UpdateEnvironmentErrorKind::ValidationException({
            #[allow(unused_mut)]let mut tmp =
                 {
                    #[allow(unused_mut)]let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateEnvironmentError::unhandled)?;
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        })},
        _ => crate::error::UpdateEnvironmentError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_update_environment_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateEnvironmentOutput, crate::error::UpdateEnvironmentError> {
    Ok({
        #[allow(unused_mut)]let mut output = crate::output::update_environment_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_update_environment(response.body().as_ref(), output).map_err(crate::error::UpdateEnvironmentError::unhandled)?;
        output.build()
    })
}

