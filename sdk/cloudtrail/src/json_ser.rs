// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_add_tags_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AddTagsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.resource_id {
        object.key("ResourceId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tags_list {
        let mut array_3 = object.key("TagsList").start_array();
        for item_4 in var_2 {
             {
                let mut object_5 = array_3.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_cancel_query_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CancelQueryInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_6) = &input.event_data_store {
        object.key("EventDataStore").string(var_6.as_str());
    }
    if let Some(var_7) = &input.query_id {
        object.key("QueryId").string(var_7.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_event_data_store_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateEventDataStoreInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_8) = &input.name {
        object.key("Name").string(var_8.as_str());
    }
    if let Some(var_9) = &input.advanced_event_selectors {
        let mut array_10 = object.key("AdvancedEventSelectors").start_array();
        for item_11 in var_9 {
             {
                let mut object_12 = array_10.value().start_object();
                crate::json_ser::serialize_structure_crate_model_advanced_event_selector(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.multi_region_enabled {
        object.key("MultiRegionEnabled").boolean(*var_13);
    }
    if let Some(var_14) = &input.organization_enabled {
        object.key("OrganizationEnabled").boolean(*var_14);
    }
    if let Some(var_15) = &input.retention_period {
        object.key("RetentionPeriod").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_15).into()));
    }
    if let Some(var_16) = &input.termination_protection_enabled {
        object.key("TerminationProtectionEnabled").boolean(*var_16);
    }
    if let Some(var_17) = &input.tags_list {
        let mut array_18 = object.key("TagsList").start_array();
        for item_19 in var_17 {
             {
                let mut object_20 = array_18.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_20, item_19)?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_trail_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateTrailInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_21) = &input.name {
        object.key("Name").string(var_21.as_str());
    }
    if let Some(var_22) = &input.s3_bucket_name {
        object.key("S3BucketName").string(var_22.as_str());
    }
    if let Some(var_23) = &input.s3_key_prefix {
        object.key("S3KeyPrefix").string(var_23.as_str());
    }
    if let Some(var_24) = &input.sns_topic_name {
        object.key("SnsTopicName").string(var_24.as_str());
    }
    if let Some(var_25) = &input.include_global_service_events {
        object.key("IncludeGlobalServiceEvents").boolean(*var_25);
    }
    if let Some(var_26) = &input.is_multi_region_trail {
        object.key("IsMultiRegionTrail").boolean(*var_26);
    }
    if let Some(var_27) = &input.enable_log_file_validation {
        object.key("EnableLogFileValidation").boolean(*var_27);
    }
    if let Some(var_28) = &input.cloud_watch_logs_log_group_arn {
        object.key("CloudWatchLogsLogGroupArn").string(var_28.as_str());
    }
    if let Some(var_29) = &input.cloud_watch_logs_role_arn {
        object.key("CloudWatchLogsRoleArn").string(var_29.as_str());
    }
    if let Some(var_30) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_30.as_str());
    }
    if let Some(var_31) = &input.is_organization_trail {
        object.key("IsOrganizationTrail").boolean(*var_31);
    }
    if let Some(var_32) = &input.tags_list {
        let mut array_33 = object.key("TagsList").start_array();
        for item_34 in var_32 {
             {
                let mut object_35 = array_33.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_35, item_34)?;
                object_35.finish();
            }
        }
        array_33.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_event_data_store_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteEventDataStoreInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_36) = &input.event_data_store {
        object.key("EventDataStore").string(var_36.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_trail_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteTrailInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_37) = &input.name {
        object.key("Name").string(var_37.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_query_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeQueryInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_38) = &input.event_data_store {
        object.key("EventDataStore").string(var_38.as_str());
    }
    if let Some(var_39) = &input.query_id {
        object.key("QueryId").string(var_39.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_trails_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeTrailsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_40) = &input.trail_name_list {
        let mut array_41 = object.key("trailNameList").start_array();
        for item_42 in var_40 {
             {
                array_41.value().string(item_42.as_str());
            }
        }
        array_41.finish();
    }
    if let Some(var_43) = &input.include_shadow_trails {
        object.key("includeShadowTrails").boolean(*var_43);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_channel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetChannelInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.channel {
        object.key("Channel").string(var_44.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_event_data_store_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetEventDataStoreInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_45) = &input.event_data_store {
        object.key("EventDataStore").string(var_45.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_event_selectors_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetEventSelectorsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.trail_name {
        object.key("TrailName").string(var_46.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_import_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetImportInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_47) = &input.import_id {
        object.key("ImportId").string(var_47.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_insight_selectors_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetInsightSelectorsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_48) = &input.trail_name {
        object.key("TrailName").string(var_48.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_query_results_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetQueryResultsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_49) = &input.event_data_store {
        object.key("EventDataStore").string(var_49.as_str());
    }
    if let Some(var_50) = &input.query_id {
        object.key("QueryId").string(var_50.as_str());
    }
    if let Some(var_51) = &input.next_token {
        object.key("NextToken").string(var_51.as_str());
    }
    if let Some(var_52) = &input.max_query_results {
        object.key("MaxQueryResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_52).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_trail_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetTrailInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_53) = &input.name {
        object.key("Name").string(var_53.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_trail_status_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetTrailStatusInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_54) = &input.name {
        object.key("Name").string(var_54.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_channels_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListChannelsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_55) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_55).into()));
    }
    if let Some(var_56) = &input.next_token {
        object.key("NextToken").string(var_56.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_event_data_stores_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListEventDataStoresInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_57) = &input.next_token {
        object.key("NextToken").string(var_57.as_str());
    }
    if let Some(var_58) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_58).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_import_failures_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListImportFailuresInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_59) = &input.import_id {
        object.key("ImportId").string(var_59.as_str());
    }
    if let Some(var_60) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_60).into()));
    }
    if let Some(var_61) = &input.next_token {
        object.key("NextToken").string(var_61.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_imports_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListImportsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_62) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_62).into()));
    }
    if let Some(var_63) = &input.destination {
        object.key("Destination").string(var_63.as_str());
    }
    if let Some(var_64) = &input.import_status {
        object.key("ImportStatus").string(var_64.as_str());
    }
    if let Some(var_65) = &input.next_token {
        object.key("NextToken").string(var_65.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_public_keys_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListPublicKeysInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_66) = &input.start_time {
        object.key("StartTime").date_time(var_66, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_67) = &input.end_time {
        object.key("EndTime").date_time(var_67, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_68) = &input.next_token {
        object.key("NextToken").string(var_68.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_queries_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListQueriesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_69) = &input.event_data_store {
        object.key("EventDataStore").string(var_69.as_str());
    }
    if let Some(var_70) = &input.next_token {
        object.key("NextToken").string(var_70.as_str());
    }
    if let Some(var_71) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_71).into()));
    }
    if let Some(var_72) = &input.start_time {
        object.key("StartTime").date_time(var_72, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_73) = &input.end_time {
        object.key("EndTime").date_time(var_73, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_74) = &input.query_status {
        object.key("QueryStatus").string(var_74.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTagsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_75) = &input.resource_id_list {
        let mut array_76 = object.key("ResourceIdList").start_array();
        for item_77 in var_75 {
             {
                array_76.value().string(item_77.as_str());
            }
        }
        array_76.finish();
    }
    if let Some(var_78) = &input.next_token {
        object.key("NextToken").string(var_78.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_trails_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTrailsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.next_token {
        object.key("NextToken").string(var_79.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_lookup_events_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::LookupEventsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_80) = &input.lookup_attributes {
        let mut array_81 = object.key("LookupAttributes").start_array();
        for item_82 in var_80 {
             {
                let mut object_83 = array_81.value().start_object();
                crate::json_ser::serialize_structure_crate_model_lookup_attribute(&mut object_83, item_82)?;
                object_83.finish();
            }
        }
        array_81.finish();
    }
    if let Some(var_84) = &input.start_time {
        object.key("StartTime").date_time(var_84, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_85) = &input.end_time {
        object.key("EndTime").date_time(var_85, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_86) = &input.event_category {
        object.key("EventCategory").string(var_86.as_str());
    }
    if let Some(var_87) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_87).into()));
    }
    if let Some(var_88) = &input.next_token {
        object.key("NextToken").string(var_88.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_event_selectors_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutEventSelectorsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_89) = &input.trail_name {
        object.key("TrailName").string(var_89.as_str());
    }
    if let Some(var_90) = &input.event_selectors {
        let mut array_91 = object.key("EventSelectors").start_array();
        for item_92 in var_90 {
             {
                let mut object_93 = array_91.value().start_object();
                crate::json_ser::serialize_structure_crate_model_event_selector(&mut object_93, item_92)?;
                object_93.finish();
            }
        }
        array_91.finish();
    }
    if let Some(var_94) = &input.advanced_event_selectors {
        let mut array_95 = object.key("AdvancedEventSelectors").start_array();
        for item_96 in var_94 {
             {
                let mut object_97 = array_95.value().start_object();
                crate::json_ser::serialize_structure_crate_model_advanced_event_selector(&mut object_97, item_96)?;
                object_97.finish();
            }
        }
        array_95.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_insight_selectors_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutInsightSelectorsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_98) = &input.trail_name {
        object.key("TrailName").string(var_98.as_str());
    }
    if let Some(var_99) = &input.insight_selectors {
        let mut array_100 = object.key("InsightSelectors").start_array();
        for item_101 in var_99 {
             {
                let mut object_102 = array_100.value().start_object();
                crate::json_ser::serialize_structure_crate_model_insight_selector(&mut object_102, item_101)?;
                object_102.finish();
            }
        }
        array_100.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_remove_tags_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RemoveTagsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_103) = &input.resource_id {
        object.key("ResourceId").string(var_103.as_str());
    }
    if let Some(var_104) = &input.tags_list {
        let mut array_105 = object.key("TagsList").start_array();
        for item_106 in var_104 {
             {
                let mut object_107 = array_105.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_107, item_106)?;
                object_107.finish();
            }
        }
        array_105.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_restore_event_data_store_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RestoreEventDataStoreInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_108) = &input.event_data_store {
        object.key("EventDataStore").string(var_108.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_import_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartImportInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_109) = &input.destinations {
        let mut array_110 = object.key("Destinations").start_array();
        for item_111 in var_109 {
             {
                array_110.value().string(item_111.as_str());
            }
        }
        array_110.finish();
    }
    if let Some(var_112) = &input.import_source {
        let mut object_113 = object.key("ImportSource").start_object();
        crate::json_ser::serialize_structure_crate_model_import_source(&mut object_113, var_112)?;
        object_113.finish();
    }
    if let Some(var_114) = &input.start_event_time {
        object.key("StartEventTime").date_time(var_114, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_115) = &input.end_event_time {
        object.key("EndEventTime").date_time(var_115, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_116) = &input.import_id {
        object.key("ImportId").string(var_116.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_logging_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartLoggingInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_117) = &input.name {
        object.key("Name").string(var_117.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_query_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartQueryInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_118) = &input.query_statement {
        object.key("QueryStatement").string(var_118.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_import_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StopImportInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_119) = &input.import_id {
        object.key("ImportId").string(var_119.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_logging_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StopLoggingInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_120) = &input.name {
        object.key("Name").string(var_120.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_event_data_store_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateEventDataStoreInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_121) = &input.event_data_store {
        object.key("EventDataStore").string(var_121.as_str());
    }
    if let Some(var_122) = &input.name {
        object.key("Name").string(var_122.as_str());
    }
    if let Some(var_123) = &input.advanced_event_selectors {
        let mut array_124 = object.key("AdvancedEventSelectors").start_array();
        for item_125 in var_123 {
             {
                let mut object_126 = array_124.value().start_object();
                crate::json_ser::serialize_structure_crate_model_advanced_event_selector(&mut object_126, item_125)?;
                object_126.finish();
            }
        }
        array_124.finish();
    }
    if let Some(var_127) = &input.multi_region_enabled {
        object.key("MultiRegionEnabled").boolean(*var_127);
    }
    if let Some(var_128) = &input.organization_enabled {
        object.key("OrganizationEnabled").boolean(*var_128);
    }
    if let Some(var_129) = &input.retention_period {
        object.key("RetentionPeriod").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_129).into()));
    }
    if let Some(var_130) = &input.termination_protection_enabled {
        object.key("TerminationProtectionEnabled").boolean(*var_130);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_trail_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateTrailInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_131) = &input.name {
        object.key("Name").string(var_131.as_str());
    }
    if let Some(var_132) = &input.s3_bucket_name {
        object.key("S3BucketName").string(var_132.as_str());
    }
    if let Some(var_133) = &input.s3_key_prefix {
        object.key("S3KeyPrefix").string(var_133.as_str());
    }
    if let Some(var_134) = &input.sns_topic_name {
        object.key("SnsTopicName").string(var_134.as_str());
    }
    if let Some(var_135) = &input.include_global_service_events {
        object.key("IncludeGlobalServiceEvents").boolean(*var_135);
    }
    if let Some(var_136) = &input.is_multi_region_trail {
        object.key("IsMultiRegionTrail").boolean(*var_136);
    }
    if let Some(var_137) = &input.enable_log_file_validation {
        object.key("EnableLogFileValidation").boolean(*var_137);
    }
    if let Some(var_138) = &input.cloud_watch_logs_log_group_arn {
        object.key("CloudWatchLogsLogGroupArn").string(var_138.as_str());
    }
    if let Some(var_139) = &input.cloud_watch_logs_role_arn {
        object.key("CloudWatchLogsRoleArn").string(var_139.as_str());
    }
    if let Some(var_140) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_140.as_str());
    }
    if let Some(var_141) = &input.is_organization_trail {
        object.key("IsOrganizationTrail").boolean(*var_141);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Tag) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_142) = &input.key {
        object.key("Key").string(var_142.as_str());
    }
    if let Some(var_143) = &input.value {
        object.key("Value").string(var_143.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_advanced_event_selector(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AdvancedEventSelector) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_144) = &input.name {
        object.key("Name").string(var_144.as_str());
    }
    if let Some(var_145) = &input.field_selectors {
        let mut array_146 = object.key("FieldSelectors").start_array();
        for item_147 in var_145 {
             {
                let mut object_148 = array_146.value().start_object();
                crate::json_ser::serialize_structure_crate_model_advanced_field_selector(&mut object_148, item_147)?;
                object_148.finish();
            }
        }
        array_146.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_lookup_attribute(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LookupAttribute) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_149) = &input.attribute_key {
        object.key("AttributeKey").string(var_149.as_str());
    }
    if let Some(var_150) = &input.attribute_value {
        object.key("AttributeValue").string(var_150.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_event_selector(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EventSelector) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_151) = &input.read_write_type {
        object.key("ReadWriteType").string(var_151.as_str());
    }
    if let Some(var_152) = &input.include_management_events {
        object.key("IncludeManagementEvents").boolean(*var_152);
    }
    if let Some(var_153) = &input.data_resources {
        let mut array_154 = object.key("DataResources").start_array();
        for item_155 in var_153 {
             {
                let mut object_156 = array_154.value().start_object();
                crate::json_ser::serialize_structure_crate_model_data_resource(&mut object_156, item_155)?;
                object_156.finish();
            }
        }
        array_154.finish();
    }
    if let Some(var_157) = &input.exclude_management_event_sources {
        let mut array_158 = object.key("ExcludeManagementEventSources").start_array();
        for item_159 in var_157 {
             {
                array_158.value().string(item_159.as_str());
            }
        }
        array_158.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_insight_selector(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::InsightSelector) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_160) = &input.insight_type {
        object.key("InsightType").string(var_160.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_import_source(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ImportSource) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_161) = &input.s3 {
        let mut object_162 = object.key("S3").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_import_source(&mut object_162, var_161)?;
        object_162.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_advanced_field_selector(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AdvancedFieldSelector) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_163) = &input.field {
        object.key("Field").string(var_163.as_str());
    }
    if let Some(var_164) = &input.equals {
        let mut array_165 = object.key("Equals").start_array();
        for item_166 in var_164 {
             {
                array_165.value().string(item_166.as_str());
            }
        }
        array_165.finish();
    }
    if let Some(var_167) = &input.starts_with {
        let mut array_168 = object.key("StartsWith").start_array();
        for item_169 in var_167 {
             {
                array_168.value().string(item_169.as_str());
            }
        }
        array_168.finish();
    }
    if let Some(var_170) = &input.ends_with {
        let mut array_171 = object.key("EndsWith").start_array();
        for item_172 in var_170 {
             {
                array_171.value().string(item_172.as_str());
            }
        }
        array_171.finish();
    }
    if let Some(var_173) = &input.not_equals {
        let mut array_174 = object.key("NotEquals").start_array();
        for item_175 in var_173 {
             {
                array_174.value().string(item_175.as_str());
            }
        }
        array_174.finish();
    }
    if let Some(var_176) = &input.not_starts_with {
        let mut array_177 = object.key("NotStartsWith").start_array();
        for item_178 in var_176 {
             {
                array_177.value().string(item_178.as_str());
            }
        }
        array_177.finish();
    }
    if let Some(var_179) = &input.not_ends_with {
        let mut array_180 = object.key("NotEndsWith").start_array();
        for item_181 in var_179 {
             {
                array_180.value().string(item_181.as_str());
            }
        }
        array_180.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_data_resource(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DataResource) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_182) = &input.r#type {
        object.key("Type").string(var_182.as_str());
    }
    if let Some(var_183) = &input.values {
        let mut array_184 = object.key("Values").start_array();
        for item_185 in var_183 {
             {
                array_184.value().string(item_185.as_str());
            }
        }
        array_184.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_import_source(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::S3ImportSource) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_186) = &input.s3_location_uri {
        object.key("S3LocationUri").string(var_186.as_str());
    }
    if let Some(var_187) = &input.s3_bucket_region {
        object.key("S3BucketRegion").string(var_187.as_str());
    }
    if let Some(var_188) = &input.s3_bucket_access_role_arn {
        object.key("S3BucketAccessRoleArn").string(var_188.as_str());
    }
    Ok(())
}

