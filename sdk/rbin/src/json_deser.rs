// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn parse_http_generic_error(response: &http::Response<bytes::Bytes>) -> Result<aws_smithy_types::Error, aws_smithy_json::deserialize::Error> {
                    crate::json_errors::parse_generic_error(response.body(), response.headers())
                }

pub fn deser_structure_crate_error_internal_server_exception_json_err(value: &[u8], mut builder: crate::error::internal_server_exception::Builder) -> Result<crate::error::internal_server_exception::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                }
            }
            other => return Err(aws_smithy_json::deserialize::Error::custom(format!("expected object key or end object, found: {:?}", other)))
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom("found more JSON tokens after completing parsing"));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_service_quota_exceeded_exception_json_err(value: &[u8], mut builder: crate::error::service_quota_exceeded_exception::Builder) -> Result<crate::error::service_quota_exceeded_exception::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Reason" => {
                        builder = builder.set_reason(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::ServiceQuotaExceededExceptionReason::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                }
            }
            other => return Err(aws_smithy_json::deserialize::Error::custom(format!("expected object key or end object, found: {:?}", other)))
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom("found more JSON tokens after completing parsing"));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_validation_exception_json_err(value: &[u8], mut builder: crate::error::validation_exception::Builder) -> Result<crate::error::validation_exception::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Reason" => {
                        builder = builder.set_reason(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::ValidationExceptionReason::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                }
            }
            other => return Err(aws_smithy_json::deserialize::Error::custom(format!("expected object key or end object, found: {:?}", other)))
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom("found more JSON tokens after completing parsing"));
    }
    Ok(builder)
}

pub fn deser_operation_crate_operation_create_rule(value: &[u8], mut builder: crate::output::create_rule_output::Builder) -> Result<crate::output::create_rule_output::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Description" => {
                        builder = builder.set_description(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Identifier" => {
                        builder = builder.set_identifier(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ResourceTags" => {
                        builder = builder.set_resource_tags(
                            crate::json_deser::deser_list_com_amazonaws_rbin_resource_tags(tokens)?
                        );
                    }
                    "ResourceType" => {
                        builder = builder.set_resource_type(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::ResourceType::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "RetentionPeriod" => {
                        builder = builder.set_retention_period(
                            crate::json_deser::deser_structure_crate_model_retention_period(tokens)?
                        );
                    }
                    "Status" => {
                        builder = builder.set_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::RuleStatus::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "Tags" => {
                        builder = builder.set_tags(
                            crate::json_deser::deser_list_com_amazonaws_rbin_tag_list(tokens)?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                }
            }
            other => return Err(aws_smithy_json::deserialize::Error::custom(format!("expected object key or end object, found: {:?}", other)))
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom("found more JSON tokens after completing parsing"));
    }
    Ok(builder)
}

pub fn deser_structure_crate_error_resource_not_found_exception_json_err(value: &[u8], mut builder: crate::error::resource_not_found_exception::Builder) -> Result<crate::error::resource_not_found_exception::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Message" => {
                        builder = builder.set_message(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Reason" => {
                        builder = builder.set_reason(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::ResourceNotFoundExceptionReason::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                }
            }
            other => return Err(aws_smithy_json::deserialize::Error::custom(format!("expected object key or end object, found: {:?}", other)))
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom("found more JSON tokens after completing parsing"));
    }
    Ok(builder)
}

pub fn deser_operation_crate_operation_get_rule(value: &[u8], mut builder: crate::output::get_rule_output::Builder) -> Result<crate::output::get_rule_output::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Description" => {
                        builder = builder.set_description(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Identifier" => {
                        builder = builder.set_identifier(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ResourceTags" => {
                        builder = builder.set_resource_tags(
                            crate::json_deser::deser_list_com_amazonaws_rbin_resource_tags(tokens)?
                        );
                    }
                    "ResourceType" => {
                        builder = builder.set_resource_type(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::ResourceType::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "RetentionPeriod" => {
                        builder = builder.set_retention_period(
                            crate::json_deser::deser_structure_crate_model_retention_period(tokens)?
                        );
                    }
                    "Status" => {
                        builder = builder.set_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::RuleStatus::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                }
            }
            other => return Err(aws_smithy_json::deserialize::Error::custom(format!("expected object key or end object, found: {:?}", other)))
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom("found more JSON tokens after completing parsing"));
    }
    Ok(builder)
}

pub fn deser_operation_crate_operation_list_rules(value: &[u8], mut builder: crate::output::list_rules_output::Builder) -> Result<crate::output::list_rules_output::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "NextToken" => {
                        builder = builder.set_next_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Rules" => {
                        builder = builder.set_rules(
                            crate::json_deser::deser_list_com_amazonaws_rbin_rule_summary_list(tokens)?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                }
            }
            other => return Err(aws_smithy_json::deserialize::Error::custom(format!("expected object key or end object, found: {:?}", other)))
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom("found more JSON tokens after completing parsing"));
    }
    Ok(builder)
}

pub fn deser_operation_crate_operation_list_tags_for_resource(value: &[u8], mut builder: crate::output::list_tags_for_resource_output::Builder) -> Result<crate::output::list_tags_for_resource_output::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Tags" => {
                        builder = builder.set_tags(
                            crate::json_deser::deser_list_com_amazonaws_rbin_tag_list(tokens)?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                }
            }
            other => return Err(aws_smithy_json::deserialize::Error::custom(format!("expected object key or end object, found: {:?}", other)))
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom("found more JSON tokens after completing parsing"));
    }
    Ok(builder)
}

pub fn deser_operation_crate_operation_update_rule(value: &[u8], mut builder: crate::output::update_rule_output::Builder) -> Result<crate::output::update_rule_output::Builder, aws_smithy_json::deserialize::Error> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::json_deser::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Description" => {
                        builder = builder.set_description(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Identifier" => {
                        builder = builder.set_identifier(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ResourceTags" => {
                        builder = builder.set_resource_tags(
                            crate::json_deser::deser_list_com_amazonaws_rbin_resource_tags(tokens)?
                        );
                    }
                    "ResourceType" => {
                        builder = builder.set_resource_type(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::ResourceType::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "RetentionPeriod" => {
                        builder = builder.set_retention_period(
                            crate::json_deser::deser_structure_crate_model_retention_period(tokens)?
                        );
                    }
                    "Status" => {
                        builder = builder.set_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::RuleStatus::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                }
            }
            other => return Err(aws_smithy_json::deserialize::Error::custom(format!("expected object key or end object, found: {:?}", other)))
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::Error::custom("found more JSON tokens after completing parsing"));
    }
    Ok(builder)
}

pub fn or_empty_doc(data: &[u8]) -> &[u8] {
                if data.is_empty() {
                    b"{}"
                } else {
                    data
                }
            }

#[allow(clippy::type_complexity, non_snake_case)]
                pub fn deser_list_com_amazonaws_rbin_resource_tags<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<std::vec::Vec<crate::model::ResourceTag>>, aws_smithy_json::deserialize::Error>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartArray { .. }) => {
            let mut items = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Ok(aws_smithy_json::deserialize::Token::EndArray { .. })) => {
                        tokens.next().transpose().unwrap(); break;
                    }
                    _ =>  {
                        let value =
                            crate::json_deser::deser_structure_crate_model_resource_tag(tokens)?
                        ;
                        if let Some(value) = value {
                            items.push(value);
                        }
                    }
                }
            }
            Ok(Some(items))
        }
        _ => {
            Err(aws_smithy_json::deserialize::Error::custom("expected start array or null"))
        }
    }
}

pub fn deser_structure_crate_model_retention_period<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::RetentionPeriod>, aws_smithy_json::deserialize::Error>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]let mut builder = crate::model::RetentionPeriod::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "RetentionPeriodValue" => {
                                builder = builder.set_retention_period_value(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(|v| v.try_into())
                                                        .transpose()?
                                );
                            }
                            "RetentionPeriodUnit" => {
                                builder = builder.set_retention_period_unit(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::RetentionPeriodUnit::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                        }
                    }
                    other => return Err(aws_smithy_json::deserialize::Error::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
            Ok(Some(builder.build()
            ))
        }
        _ => {
            Err(aws_smithy_json::deserialize::Error::custom("expected start object or null"))
        }
    }
}

#[allow(clippy::type_complexity, non_snake_case)]
                pub fn deser_list_com_amazonaws_rbin_tag_list<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<std::vec::Vec<crate::model::Tag>>, aws_smithy_json::deserialize::Error>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartArray { .. }) => {
            let mut items = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Ok(aws_smithy_json::deserialize::Token::EndArray { .. })) => {
                        tokens.next().transpose().unwrap(); break;
                    }
                    _ =>  {
                        let value =
                            crate::json_deser::deser_structure_crate_model_tag(tokens)?
                        ;
                        if let Some(value) = value {
                            items.push(value);
                        }
                    }
                }
            }
            Ok(Some(items))
        }
        _ => {
            Err(aws_smithy_json::deserialize::Error::custom("expected start array or null"))
        }
    }
}

#[allow(clippy::type_complexity, non_snake_case)]
                pub fn deser_list_com_amazonaws_rbin_rule_summary_list<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<std::vec::Vec<crate::model::RuleSummary>>, aws_smithy_json::deserialize::Error>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartArray { .. }) => {
            let mut items = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Ok(aws_smithy_json::deserialize::Token::EndArray { .. })) => {
                        tokens.next().transpose().unwrap(); break;
                    }
                    _ =>  {
                        let value =
                            crate::json_deser::deser_structure_crate_model_rule_summary(tokens)?
                        ;
                        if let Some(value) = value {
                            items.push(value);
                        }
                    }
                }
            }
            Ok(Some(items))
        }
        _ => {
            Err(aws_smithy_json::deserialize::Error::custom("expected start array or null"))
        }
    }
}

pub fn deser_structure_crate_model_resource_tag<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ResourceTag>, aws_smithy_json::deserialize::Error>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]let mut builder = crate::model::ResourceTag::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ResourceTagKey" => {
                                builder = builder.set_resource_tag_key(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "ResourceTagValue" => {
                                builder = builder.set_resource_tag_value(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                        }
                    }
                    other => return Err(aws_smithy_json::deserialize::Error::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
            Ok(Some(builder.build()
            ))
        }
        _ => {
            Err(aws_smithy_json::deserialize::Error::custom("expected start object or null"))
        }
    }
}

pub fn deser_structure_crate_model_tag<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Tag>, aws_smithy_json::deserialize::Error>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]let mut builder = crate::model::Tag::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Key" => {
                                builder = builder.set_key(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Value" => {
                                builder = builder.set_value(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                        }
                    }
                    other => return Err(aws_smithy_json::deserialize::Error::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
            Ok(Some(builder.build()
            ))
        }
        _ => {
            Err(aws_smithy_json::deserialize::Error::custom("expected start object or null"))
        }
    }
}

pub fn deser_structure_crate_model_rule_summary<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::RuleSummary>, aws_smithy_json::deserialize::Error>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::Error>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]let mut builder = crate::model::RuleSummary::builder();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Identifier" => {
                                builder = builder.set_identifier(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Description" => {
                                builder = builder.set_description(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "RetentionPeriod" => {
                                builder = builder.set_retention_period(
                                    crate::json_deser::deser_structure_crate_model_retention_period(tokens)?
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                        }
                    }
                    other => return Err(aws_smithy_json::deserialize::Error::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
            Ok(Some(builder.build()
            ))
        }
        _ => {
            Err(aws_smithy_json::deserialize::Error::custom("expected start object or null"))
        }
    }
}

