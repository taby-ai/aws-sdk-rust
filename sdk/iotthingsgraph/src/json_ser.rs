// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_entity_to_thing_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AssociateEntityToThingInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.thing_name {
        object.key("thingName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.entity_id {
        object.key("entityId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.namespace_version {
        object.key("namespaceVersion").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_flow_template_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateFlowTemplateInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_4) = &input.definition {
        let mut object_5 = object.key("definition").start_object();
        crate::json_ser::serialize_structure_crate_model_definition_document(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.compatible_namespace_version {
        object.key("compatibleNamespaceVersion").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_6).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_system_instance_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateSystemInstanceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_7) = &input.tags {
        let mut array_8 = object.key("tags").start_array();
        for item_9 in var_7 {
             {
                let mut object_10 = array_8.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.definition {
        let mut object_12 = object.key("definition").start_object();
        crate::json_ser::serialize_structure_crate_model_definition_document(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.target {
        object.key("target").string(var_13.as_str());
    }
    if let Some(var_14) = &input.greengrass_group_name {
        object.key("greengrassGroupName").string(var_14.as_str());
    }
    if let Some(var_15) = &input.s3_bucket_name {
        object.key("s3BucketName").string(var_15.as_str());
    }
    if let Some(var_16) = &input.metrics_configuration {
        let mut object_17 = object.key("metricsConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_metrics_configuration(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.flow_actions_role_arn {
        object.key("flowActionsRoleArn").string(var_18.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_system_template_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateSystemTemplateInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_19) = &input.definition {
        let mut object_20 = object.key("definition").start_object();
        crate::json_ser::serialize_structure_crate_model_definition_document(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.compatible_namespace_version {
        object.key("compatibleNamespaceVersion").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_21).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_flow_template_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteFlowTemplateInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_22) = &input.id {
        object.key("id").string(var_22.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_system_instance_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteSystemInstanceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_23) = &input.id {
        object.key("id").string(var_23.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_system_template_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteSystemTemplateInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_24) = &input.id {
        object.key("id").string(var_24.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_deploy_system_instance_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeploySystemInstanceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_25) = &input.id {
        object.key("id").string(var_25.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_deprecate_flow_template_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeprecateFlowTemplateInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_26) = &input.id {
        object.key("id").string(var_26.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_deprecate_system_template_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeprecateSystemTemplateInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_27) = &input.id {
        object.key("id").string(var_27.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_namespace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeNamespaceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.namespace_name {
        object.key("namespaceName").string(var_28.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_dissociate_entity_from_thing_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DissociateEntityFromThingInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_29) = &input.thing_name {
        object.key("thingName").string(var_29.as_str());
    }
    if let Some(var_30) = &input.entity_type {
        object.key("entityType").string(var_30.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_entities_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetEntitiesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_31) = &input.ids {
        let mut array_32 = object.key("ids").start_array();
        for item_33 in var_31 {
             {
                array_32.value().string(item_33.as_str());
            }
        }
        array_32.finish();
    }
    if let Some(var_34) = &input.namespace_version {
        object.key("namespaceVersion").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_34).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_flow_template_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetFlowTemplateInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_35) = &input.id {
        object.key("id").string(var_35.as_str());
    }
    if let Some(var_36) = &input.revision_number {
        object.key("revisionNumber").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_36).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_flow_template_revisions_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetFlowTemplateRevisionsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_37) = &input.id {
        object.key("id").string(var_37.as_str());
    }
    if let Some(var_38) = &input.next_token {
        object.key("nextToken").string(var_38.as_str());
    }
    if let Some(var_39) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_39).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_system_instance_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetSystemInstanceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_40) = &input.id {
        object.key("id").string(var_40.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_system_template_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetSystemTemplateInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_41) = &input.id {
        object.key("id").string(var_41.as_str());
    }
    if let Some(var_42) = &input.revision_number {
        object.key("revisionNumber").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_42).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_system_template_revisions_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetSystemTemplateRevisionsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_43) = &input.id {
        object.key("id").string(var_43.as_str());
    }
    if let Some(var_44) = &input.next_token {
        object.key("nextToken").string(var_44.as_str());
    }
    if let Some(var_45) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_45).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_upload_status_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetUploadStatusInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.upload_id {
        object.key("uploadId").string(var_46.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_flow_execution_messages_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListFlowExecutionMessagesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_47) = &input.flow_execution_id {
        object.key("flowExecutionId").string(var_47.as_str());
    }
    if let Some(var_48) = &input.next_token {
        object.key("nextToken").string(var_48.as_str());
    }
    if let Some(var_49) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_49).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTagsForResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_50) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_50).into()));
    }
    if let Some(var_51) = &input.resource_arn {
        object.key("resourceArn").string(var_51.as_str());
    }
    if let Some(var_52) = &input.next_token {
        object.key("nextToken").string(var_52.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_search_entities_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SearchEntitiesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_53) = &input.entity_types {
        let mut array_54 = object.key("entityTypes").start_array();
        for item_55 in var_53 {
             {
                array_54.value().string(item_55.as_str());
            }
        }
        array_54.finish();
    }
    if let Some(var_56) = &input.filters {
        let mut array_57 = object.key("filters").start_array();
        for item_58 in var_56 {
             {
                let mut object_59 = array_57.value().start_object();
                crate::json_ser::serialize_structure_crate_model_entity_filter(&mut object_59, item_58)?;
                object_59.finish();
            }
        }
        array_57.finish();
    }
    if let Some(var_60) = &input.next_token {
        object.key("nextToken").string(var_60.as_str());
    }
    if let Some(var_61) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_61).into()));
    }
    if let Some(var_62) = &input.namespace_version {
        object.key("namespaceVersion").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_62).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_search_flow_executions_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SearchFlowExecutionsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_63) = &input.system_instance_id {
        object.key("systemInstanceId").string(var_63.as_str());
    }
    if let Some(var_64) = &input.flow_execution_id {
        object.key("flowExecutionId").string(var_64.as_str());
    }
    if let Some(var_65) = &input.start_time {
        object.key("startTime").date_time(var_65, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_66) = &input.end_time {
        object.key("endTime").date_time(var_66, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_67) = &input.next_token {
        object.key("nextToken").string(var_67.as_str());
    }
    if let Some(var_68) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_68).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_search_flow_templates_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SearchFlowTemplatesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_69) = &input.filters {
        let mut array_70 = object.key("filters").start_array();
        for item_71 in var_69 {
             {
                let mut object_72 = array_70.value().start_object();
                crate::json_ser::serialize_structure_crate_model_flow_template_filter(&mut object_72, item_71)?;
                object_72.finish();
            }
        }
        array_70.finish();
    }
    if let Some(var_73) = &input.next_token {
        object.key("nextToken").string(var_73.as_str());
    }
    if let Some(var_74) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_74).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_search_system_instances_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SearchSystemInstancesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_75) = &input.filters {
        let mut array_76 = object.key("filters").start_array();
        for item_77 in var_75 {
             {
                let mut object_78 = array_76.value().start_object();
                crate::json_ser::serialize_structure_crate_model_system_instance_filter(&mut object_78, item_77)?;
                object_78.finish();
            }
        }
        array_76.finish();
    }
    if let Some(var_79) = &input.next_token {
        object.key("nextToken").string(var_79.as_str());
    }
    if let Some(var_80) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_80).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_search_system_templates_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SearchSystemTemplatesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_81) = &input.filters {
        let mut array_82 = object.key("filters").start_array();
        for item_83 in var_81 {
             {
                let mut object_84 = array_82.value().start_object();
                crate::json_ser::serialize_structure_crate_model_system_template_filter(&mut object_84, item_83)?;
                object_84.finish();
            }
        }
        array_82.finish();
    }
    if let Some(var_85) = &input.next_token {
        object.key("nextToken").string(var_85.as_str());
    }
    if let Some(var_86) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_86).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_search_things_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SearchThingsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_87) = &input.entity_id {
        object.key("entityId").string(var_87.as_str());
    }
    if let Some(var_88) = &input.next_token {
        object.key("nextToken").string(var_88.as_str());
    }
    if let Some(var_89) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_89).into()));
    }
    if let Some(var_90) = &input.namespace_version {
        object.key("namespaceVersion").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_90).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_91) = &input.resource_arn {
        object.key("resourceArn").string(var_91.as_str());
    }
    if let Some(var_92) = &input.tags {
        let mut array_93 = object.key("tags").start_array();
        for item_94 in var_92 {
             {
                let mut object_95 = array_93.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_95, item_94)?;
                object_95.finish();
            }
        }
        array_93.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_undeploy_system_instance_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UndeploySystemInstanceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_96) = &input.id {
        object.key("id").string(var_96.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UntagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_97) = &input.resource_arn {
        object.key("resourceArn").string(var_97.as_str());
    }
    if let Some(var_98) = &input.tag_keys {
        let mut array_99 = object.key("tagKeys").start_array();
        for item_100 in var_98 {
             {
                array_99.value().string(item_100.as_str());
            }
        }
        array_99.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_flow_template_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateFlowTemplateInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_101) = &input.id {
        object.key("id").string(var_101.as_str());
    }
    if let Some(var_102) = &input.definition {
        let mut object_103 = object.key("definition").start_object();
        crate::json_ser::serialize_structure_crate_model_definition_document(&mut object_103, var_102)?;
        object_103.finish();
    }
    if let Some(var_104) = &input.compatible_namespace_version {
        object.key("compatibleNamespaceVersion").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_104).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_system_template_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateSystemTemplateInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_105) = &input.id {
        object.key("id").string(var_105.as_str());
    }
    if let Some(var_106) = &input.definition {
        let mut object_107 = object.key("definition").start_object();
        crate::json_ser::serialize_structure_crate_model_definition_document(&mut object_107, var_106)?;
        object_107.finish();
    }
    if let Some(var_108) = &input.compatible_namespace_version {
        object.key("compatibleNamespaceVersion").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_108).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_upload_entity_definitions_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UploadEntityDefinitionsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_109) = &input.document {
        let mut object_110 = object.key("document").start_object();
        crate::json_ser::serialize_structure_crate_model_definition_document(&mut object_110, var_109)?;
        object_110.finish();
    }
    if input.sync_with_public_namespace {
        object.key("syncWithPublicNamespace").boolean(input.sync_with_public_namespace);
    }
    if input.deprecate_existing_entities {
        object.key("deprecateExistingEntities").boolean(input.deprecate_existing_entities);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_definition_document(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DefinitionDocument) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_111) = &input.language {
        object.key("language").string(var_111.as_str());
    }
    if let Some(var_112) = &input.text {
        object.key("text").string(var_112.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Tag) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_113) = &input.key {
        object.key("key").string(var_113.as_str());
    }
    if let Some(var_114) = &input.value {
        object.key("value").string(var_114.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_metrics_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MetricsConfiguration) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.cloud_metric_enabled {
        object.key("cloudMetricEnabled").boolean(input.cloud_metric_enabled);
    }
    if let Some(var_115) = &input.metric_rule_role_arn {
        object.key("metricRuleRoleArn").string(var_115.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_entity_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EntityFilter) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_116) = &input.name {
        object.key("name").string(var_116.as_str());
    }
    if let Some(var_117) = &input.value {
        let mut array_118 = object.key("value").start_array();
        for item_119 in var_117 {
             {
                array_118.value().string(item_119.as_str());
            }
        }
        array_118.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_flow_template_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::FlowTemplateFilter) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_120) = &input.name {
        object.key("name").string(var_120.as_str());
    }
    if let Some(var_121) = &input.value {
        let mut array_122 = object.key("value").start_array();
        for item_123 in var_121 {
             {
                array_122.value().string(item_123.as_str());
            }
        }
        array_122.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_system_instance_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SystemInstanceFilter) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_124) = &input.name {
        object.key("name").string(var_124.as_str());
    }
    if let Some(var_125) = &input.value {
        let mut array_126 = object.key("value").start_array();
        for item_127 in var_125 {
             {
                array_126.value().string(item_127.as_str());
            }
        }
        array_126.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_system_template_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SystemTemplateFilter) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_128) = &input.name {
        object.key("name").string(var_128.as_str());
    }
    if let Some(var_129) = &input.value {
        let mut array_130 = object.key("value").start_array();
        for item_131 in var_129 {
             {
                array_130.value().string(item_131.as_str());
            }
        }
        array_130.finish();
    }
    Ok(())
}

