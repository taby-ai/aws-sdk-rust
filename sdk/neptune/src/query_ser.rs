// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]pub fn serialize_structure_crate_model_tag(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::Tag) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_1 = writer.prefix("Key");
    if let Some(var_2) = &input.key {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]let mut scope_3 = writer.prefix("Value");
    if let Some(var_4) = &input.value {
        scope_3.string(var_4);
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_filter(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::Filter) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_5 = writer.prefix("Name");
    if let Some(var_6) = &input.name {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]let mut scope_7 = writer.prefix("Values");
    if let Some(var_8) = &input.values {
        let mut list_10 = scope_7.start_list(false, Some("Value"));
        for item_9 in var_8 {
            #[allow(unused_mut)]let mut entry_11 = list_10.entry();
            entry_11.string(item_9);
        }
        list_10.finish();
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_cloudwatch_logs_export_configuration(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::CloudwatchLogsExportConfiguration) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_12 = writer.prefix("EnableLogTypes");
    if let Some(var_13) = &input.enable_log_types {
        let mut list_15 = scope_12.start_list(false, None);
        for item_14 in var_13 {
            #[allow(unused_mut)]let mut entry_16 = list_15.entry();
            entry_16.string(item_14);
        }
        list_15.finish();
    }
    #[allow(unused_mut)]let mut scope_17 = writer.prefix("DisableLogTypes");
    if let Some(var_18) = &input.disable_log_types {
        let mut list_20 = scope_17.start_list(false, None);
        for item_19 in var_18 {
            #[allow(unused_mut)]let mut entry_21 = list_20.entry();
            entry_21.string(item_19);
        }
        list_20.finish();
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_parameter(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::Parameter) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_22 = writer.prefix("ParameterName");
    if let Some(var_23) = &input.parameter_name {
        scope_22.string(var_23);
    }
    #[allow(unused_mut)]let mut scope_24 = writer.prefix("ParameterValue");
    if let Some(var_25) = &input.parameter_value {
        scope_24.string(var_25);
    }
    #[allow(unused_mut)]let mut scope_26 = writer.prefix("Description");
    if let Some(var_27) = &input.description {
        scope_26.string(var_27);
    }
    #[allow(unused_mut)]let mut scope_28 = writer.prefix("Source");
    if let Some(var_29) = &input.source {
        scope_28.string(var_29);
    }
    #[allow(unused_mut)]let mut scope_30 = writer.prefix("ApplyType");
    if let Some(var_31) = &input.apply_type {
        scope_30.string(var_31);
    }
    #[allow(unused_mut)]let mut scope_32 = writer.prefix("DataType");
    if let Some(var_33) = &input.data_type {
        scope_32.string(var_33);
    }
    #[allow(unused_mut)]let mut scope_34 = writer.prefix("AllowedValues");
    if let Some(var_35) = &input.allowed_values {
        scope_34.string(var_35);
    }
    #[allow(unused_mut)]let mut scope_36 = writer.prefix("IsModifiable");
    if input.is_modifiable {
        scope_36.boolean(input.is_modifiable);
    }
    #[allow(unused_mut)]let mut scope_37 = writer.prefix("MinimumEngineVersion");
    if let Some(var_38) = &input.minimum_engine_version {
        scope_37.string(var_38);
    }
    #[allow(unused_mut)]let mut scope_39 = writer.prefix("ApplyMethod");
    if let Some(var_40) = &input.apply_method {
        scope_39.string(var_40.as_str());
    }
    Ok(())
}

