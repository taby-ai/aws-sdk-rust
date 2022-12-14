// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn add_headers_create_connector_definition(
                    input: &crate::input::CreateConnectorDefinitionInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_1) = &input.amzn_client_token {
        let formatted_2 = AsRef::<str>::as_ref(inner_1);
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_connector_definition_version(
                    input: &crate::input::CreateConnectorDefinitionVersionInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_3) = &input.amzn_client_token {
        let formatted_4 = AsRef::<str>::as_ref(inner_3);
        if !formatted_4.is_empty() {
            let header_value = formatted_4;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_core_definition(
                    input: &crate::input::CreateCoreDefinitionInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_5) = &input.amzn_client_token {
        let formatted_6 = AsRef::<str>::as_ref(inner_5);
        if !formatted_6.is_empty() {
            let header_value = formatted_6;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_core_definition_version(
                    input: &crate::input::CreateCoreDefinitionVersionInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_7) = &input.amzn_client_token {
        let formatted_8 = AsRef::<str>::as_ref(inner_7);
        if !formatted_8.is_empty() {
            let header_value = formatted_8;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_deployment(
                    input: &crate::input::CreateDeploymentInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_9) = &input.amzn_client_token {
        let formatted_10 = AsRef::<str>::as_ref(inner_9);
        if !formatted_10.is_empty() {
            let header_value = formatted_10;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_device_definition(
                    input: &crate::input::CreateDeviceDefinitionInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_11) = &input.amzn_client_token {
        let formatted_12 = AsRef::<str>::as_ref(inner_11);
        if !formatted_12.is_empty() {
            let header_value = formatted_12;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_device_definition_version(
                    input: &crate::input::CreateDeviceDefinitionVersionInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_13) = &input.amzn_client_token {
        let formatted_14 = AsRef::<str>::as_ref(inner_13);
        if !formatted_14.is_empty() {
            let header_value = formatted_14;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_function_definition(
                    input: &crate::input::CreateFunctionDefinitionInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_15) = &input.amzn_client_token {
        let formatted_16 = AsRef::<str>::as_ref(inner_15);
        if !formatted_16.is_empty() {
            let header_value = formatted_16;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_function_definition_version(
                    input: &crate::input::CreateFunctionDefinitionVersionInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_17) = &input.amzn_client_token {
        let formatted_18 = AsRef::<str>::as_ref(inner_17);
        if !formatted_18.is_empty() {
            let header_value = formatted_18;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_group(
                    input: &crate::input::CreateGroupInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_19) = &input.amzn_client_token {
        let formatted_20 = AsRef::<str>::as_ref(inner_19);
        if !formatted_20.is_empty() {
            let header_value = formatted_20;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_group_certificate_authority(
                    input: &crate::input::CreateGroupCertificateAuthorityInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_21) = &input.amzn_client_token {
        let formatted_22 = AsRef::<str>::as_ref(inner_21);
        if !formatted_22.is_empty() {
            let header_value = formatted_22;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_group_version(
                    input: &crate::input::CreateGroupVersionInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_23) = &input.amzn_client_token {
        let formatted_24 = AsRef::<str>::as_ref(inner_23);
        if !formatted_24.is_empty() {
            let header_value = formatted_24;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_logger_definition(
                    input: &crate::input::CreateLoggerDefinitionInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_25) = &input.amzn_client_token {
        let formatted_26 = AsRef::<str>::as_ref(inner_25);
        if !formatted_26.is_empty() {
            let header_value = formatted_26;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_logger_definition_version(
                    input: &crate::input::CreateLoggerDefinitionVersionInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_27) = &input.amzn_client_token {
        let formatted_28 = AsRef::<str>::as_ref(inner_27);
        if !formatted_28.is_empty() {
            let header_value = formatted_28;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_resource_definition(
                    input: &crate::input::CreateResourceDefinitionInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_29) = &input.amzn_client_token {
        let formatted_30 = AsRef::<str>::as_ref(inner_29);
        if !formatted_30.is_empty() {
            let header_value = formatted_30;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_resource_definition_version(
                    input: &crate::input::CreateResourceDefinitionVersionInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_31) = &input.amzn_client_token {
        let formatted_32 = AsRef::<str>::as_ref(inner_31);
        if !formatted_32.is_empty() {
            let header_value = formatted_32;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_software_update_job(
                    input: &crate::input::CreateSoftwareUpdateJobInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_33) = &input.amzn_client_token {
        let formatted_34 = AsRef::<str>::as_ref(inner_33);
        if !formatted_34.is_empty() {
            let header_value = formatted_34;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_subscription_definition(
                    input: &crate::input::CreateSubscriptionDefinitionInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_35) = &input.amzn_client_token {
        let formatted_36 = AsRef::<str>::as_ref(inner_35);
        if !formatted_36.is_empty() {
            let header_value = formatted_36;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_create_subscription_definition_version(
                    input: &crate::input::CreateSubscriptionDefinitionVersionInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_37) = &input.amzn_client_token {
        let formatted_38 = AsRef::<str>::as_ref(inner_37);
        if !formatted_38.is_empty() {
            let header_value = formatted_38;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_reset_deployments(
                    input: &crate::input::ResetDeploymentsInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_39) = &input.amzn_client_token {
        let formatted_40 = AsRef::<str>::as_ref(inner_39);
        if !formatted_40.is_empty() {
            let header_value = formatted_40;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

pub fn add_headers_start_bulk_deployment(
                    input: &crate::input::StartBulkDeploymentInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
    if let Some(inner_41) = &input.amzn_client_token {
        let formatted_42 = AsRef::<str>::as_ref(inner_41);
        if !formatted_42.is_empty() {
            let header_value = formatted_42;
                                    let header_value = http::header::HeaderValue::try_from(&*header_value).map_err(|err| {
                                        aws_smithy_http::operation::BuildError::InvalidField { 
                                            field: "amzn_client_token", 
                                            details: format!(
                                                "`{}` cannot be used as a header value: {}", 
                                                &header_value,
                                                err,
                                            )
                                        }
                                    })?;
                                    builder = builder.header("X-Amzn-Client-Token", header_value);
        }
    }
    Ok(builder)
}

