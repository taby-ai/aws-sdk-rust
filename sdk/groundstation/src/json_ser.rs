// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_config_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateConfigInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.config_data {
        let mut object_2 = object.key("configData").start_object();
        crate::json_ser::serialize_union_crate_model_config_type_data(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.name {
        object.key("name").string(var_3.as_str());
    }
    if let Some(var_4) = &input.tags {
        let mut object_5 = object.key("tags").start_object();
        for (key_6, value_7) in var_4 {
             {
                object_5.key(key_6).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_dataflow_endpoint_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateDataflowEndpointGroupInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_8) = &input.endpoint_details {
        let mut array_9 = object.key("endpointDetails").start_array();
        for item_10 in var_8 {
             {
                let mut object_11 = array_9.value().start_object();
                crate::json_ser::serialize_structure_crate_model_endpoint_details(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.tags {
        let mut object_13 = object.key("tags").start_object();
        for (key_14, value_15) in var_12 {
             {
                object_13.key(key_14).string(value_15.as_str());
            }
        }
        object_13.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_mission_profile_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateMissionProfileInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_16) = &input.contact_post_pass_duration_seconds {
        object.key("contactPostPassDurationSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_16).into()));
    }
    if let Some(var_17) = &input.contact_pre_pass_duration_seconds {
        object.key("contactPrePassDurationSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_17).into()));
    }
    if let Some(var_18) = &input.dataflow_edges {
        let mut array_19 = object.key("dataflowEdges").start_array();
        for item_20 in var_18 {
             {
                let mut array_21 = array_19.value().start_array();
                for item_22 in item_20 {
                     {
                        array_21.value().string(item_22.as_str());
                    }
                }
                array_21.finish();
            }
        }
        array_19.finish();
    }
    if let Some(var_23) = &input.minimum_viable_contact_duration_seconds {
        object.key("minimumViableContactDurationSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_23).into()));
    }
    if let Some(var_24) = &input.name {
        object.key("name").string(var_24.as_str());
    }
    if let Some(var_25) = &input.tags {
        let mut object_26 = object.key("tags").start_object();
        for (key_27, value_28) in var_25 {
             {
                object_26.key(key_27).string(value_28.as_str());
            }
        }
        object_26.finish();
    }
    if let Some(var_29) = &input.tracking_config_arn {
        object.key("trackingConfigArn").string(var_29.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_minute_usage_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetMinuteUsageInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_30) = &input.month {
        object.key("month").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_30).into()));
    }
    if let Some(var_31) = &input.year {
        object.key("year").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_31).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_contacts_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListContactsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_32) = &input.end_time {
        object.key("endTime").date_time(var_32, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_33) = &input.ground_station {
        object.key("groundStation").string(var_33.as_str());
    }
    if let Some(var_34) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_34).into()));
    }
    if let Some(var_35) = &input.mission_profile_arn {
        object.key("missionProfileArn").string(var_35.as_str());
    }
    if let Some(var_36) = &input.next_token {
        object.key("nextToken").string(var_36.as_str());
    }
    if let Some(var_37) = &input.satellite_arn {
        object.key("satelliteArn").string(var_37.as_str());
    }
    if let Some(var_38) = &input.start_time {
        object.key("startTime").date_time(var_38, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_39) = &input.status_list {
        let mut array_40 = object.key("statusList").start_array();
        for item_41 in var_39 {
             {
                array_40.value().string(item_41.as_str());
            }
        }
        array_40.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_reserve_contact_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ReserveContactInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_42) = &input.end_time {
        object.key("endTime").date_time(var_42, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_43) = &input.ground_station {
        object.key("groundStation").string(var_43.as_str());
    }
    if let Some(var_44) = &input.mission_profile_arn {
        object.key("missionProfileArn").string(var_44.as_str());
    }
    if let Some(var_45) = &input.satellite_arn {
        object.key("satelliteArn").string(var_45.as_str());
    }
    if let Some(var_46) = &input.start_time {
        object.key("startTime").date_time(var_46, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_47) = &input.tags {
        let mut object_48 = object.key("tags").start_object();
        for (key_49, value_50) in var_47 {
             {
                object_48.key(key_49).string(value_50.as_str());
            }
        }
        object_48.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_51) = &input.tags {
        let mut object_52 = object.key("tags").start_object();
        for (key_53, value_54) in var_51 {
             {
                object_52.key(key_53).string(value_54.as_str());
            }
        }
        object_52.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_config_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateConfigInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_55) = &input.config_data {
        let mut object_56 = object.key("configData").start_object();
        crate::json_ser::serialize_union_crate_model_config_type_data(&mut object_56, var_55)?;
        object_56.finish();
    }
    if let Some(var_57) = &input.name {
        object.key("name").string(var_57.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_mission_profile_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateMissionProfileInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_58) = &input.contact_post_pass_duration_seconds {
        object.key("contactPostPassDurationSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_58).into()));
    }
    if let Some(var_59) = &input.contact_pre_pass_duration_seconds {
        object.key("contactPrePassDurationSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_59).into()));
    }
    if let Some(var_60) = &input.dataflow_edges {
        let mut array_61 = object.key("dataflowEdges").start_array();
        for item_62 in var_60 {
             {
                let mut array_63 = array_61.value().start_array();
                for item_64 in item_62 {
                     {
                        array_63.value().string(item_64.as_str());
                    }
                }
                array_63.finish();
            }
        }
        array_61.finish();
    }
    if let Some(var_65) = &input.minimum_viable_contact_duration_seconds {
        object.key("minimumViableContactDurationSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_65).into()));
    }
    if let Some(var_66) = &input.name {
        object.key("name").string(var_66.as_str());
    }
    if let Some(var_67) = &input.tracking_config_arn {
        object.key("trackingConfigArn").string(var_67.as_str());
    }
    Ok(())
}

pub fn serialize_union_crate_model_config_type_data(object_2: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ConfigTypeData) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::ConfigTypeData::AntennaDownlinkConfig(inner) => {
             {
                let mut object_68 = object_2.key("antennaDownlinkConfig").start_object();
                crate::json_ser::serialize_structure_crate_model_antenna_downlink_config(&mut object_68, inner)?;
                object_68.finish();
            }
        },
        crate::model::ConfigTypeData::TrackingConfig(inner) => {
             {
                let mut object_69 = object_2.key("trackingConfig").start_object();
                crate::json_ser::serialize_structure_crate_model_tracking_config(&mut object_69, inner)?;
                object_69.finish();
            }
        },
        crate::model::ConfigTypeData::DataflowEndpointConfig(inner) => {
             {
                let mut object_70 = object_2.key("dataflowEndpointConfig").start_object();
                crate::json_ser::serialize_structure_crate_model_dataflow_endpoint_config(&mut object_70, inner)?;
                object_70.finish();
            }
        },
        crate::model::ConfigTypeData::AntennaDownlinkDemodDecodeConfig(inner) => {
             {
                let mut object_71 = object_2.key("antennaDownlinkDemodDecodeConfig").start_object();
                crate::json_ser::serialize_structure_crate_model_antenna_downlink_demod_decode_config(&mut object_71, inner)?;
                object_71.finish();
            }
        },
        crate::model::ConfigTypeData::AntennaUplinkConfig(inner) => {
             {
                let mut object_72 = object_2.key("antennaUplinkConfig").start_object();
                crate::json_ser::serialize_structure_crate_model_antenna_uplink_config(&mut object_72, inner)?;
                object_72.finish();
            }
        },
        crate::model::ConfigTypeData::UplinkEchoConfig(inner) => {
             {
                let mut object_73 = object_2.key("uplinkEchoConfig").start_object();
                crate::json_ser::serialize_structure_crate_model_uplink_echo_config(&mut object_73, inner)?;
                object_73.finish();
            }
        },
        crate::model::ConfigTypeData::S3RecordingConfig(inner) => {
             {
                let mut object_74 = object_2.key("s3RecordingConfig").start_object();
                crate::json_ser::serialize_structure_crate_model_s3_recording_config(&mut object_74, inner)?;
                object_74.finish();
            }
        },
        crate::model::ConfigTypeData::Unknown => return Err(aws_smithy_http::operation::SerializationError::unknown_variant("ConfigTypeData"))
    }
    Ok(())
}

pub fn serialize_structure_crate_model_endpoint_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EndpointDetails) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_75) = &input.security_details {
        let mut object_76 = object.key("securityDetails").start_object();
        crate::json_ser::serialize_structure_crate_model_security_details(&mut object_76, var_75)?;
        object_76.finish();
    }
    if let Some(var_77) = &input.endpoint {
        let mut object_78 = object.key("endpoint").start_object();
        crate::json_ser::serialize_structure_crate_model_dataflow_endpoint(&mut object_78, var_77)?;
        object_78.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_antenna_downlink_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AntennaDownlinkConfig) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.spectrum_config {
        let mut object_80 = object.key("spectrumConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_spectrum_config(&mut object_80, var_79)?;
        object_80.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tracking_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TrackingConfig) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_81) = &input.autotrack {
        object.key("autotrack").string(var_81.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dataflow_endpoint_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DataflowEndpointConfig) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_82) = &input.dataflow_endpoint_name {
        object.key("dataflowEndpointName").string(var_82.as_str());
    }
    if let Some(var_83) = &input.dataflow_endpoint_region {
        object.key("dataflowEndpointRegion").string(var_83.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_antenna_downlink_demod_decode_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AntennaDownlinkDemodDecodeConfig) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_84) = &input.spectrum_config {
        let mut object_85 = object.key("spectrumConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_spectrum_config(&mut object_85, var_84)?;
        object_85.finish();
    }
    if let Some(var_86) = &input.demodulation_config {
        let mut object_87 = object.key("demodulationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_demodulation_config(&mut object_87, var_86)?;
        object_87.finish();
    }
    if let Some(var_88) = &input.decode_config {
        let mut object_89 = object.key("decodeConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_decode_config(&mut object_89, var_88)?;
        object_89.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_antenna_uplink_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AntennaUplinkConfig) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_90) = &input.transmit_disabled {
        object.key("transmitDisabled").boolean(*var_90);
    }
    if let Some(var_91) = &input.spectrum_config {
        let mut object_92 = object.key("spectrumConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_uplink_spectrum_config(&mut object_92, var_91)?;
        object_92.finish();
    }
    if let Some(var_93) = &input.target_eirp {
        let mut object_94 = object.key("targetEirp").start_object();
        crate::json_ser::serialize_structure_crate_model_eirp(&mut object_94, var_93)?;
        object_94.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_uplink_echo_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::UplinkEchoConfig) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_95) = &input.enabled {
        object.key("enabled").boolean(*var_95);
    }
    if let Some(var_96) = &input.antenna_uplink_config_arn {
        object.key("antennaUplinkConfigArn").string(var_96.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_recording_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::S3RecordingConfig) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_97) = &input.bucket_arn {
        object.key("bucketArn").string(var_97.as_str());
    }
    if let Some(var_98) = &input.role_arn {
        object.key("roleArn").string(var_98.as_str());
    }
    if let Some(var_99) = &input.prefix {
        object.key("prefix").string(var_99.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_security_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SecurityDetails) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_100) = &input.subnet_ids {
        let mut array_101 = object.key("subnetIds").start_array();
        for item_102 in var_100 {
             {
                array_101.value().string(item_102.as_str());
            }
        }
        array_101.finish();
    }
    if let Some(var_103) = &input.security_group_ids {
        let mut array_104 = object.key("securityGroupIds").start_array();
        for item_105 in var_103 {
             {
                array_104.value().string(item_105.as_str());
            }
        }
        array_104.finish();
    }
    if let Some(var_106) = &input.role_arn {
        object.key("roleArn").string(var_106.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dataflow_endpoint(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DataflowEndpoint) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_107) = &input.name {
        object.key("name").string(var_107.as_str());
    }
    if let Some(var_108) = &input.address {
        let mut object_109 = object.key("address").start_object();
        crate::json_ser::serialize_structure_crate_model_socket_address(&mut object_109, var_108)?;
        object_109.finish();
    }
    if let Some(var_110) = &input.status {
        object.key("status").string(var_110.as_str());
    }
    if let Some(var_111) = &input.mtu {
        object.key("mtu").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_111).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_spectrum_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SpectrumConfig) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_112) = &input.center_frequency {
        let mut object_113 = object.key("centerFrequency").start_object();
        crate::json_ser::serialize_structure_crate_model_frequency(&mut object_113, var_112)?;
        object_113.finish();
    }
    if let Some(var_114) = &input.bandwidth {
        let mut object_115 = object.key("bandwidth").start_object();
        crate::json_ser::serialize_structure_crate_model_frequency_bandwidth(&mut object_115, var_114)?;
        object_115.finish();
    }
    if let Some(var_116) = &input.polarization {
        object.key("polarization").string(var_116.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_demodulation_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DemodulationConfig) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_117) = &input.unvalidated_json {
        object.key("unvalidatedJSON").string(var_117.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_decode_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DecodeConfig) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_118) = &input.unvalidated_json {
        object.key("unvalidatedJSON").string(var_118.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_uplink_spectrum_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::UplinkSpectrumConfig) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_119) = &input.center_frequency {
        let mut object_120 = object.key("centerFrequency").start_object();
        crate::json_ser::serialize_structure_crate_model_frequency(&mut object_120, var_119)?;
        object_120.finish();
    }
    if let Some(var_121) = &input.polarization {
        object.key("polarization").string(var_121.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_eirp(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Eirp) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_122) = &input.value {
        object.key("value").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_122).into()));
    }
    if let Some(var_123) = &input.units {
        object.key("units").string(var_123.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_socket_address(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SocketAddress) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_124) = &input.name {
        object.key("name").string(var_124.as_str());
    }
    if let Some(var_125) = &input.port {
        object.key("port").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_125).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_frequency(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Frequency) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_126) = &input.value {
        object.key("value").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_126).into()));
    }
    if let Some(var_127) = &input.units {
        object.key("units").string(var_127.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_frequency_bandwidth(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::FrequencyBandwidth) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_128) = &input.value {
        object.key("value").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_128).into()));
    }
    if let Some(var_129) = &input.units {
        object.key("units").string(var_129.as_str());
    }
    Ok(())
}

