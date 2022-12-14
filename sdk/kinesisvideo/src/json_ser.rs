// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_signaling_channel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateSignalingChannelInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.channel_name {
        object.key("ChannelName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.channel_type {
        object.key("ChannelType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.single_master_configuration {
        let mut object_4 = object.key("SingleMasterConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_single_master_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.tags {
        let mut array_6 = object.key("Tags").start_array();
        for item_7 in var_5 {
             {
                let mut object_8 = array_6.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateStreamInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_9) = &input.data_retention_in_hours {
        object.key("DataRetentionInHours").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_9).into()));
    }
    if let Some(var_10) = &input.device_name {
        object.key("DeviceName").string(var_10.as_str());
    }
    if let Some(var_11) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_11.as_str());
    }
    if let Some(var_12) = &input.media_type {
        object.key("MediaType").string(var_12.as_str());
    }
    if let Some(var_13) = &input.stream_name {
        object.key("StreamName").string(var_13.as_str());
    }
    if let Some(var_14) = &input.tags {
        let mut object_15 = object.key("Tags").start_object();
        for (key_16, value_17) in var_14 {
             {
                object_15.key(key_16).string(value_17.as_str());
            }
        }
        object_15.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_signaling_channel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteSignalingChannelInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_18) = &input.channel_arn {
        object.key("ChannelARN").string(var_18.as_str());
    }
    if let Some(var_19) = &input.current_version {
        object.key("CurrentVersion").string(var_19.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteStreamInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_20) = &input.current_version {
        object.key("CurrentVersion").string(var_20.as_str());
    }
    if let Some(var_21) = &input.stream_arn {
        object.key("StreamARN").string(var_21.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_image_generation_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeImageGenerationConfigurationInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_22) = &input.stream_arn {
        object.key("StreamARN").string(var_22.as_str());
    }
    if let Some(var_23) = &input.stream_name {
        object.key("StreamName").string(var_23.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_notification_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeNotificationConfigurationInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_24) = &input.stream_arn {
        object.key("StreamARN").string(var_24.as_str());
    }
    if let Some(var_25) = &input.stream_name {
        object.key("StreamName").string(var_25.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_signaling_channel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeSignalingChannelInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_26) = &input.channel_arn {
        object.key("ChannelARN").string(var_26.as_str());
    }
    if let Some(var_27) = &input.channel_name {
        object.key("ChannelName").string(var_27.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeStreamInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.stream_arn {
        object.key("StreamARN").string(var_28.as_str());
    }
    if let Some(var_29) = &input.stream_name {
        object.key("StreamName").string(var_29.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_data_endpoint_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetDataEndpointInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_30) = &input.api_name {
        object.key("APIName").string(var_30.as_str());
    }
    if let Some(var_31) = &input.stream_arn {
        object.key("StreamARN").string(var_31.as_str());
    }
    if let Some(var_32) = &input.stream_name {
        object.key("StreamName").string(var_32.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_signaling_channel_endpoint_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetSignalingChannelEndpointInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.channel_arn {
        object.key("ChannelARN").string(var_33.as_str());
    }
    if let Some(var_34) = &input.single_master_channel_endpoint_configuration {
        let mut object_35 = object.key("SingleMasterChannelEndpointConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_single_master_channel_endpoint_configuration(&mut object_35, var_34)?;
        object_35.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_signaling_channels_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListSignalingChannelsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_36) = &input.channel_name_condition {
        let mut object_37 = object.key("ChannelNameCondition").start_object();
        crate::json_ser::serialize_structure_crate_model_channel_name_condition(&mut object_37, var_36)?;
        object_37.finish();
    }
    if let Some(var_38) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_38).into()));
    }
    if let Some(var_39) = &input.next_token {
        object.key("NextToken").string(var_39.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_streams_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListStreamsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_40) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_40).into()));
    }
    if let Some(var_41) = &input.next_token {
        object.key("NextToken").string(var_41.as_str());
    }
    if let Some(var_42) = &input.stream_name_condition {
        let mut object_43 = object.key("StreamNameCondition").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_name_condition(&mut object_43, var_42)?;
        object_43.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTagsForResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.next_token {
        object.key("NextToken").string(var_44.as_str());
    }
    if let Some(var_45) = &input.resource_arn {
        object.key("ResourceARN").string(var_45.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTagsForStreamInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.next_token {
        object.key("NextToken").string(var_46.as_str());
    }
    if let Some(var_47) = &input.stream_arn {
        object.key("StreamARN").string(var_47.as_str());
    }
    if let Some(var_48) = &input.stream_name {
        object.key("StreamName").string(var_48.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_49) = &input.resource_arn {
        object.key("ResourceARN").string(var_49.as_str());
    }
    if let Some(var_50) = &input.tags {
        let mut array_51 = object.key("Tags").start_array();
        for item_52 in var_50 {
             {
                let mut object_53 = array_51.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_53, item_52)?;
                object_53.finish();
            }
        }
        array_51.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagStreamInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_54) = &input.stream_arn {
        object.key("StreamARN").string(var_54.as_str());
    }
    if let Some(var_55) = &input.stream_name {
        object.key("StreamName").string(var_55.as_str());
    }
    if let Some(var_56) = &input.tags {
        let mut object_57 = object.key("Tags").start_object();
        for (key_58, value_59) in var_56 {
             {
                object_57.key(key_58).string(value_59.as_str());
            }
        }
        object_57.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UntagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_60) = &input.resource_arn {
        object.key("ResourceARN").string(var_60.as_str());
    }
    if let Some(var_61) = &input.tag_key_list {
        let mut array_62 = object.key("TagKeyList").start_array();
        for item_63 in var_61 {
             {
                array_62.value().string(item_63.as_str());
            }
        }
        array_62.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UntagStreamInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_64) = &input.stream_arn {
        object.key("StreamARN").string(var_64.as_str());
    }
    if let Some(var_65) = &input.stream_name {
        object.key("StreamName").string(var_65.as_str());
    }
    if let Some(var_66) = &input.tag_key_list {
        let mut array_67 = object.key("TagKeyList").start_array();
        for item_68 in var_66 {
             {
                array_67.value().string(item_68.as_str());
            }
        }
        array_67.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_data_retention_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateDataRetentionInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_69) = &input.current_version {
        object.key("CurrentVersion").string(var_69.as_str());
    }
    if let Some(var_70) = &input.data_retention_change_in_hours {
        object.key("DataRetentionChangeInHours").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_70).into()));
    }
    if let Some(var_71) = &input.operation {
        object.key("Operation").string(var_71.as_str());
    }
    if let Some(var_72) = &input.stream_arn {
        object.key("StreamARN").string(var_72.as_str());
    }
    if let Some(var_73) = &input.stream_name {
        object.key("StreamName").string(var_73.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_image_generation_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateImageGenerationConfigurationInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_74) = &input.image_generation_configuration {
        let mut object_75 = object.key("ImageGenerationConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_image_generation_configuration(&mut object_75, var_74)?;
        object_75.finish();
    }
    if let Some(var_76) = &input.stream_arn {
        object.key("StreamARN").string(var_76.as_str());
    }
    if let Some(var_77) = &input.stream_name {
        object.key("StreamName").string(var_77.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_notification_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateNotificationConfigurationInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_78) = &input.notification_configuration {
        let mut object_79 = object.key("NotificationConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_notification_configuration(&mut object_79, var_78)?;
        object_79.finish();
    }
    if let Some(var_80) = &input.stream_arn {
        object.key("StreamARN").string(var_80.as_str());
    }
    if let Some(var_81) = &input.stream_name {
        object.key("StreamName").string(var_81.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_signaling_channel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateSignalingChannelInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_82) = &input.channel_arn {
        object.key("ChannelARN").string(var_82.as_str());
    }
    if let Some(var_83) = &input.current_version {
        object.key("CurrentVersion").string(var_83.as_str());
    }
    if let Some(var_84) = &input.single_master_configuration {
        let mut object_85 = object.key("SingleMasterConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_single_master_configuration(&mut object_85, var_84)?;
        object_85.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateStreamInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_86) = &input.current_version {
        object.key("CurrentVersion").string(var_86.as_str());
    }
    if let Some(var_87) = &input.device_name {
        object.key("DeviceName").string(var_87.as_str());
    }
    if let Some(var_88) = &input.media_type {
        object.key("MediaType").string(var_88.as_str());
    }
    if let Some(var_89) = &input.stream_arn {
        object.key("StreamARN").string(var_89.as_str());
    }
    if let Some(var_90) = &input.stream_name {
        object.key("StreamName").string(var_90.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_single_master_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SingleMasterConfiguration) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_91) = &input.message_ttl_seconds {
        object.key("MessageTtlSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_91).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Tag) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_92) = &input.key {
        object.key("Key").string(var_92.as_str());
    }
    if let Some(var_93) = &input.value {
        object.key("Value").string(var_93.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_single_master_channel_endpoint_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SingleMasterChannelEndpointConfiguration) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_94) = &input.protocols {
        let mut array_95 = object.key("Protocols").start_array();
        for item_96 in var_94 {
             {
                array_95.value().string(item_96.as_str());
            }
        }
        array_95.finish();
    }
    if let Some(var_97) = &input.role {
        object.key("Role").string(var_97.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_channel_name_condition(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ChannelNameCondition) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_98) = &input.comparison_operator {
        object.key("ComparisonOperator").string(var_98.as_str());
    }
    if let Some(var_99) = &input.comparison_value {
        object.key("ComparisonValue").string(var_99.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_stream_name_condition(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StreamNameCondition) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_100) = &input.comparison_operator {
        object.key("ComparisonOperator").string(var_100.as_str());
    }
    if let Some(var_101) = &input.comparison_value {
        object.key("ComparisonValue").string(var_101.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_image_generation_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ImageGenerationConfiguration) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_102) = &input.status {
        object.key("Status").string(var_102.as_str());
    }
    if let Some(var_103) = &input.image_selector_type {
        object.key("ImageSelectorType").string(var_103.as_str());
    }
    if let Some(var_104) = &input.destination_config {
        let mut object_105 = object.key("DestinationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_image_generation_destination_config(&mut object_105, var_104)?;
        object_105.finish();
    }
    if let Some(var_106) = &input.sampling_interval {
        object.key("SamplingInterval").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_106).into()));
    }
    if let Some(var_107) = &input.format {
        object.key("Format").string(var_107.as_str());
    }
    if let Some(var_108) = &input.format_config {
        let mut object_109 = object.key("FormatConfig").start_object();
        for (key_110, value_111) in var_108 {
             {
                object_109.key(key_110.as_str()).string(value_111.as_str());
            }
        }
        object_109.finish();
    }
    if let Some(var_112) = &input.width_pixels {
        object.key("WidthPixels").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_112).into()));
    }
    if let Some(var_113) = &input.height_pixels {
        object.key("HeightPixels").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_113).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_notification_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NotificationConfiguration) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_114) = &input.status {
        object.key("Status").string(var_114.as_str());
    }
    if let Some(var_115) = &input.destination_config {
        let mut object_116 = object.key("DestinationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_notification_destination_config(&mut object_116, var_115)?;
        object_116.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_image_generation_destination_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ImageGenerationDestinationConfig) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_117) = &input.uri {
        object.key("Uri").string(var_117.as_str());
    }
    if let Some(var_118) = &input.destination_region {
        object.key("DestinationRegion").string(var_118.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_notification_destination_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NotificationDestinationConfig) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_119) = &input.uri {
        object.key("Uri").string(var_119.as_str());
    }
    Ok(())
}

