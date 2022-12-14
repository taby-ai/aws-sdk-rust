// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_kms_key_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AssociateKmsKeyInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.log_group_name {
        object.key("logGroupName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.kms_key_id {
        object.key("kmsKeyId").string(var_2.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_cancel_export_task_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CancelExportTaskInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_3) = &input.task_id {
        object.key("taskId").string(var_3.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_export_task_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateExportTaskInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_4) = &input.task_name {
        object.key("taskName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.log_group_name {
        object.key("logGroupName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.log_stream_name_prefix {
        object.key("logStreamNamePrefix").string(var_6.as_str());
    }
    if let Some(var_7) = &input.from {
        object.key("from").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_7).into()));
    }
    if let Some(var_8) = &input.to {
        object.key("to").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_8).into()));
    }
    if let Some(var_9) = &input.destination {
        object.key("destination").string(var_9.as_str());
    }
    if let Some(var_10) = &input.destination_prefix {
        object.key("destinationPrefix").string(var_10.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_log_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateLogGroupInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_11) = &input.log_group_name {
        object.key("logGroupName").string(var_11.as_str());
    }
    if let Some(var_12) = &input.kms_key_id {
        object.key("kmsKeyId").string(var_12.as_str());
    }
    if let Some(var_13) = &input.tags {
        let mut object_14 = object.key("tags").start_object();
        for (key_15, value_16) in var_13 {
             {
                object_14.key(key_15).string(value_16.as_str());
            }
        }
        object_14.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_log_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateLogStreamInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_17) = &input.log_group_name {
        object.key("logGroupName").string(var_17.as_str());
    }
    if let Some(var_18) = &input.log_stream_name {
        object.key("logStreamName").string(var_18.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_destination_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteDestinationInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_19) = &input.destination_name {
        object.key("destinationName").string(var_19.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_log_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteLogGroupInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_20) = &input.log_group_name {
        object.key("logGroupName").string(var_20.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_log_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteLogStreamInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_21) = &input.log_group_name {
        object.key("logGroupName").string(var_21.as_str());
    }
    if let Some(var_22) = &input.log_stream_name {
        object.key("logStreamName").string(var_22.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_metric_filter_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteMetricFilterInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_23) = &input.log_group_name {
        object.key("logGroupName").string(var_23.as_str());
    }
    if let Some(var_24) = &input.filter_name {
        object.key("filterName").string(var_24.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_query_definition_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteQueryDefinitionInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_25) = &input.query_definition_id {
        object.key("queryDefinitionId").string(var_25.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_resource_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteResourcePolicyInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_26) = &input.policy_name {
        object.key("policyName").string(var_26.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_retention_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteRetentionPolicyInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_27) = &input.log_group_name {
        object.key("logGroupName").string(var_27.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_subscription_filter_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteSubscriptionFilterInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.log_group_name {
        object.key("logGroupName").string(var_28.as_str());
    }
    if let Some(var_29) = &input.filter_name {
        object.key("filterName").string(var_29.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_destinations_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeDestinationsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_30) = &input.destination_name_prefix {
        object.key("DestinationNamePrefix").string(var_30.as_str());
    }
    if let Some(var_31) = &input.next_token {
        object.key("nextToken").string(var_31.as_str());
    }
    if let Some(var_32) = &input.limit {
        object.key("limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_32).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_export_tasks_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeExportTasksInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.task_id {
        object.key("taskId").string(var_33.as_str());
    }
    if let Some(var_34) = &input.status_code {
        object.key("statusCode").string(var_34.as_str());
    }
    if let Some(var_35) = &input.next_token {
        object.key("nextToken").string(var_35.as_str());
    }
    if let Some(var_36) = &input.limit {
        object.key("limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_36).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_log_groups_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeLogGroupsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_37) = &input.log_group_name_prefix {
        object.key("logGroupNamePrefix").string(var_37.as_str());
    }
    if let Some(var_38) = &input.next_token {
        object.key("nextToken").string(var_38.as_str());
    }
    if let Some(var_39) = &input.limit {
        object.key("limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_39).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_log_streams_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeLogStreamsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_40) = &input.log_group_name {
        object.key("logGroupName").string(var_40.as_str());
    }
    if let Some(var_41) = &input.log_stream_name_prefix {
        object.key("logStreamNamePrefix").string(var_41.as_str());
    }
    if let Some(var_42) = &input.order_by {
        object.key("orderBy").string(var_42.as_str());
    }
    if let Some(var_43) = &input.descending {
        object.key("descending").boolean(*var_43);
    }
    if let Some(var_44) = &input.next_token {
        object.key("nextToken").string(var_44.as_str());
    }
    if let Some(var_45) = &input.limit {
        object.key("limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_45).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_metric_filters_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeMetricFiltersInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.log_group_name {
        object.key("logGroupName").string(var_46.as_str());
    }
    if let Some(var_47) = &input.filter_name_prefix {
        object.key("filterNamePrefix").string(var_47.as_str());
    }
    if let Some(var_48) = &input.next_token {
        object.key("nextToken").string(var_48.as_str());
    }
    if let Some(var_49) = &input.limit {
        object.key("limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_49).into()));
    }
    if let Some(var_50) = &input.metric_name {
        object.key("metricName").string(var_50.as_str());
    }
    if let Some(var_51) = &input.metric_namespace {
        object.key("metricNamespace").string(var_51.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_queries_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeQueriesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_52) = &input.log_group_name {
        object.key("logGroupName").string(var_52.as_str());
    }
    if let Some(var_53) = &input.status {
        object.key("status").string(var_53.as_str());
    }
    if let Some(var_54) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_54).into()));
    }
    if let Some(var_55) = &input.next_token {
        object.key("nextToken").string(var_55.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_query_definitions_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeQueryDefinitionsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_56) = &input.query_definition_name_prefix {
        object.key("queryDefinitionNamePrefix").string(var_56.as_str());
    }
    if let Some(var_57) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_57).into()));
    }
    if let Some(var_58) = &input.next_token {
        object.key("nextToken").string(var_58.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_resource_policies_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeResourcePoliciesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_59) = &input.next_token {
        object.key("nextToken").string(var_59.as_str());
    }
    if let Some(var_60) = &input.limit {
        object.key("limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_60).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_subscription_filters_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeSubscriptionFiltersInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_61) = &input.log_group_name {
        object.key("logGroupName").string(var_61.as_str());
    }
    if let Some(var_62) = &input.filter_name_prefix {
        object.key("filterNamePrefix").string(var_62.as_str());
    }
    if let Some(var_63) = &input.next_token {
        object.key("nextToken").string(var_63.as_str());
    }
    if let Some(var_64) = &input.limit {
        object.key("limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_64).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disassociate_kms_key_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DisassociateKmsKeyInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_65) = &input.log_group_name {
        object.key("logGroupName").string(var_65.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_filter_log_events_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::FilterLogEventsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_66) = &input.log_group_name {
        object.key("logGroupName").string(var_66.as_str());
    }
    if let Some(var_67) = &input.log_stream_names {
        let mut array_68 = object.key("logStreamNames").start_array();
        for item_69 in var_67 {
             {
                array_68.value().string(item_69.as_str());
            }
        }
        array_68.finish();
    }
    if let Some(var_70) = &input.log_stream_name_prefix {
        object.key("logStreamNamePrefix").string(var_70.as_str());
    }
    if let Some(var_71) = &input.start_time {
        object.key("startTime").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_71).into()));
    }
    if let Some(var_72) = &input.end_time {
        object.key("endTime").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_72).into()));
    }
    if let Some(var_73) = &input.filter_pattern {
        object.key("filterPattern").string(var_73.as_str());
    }
    if let Some(var_74) = &input.next_token {
        object.key("nextToken").string(var_74.as_str());
    }
    if let Some(var_75) = &input.limit {
        object.key("limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_75).into()));
    }
    if let Some(var_76) = &input.interleaved {
        object.key("interleaved").boolean(*var_76);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_log_events_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetLogEventsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_77) = &input.log_group_name {
        object.key("logGroupName").string(var_77.as_str());
    }
    if let Some(var_78) = &input.log_stream_name {
        object.key("logStreamName").string(var_78.as_str());
    }
    if let Some(var_79) = &input.start_time {
        object.key("startTime").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_79).into()));
    }
    if let Some(var_80) = &input.end_time {
        object.key("endTime").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_80).into()));
    }
    if let Some(var_81) = &input.next_token {
        object.key("nextToken").string(var_81.as_str());
    }
    if let Some(var_82) = &input.limit {
        object.key("limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_82).into()));
    }
    if let Some(var_83) = &input.start_from_head {
        object.key("startFromHead").boolean(*var_83);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_log_group_fields_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetLogGroupFieldsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_84) = &input.log_group_name {
        object.key("logGroupName").string(var_84.as_str());
    }
    if let Some(var_85) = &input.time {
        object.key("time").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_85).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_log_record_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetLogRecordInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_86) = &input.log_record_pointer {
        object.key("logRecordPointer").string(var_86.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_query_results_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetQueryResultsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_87) = &input.query_id {
        object.key("queryId").string(var_87.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_log_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTagsLogGroupInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_88) = &input.log_group_name {
        object.key("logGroupName").string(var_88.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_destination_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutDestinationInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_89) = &input.destination_name {
        object.key("destinationName").string(var_89.as_str());
    }
    if let Some(var_90) = &input.target_arn {
        object.key("targetArn").string(var_90.as_str());
    }
    if let Some(var_91) = &input.role_arn {
        object.key("roleArn").string(var_91.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_destination_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutDestinationPolicyInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_92) = &input.destination_name {
        object.key("destinationName").string(var_92.as_str());
    }
    if let Some(var_93) = &input.access_policy {
        object.key("accessPolicy").string(var_93.as_str());
    }
    if let Some(var_94) = &input.force_update {
        object.key("forceUpdate").boolean(*var_94);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_log_events_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutLogEventsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_95) = &input.log_group_name {
        object.key("logGroupName").string(var_95.as_str());
    }
    if let Some(var_96) = &input.log_stream_name {
        object.key("logStreamName").string(var_96.as_str());
    }
    if let Some(var_97) = &input.log_events {
        let mut array_98 = object.key("logEvents").start_array();
        for item_99 in var_97 {
             {
                let mut object_100 = array_98.value().start_object();
                crate::json_ser::serialize_structure_crate_model_input_log_event(&mut object_100, item_99)?;
                object_100.finish();
            }
        }
        array_98.finish();
    }
    if let Some(var_101) = &input.sequence_token {
        object.key("sequenceToken").string(var_101.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_metric_filter_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutMetricFilterInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_102) = &input.log_group_name {
        object.key("logGroupName").string(var_102.as_str());
    }
    if let Some(var_103) = &input.filter_name {
        object.key("filterName").string(var_103.as_str());
    }
    if let Some(var_104) = &input.filter_pattern {
        object.key("filterPattern").string(var_104.as_str());
    }
    if let Some(var_105) = &input.metric_transformations {
        let mut array_106 = object.key("metricTransformations").start_array();
        for item_107 in var_105 {
             {
                let mut object_108 = array_106.value().start_object();
                crate::json_ser::serialize_structure_crate_model_metric_transformation(&mut object_108, item_107)?;
                object_108.finish();
            }
        }
        array_106.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_query_definition_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutQueryDefinitionInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_109) = &input.name {
        object.key("name").string(var_109.as_str());
    }
    if let Some(var_110) = &input.query_definition_id {
        object.key("queryDefinitionId").string(var_110.as_str());
    }
    if let Some(var_111) = &input.log_group_names {
        let mut array_112 = object.key("logGroupNames").start_array();
        for item_113 in var_111 {
             {
                array_112.value().string(item_113.as_str());
            }
        }
        array_112.finish();
    }
    if let Some(var_114) = &input.query_string {
        object.key("queryString").string(var_114.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_resource_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutResourcePolicyInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_115) = &input.policy_name {
        object.key("policyName").string(var_115.as_str());
    }
    if let Some(var_116) = &input.policy_document {
        object.key("policyDocument").string(var_116.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_retention_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutRetentionPolicyInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_117) = &input.log_group_name {
        object.key("logGroupName").string(var_117.as_str());
    }
    if let Some(var_118) = &input.retention_in_days {
        object.key("retentionInDays").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_118).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_subscription_filter_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutSubscriptionFilterInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_119) = &input.log_group_name {
        object.key("logGroupName").string(var_119.as_str());
    }
    if let Some(var_120) = &input.filter_name {
        object.key("filterName").string(var_120.as_str());
    }
    if let Some(var_121) = &input.filter_pattern {
        object.key("filterPattern").string(var_121.as_str());
    }
    if let Some(var_122) = &input.destination_arn {
        object.key("destinationArn").string(var_122.as_str());
    }
    if let Some(var_123) = &input.role_arn {
        object.key("roleArn").string(var_123.as_str());
    }
    if let Some(var_124) = &input.distribution {
        object.key("distribution").string(var_124.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_query_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartQueryInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_125) = &input.log_group_name {
        object.key("logGroupName").string(var_125.as_str());
    }
    if let Some(var_126) = &input.log_group_names {
        let mut array_127 = object.key("logGroupNames").start_array();
        for item_128 in var_126 {
             {
                array_127.value().string(item_128.as_str());
            }
        }
        array_127.finish();
    }
    if let Some(var_129) = &input.start_time {
        object.key("startTime").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_129).into()));
    }
    if let Some(var_130) = &input.end_time {
        object.key("endTime").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_130).into()));
    }
    if let Some(var_131) = &input.query_string {
        object.key("queryString").string(var_131.as_str());
    }
    if let Some(var_132) = &input.limit {
        object.key("limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_132).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_query_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StopQueryInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_133) = &input.query_id {
        object.key("queryId").string(var_133.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_log_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagLogGroupInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_134) = &input.log_group_name {
        object.key("logGroupName").string(var_134.as_str());
    }
    if let Some(var_135) = &input.tags {
        let mut object_136 = object.key("tags").start_object();
        for (key_137, value_138) in var_135 {
             {
                object_136.key(key_137).string(value_138.as_str());
            }
        }
        object_136.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_test_metric_filter_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TestMetricFilterInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_139) = &input.filter_pattern {
        object.key("filterPattern").string(var_139.as_str());
    }
    if let Some(var_140) = &input.log_event_messages {
        let mut array_141 = object.key("logEventMessages").start_array();
        for item_142 in var_140 {
             {
                array_141.value().string(item_142.as_str());
            }
        }
        array_141.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_log_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UntagLogGroupInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_143) = &input.log_group_name {
        object.key("logGroupName").string(var_143.as_str());
    }
    if let Some(var_144) = &input.tags {
        let mut array_145 = object.key("tags").start_array();
        for item_146 in var_144 {
             {
                array_145.value().string(item_146.as_str());
            }
        }
        array_145.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_input_log_event(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::InputLogEvent) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_147) = &input.timestamp {
        object.key("timestamp").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_147).into()));
    }
    if let Some(var_148) = &input.message {
        object.key("message").string(var_148.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_metric_transformation(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MetricTransformation) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_149) = &input.metric_name {
        object.key("metricName").string(var_149.as_str());
    }
    if let Some(var_150) = &input.metric_namespace {
        object.key("metricNamespace").string(var_150.as_str());
    }
    if let Some(var_151) = &input.metric_value {
        object.key("metricValue").string(var_151.as_str());
    }
    if let Some(var_152) = &input.default_value {
        object.key("defaultValue").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_152).into()));
    }
    if let Some(var_153) = &input.dimensions {
        let mut object_154 = object.key("dimensions").start_object();
        for (key_155, value_156) in var_153 {
             {
                object_154.key(key_155).string(value_156.as_str());
            }
        }
        object_154.finish();
    }
    if let Some(var_157) = &input.unit {
        object.key("unit").string(var_157.as_str());
    }
    Ok(())
}

