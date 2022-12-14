// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_get_clip_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetClipInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.clip_fragment_selector {
        let mut object_2 = object.key("ClipFragmentSelector").start_object();
        crate::json_ser::serialize_structure_crate_model_clip_fragment_selector(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.stream_arn {
        object.key("StreamARN").string(var_3.as_str());
    }
    if let Some(var_4) = &input.stream_name {
        object.key("StreamName").string(var_4.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_dash_streaming_session_url_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetDashStreamingSessionUrlInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_5) = &input.dash_fragment_selector {
        let mut object_6 = object.key("DASHFragmentSelector").start_object();
        crate::json_ser::serialize_structure_crate_model_dash_fragment_selector(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.display_fragment_number {
        object.key("DisplayFragmentNumber").string(var_7.as_str());
    }
    if let Some(var_8) = &input.display_fragment_timestamp {
        object.key("DisplayFragmentTimestamp").string(var_8.as_str());
    }
    if let Some(var_9) = &input.expires {
        object.key("Expires").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_9).into()));
    }
    if let Some(var_10) = &input.max_manifest_fragment_results {
        object.key("MaxManifestFragmentResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_10).into()));
    }
    if let Some(var_11) = &input.playback_mode {
        object.key("PlaybackMode").string(var_11.as_str());
    }
    if let Some(var_12) = &input.stream_arn {
        object.key("StreamARN").string(var_12.as_str());
    }
    if let Some(var_13) = &input.stream_name {
        object.key("StreamName").string(var_13.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_hls_streaming_session_url_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetHlsStreamingSessionUrlInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_14) = &input.container_format {
        object.key("ContainerFormat").string(var_14.as_str());
    }
    if let Some(var_15) = &input.discontinuity_mode {
        object.key("DiscontinuityMode").string(var_15.as_str());
    }
    if let Some(var_16) = &input.display_fragment_timestamp {
        object.key("DisplayFragmentTimestamp").string(var_16.as_str());
    }
    if let Some(var_17) = &input.expires {
        object.key("Expires").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_17).into()));
    }
    if let Some(var_18) = &input.hls_fragment_selector {
        let mut object_19 = object.key("HLSFragmentSelector").start_object();
        crate::json_ser::serialize_structure_crate_model_hls_fragment_selector(&mut object_19, var_18)?;
        object_19.finish();
    }
    if let Some(var_20) = &input.max_media_playlist_fragment_results {
        object.key("MaxMediaPlaylistFragmentResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_20).into()));
    }
    if let Some(var_21) = &input.playback_mode {
        object.key("PlaybackMode").string(var_21.as_str());
    }
    if let Some(var_22) = &input.stream_arn {
        object.key("StreamARN").string(var_22.as_str());
    }
    if let Some(var_23) = &input.stream_name {
        object.key("StreamName").string(var_23.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_images_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetImagesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_24) = &input.end_timestamp {
        object.key("EndTimestamp").date_time(var_24, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_25) = &input.format {
        object.key("Format").string(var_25.as_str());
    }
    if let Some(var_26) = &input.format_config {
        let mut object_27 = object.key("FormatConfig").start_object();
        for (key_28, value_29) in var_26 {
             {
                object_27.key(key_28.as_str()).string(value_29.as_str());
            }
        }
        object_27.finish();
    }
    if let Some(var_30) = &input.height_pixels {
        object.key("HeightPixels").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_30).into()));
    }
    if let Some(var_31) = &input.image_selector_type {
        object.key("ImageSelectorType").string(var_31.as_str());
    }
    if let Some(var_32) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_32).into()));
    }
    if let Some(var_33) = &input.next_token {
        object.key("NextToken").string(var_33.as_str());
    }
    if let Some(var_34) = &input.sampling_interval {
        object.key("SamplingInterval").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_34).into()));
    }
    if let Some(var_35) = &input.start_timestamp {
        object.key("StartTimestamp").date_time(var_35, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_36) = &input.stream_arn {
        object.key("StreamARN").string(var_36.as_str());
    }
    if let Some(var_37) = &input.stream_name {
        object.key("StreamName").string(var_37.as_str());
    }
    if let Some(var_38) = &input.width_pixels {
        object.key("WidthPixels").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_38).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_media_for_fragment_list_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetMediaForFragmentListInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_39) = &input.fragments {
        let mut array_40 = object.key("Fragments").start_array();
        for item_41 in var_39 {
             {
                array_40.value().string(item_41.as_str());
            }
        }
        array_40.finish();
    }
    if let Some(var_42) = &input.stream_arn {
        object.key("StreamARN").string(var_42.as_str());
    }
    if let Some(var_43) = &input.stream_name {
        object.key("StreamName").string(var_43.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_fragments_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListFragmentsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.fragment_selector {
        let mut object_45 = object.key("FragmentSelector").start_object();
        crate::json_ser::serialize_structure_crate_model_fragment_selector(&mut object_45, var_44)?;
        object_45.finish();
    }
    if let Some(var_46) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_46).into()));
    }
    if let Some(var_47) = &input.next_token {
        object.key("NextToken").string(var_47.as_str());
    }
    if let Some(var_48) = &input.stream_arn {
        object.key("StreamARN").string(var_48.as_str());
    }
    if let Some(var_49) = &input.stream_name {
        object.key("StreamName").string(var_49.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_clip_fragment_selector(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ClipFragmentSelector) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_50) = &input.fragment_selector_type {
        object.key("FragmentSelectorType").string(var_50.as_str());
    }
    if let Some(var_51) = &input.timestamp_range {
        let mut object_52 = object.key("TimestampRange").start_object();
        crate::json_ser::serialize_structure_crate_model_clip_timestamp_range(&mut object_52, var_51)?;
        object_52.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dash_fragment_selector(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DashFragmentSelector) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_53) = &input.fragment_selector_type {
        object.key("FragmentSelectorType").string(var_53.as_str());
    }
    if let Some(var_54) = &input.timestamp_range {
        let mut object_55 = object.key("TimestampRange").start_object();
        crate::json_ser::serialize_structure_crate_model_dash_timestamp_range(&mut object_55, var_54)?;
        object_55.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_hls_fragment_selector(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::HlsFragmentSelector) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_56) = &input.fragment_selector_type {
        object.key("FragmentSelectorType").string(var_56.as_str());
    }
    if let Some(var_57) = &input.timestamp_range {
        let mut object_58 = object.key("TimestampRange").start_object();
        crate::json_ser::serialize_structure_crate_model_hls_timestamp_range(&mut object_58, var_57)?;
        object_58.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_fragment_selector(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::FragmentSelector) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_59) = &input.fragment_selector_type {
        object.key("FragmentSelectorType").string(var_59.as_str());
    }
    if let Some(var_60) = &input.timestamp_range {
        let mut object_61 = object.key("TimestampRange").start_object();
        crate::json_ser::serialize_structure_crate_model_timestamp_range(&mut object_61, var_60)?;
        object_61.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_clip_timestamp_range(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ClipTimestampRange) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_62) = &input.start_timestamp {
        object.key("StartTimestamp").date_time(var_62, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_63) = &input.end_timestamp {
        object.key("EndTimestamp").date_time(var_63, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dash_timestamp_range(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DashTimestampRange) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_64) = &input.start_timestamp {
        object.key("StartTimestamp").date_time(var_64, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_65) = &input.end_timestamp {
        object.key("EndTimestamp").date_time(var_65, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_model_hls_timestamp_range(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::HlsTimestampRange) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_66) = &input.start_timestamp {
        object.key("StartTimestamp").date_time(var_66, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_67) = &input.end_timestamp {
        object.key("EndTimestamp").date_time(var_67, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_model_timestamp_range(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TimestampRange) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_68) = &input.start_timestamp {
        object.key("StartTimestamp").date_time(var_68, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_69) = &input.end_timestamp {
        object.key("EndTimestamp").date_time(var_69, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

