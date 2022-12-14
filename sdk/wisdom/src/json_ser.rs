// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_assistant_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateAssistantInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.name {
        object.key("name").string(var_3.as_str());
    }
    if let Some(var_4) = &input.server_side_encryption_configuration {
        let mut object_5 = object.key("serverSideEncryptionConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_server_side_encryption_configuration(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.tags {
        let mut object_7 = object.key("tags").start_object();
        for (key_8, value_9) in var_6 {
             {
                object_7.key(key_8).string(value_9.as_str());
            }
        }
        object_7.finish();
    }
    if let Some(var_10) = &input.r#type {
        object.key("type").string(var_10.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_assistant_association_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateAssistantAssociationInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_11) = &input.association {
        let mut object_12 = object.key("association").start_object();
        crate::json_ser::serialize_union_crate_model_assistant_association_input_data(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.association_type {
        object.key("associationType").string(var_13.as_str());
    }
    if let Some(var_14) = &input.client_token {
        object.key("clientToken").string(var_14.as_str());
    }
    if let Some(var_15) = &input.tags {
        let mut object_16 = object.key("tags").start_object();
        for (key_17, value_18) in var_15 {
             {
                object_16.key(key_17).string(value_18.as_str());
            }
        }
        object_16.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_content_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateContentInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_19) = &input.client_token {
        object.key("clientToken").string(var_19.as_str());
    }
    if let Some(var_20) = &input.metadata {
        let mut object_21 = object.key("metadata").start_object();
        for (key_22, value_23) in var_20 {
             {
                object_21.key(key_22).string(value_23.as_str());
            }
        }
        object_21.finish();
    }
    if let Some(var_24) = &input.name {
        object.key("name").string(var_24.as_str());
    }
    if let Some(var_25) = &input.override_link_out_uri {
        object.key("overrideLinkOutUri").string(var_25.as_str());
    }
    if let Some(var_26) = &input.tags {
        let mut object_27 = object.key("tags").start_object();
        for (key_28, value_29) in var_26 {
             {
                object_27.key(key_28).string(value_29.as_str());
            }
        }
        object_27.finish();
    }
    if let Some(var_30) = &input.title {
        object.key("title").string(var_30.as_str());
    }
    if let Some(var_31) = &input.upload_id {
        object.key("uploadId").string(var_31.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_knowledge_base_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateKnowledgeBaseInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_32) = &input.client_token {
        object.key("clientToken").string(var_32.as_str());
    }
    if let Some(var_33) = &input.description {
        object.key("description").string(var_33.as_str());
    }
    if let Some(var_34) = &input.knowledge_base_type {
        object.key("knowledgeBaseType").string(var_34.as_str());
    }
    if let Some(var_35) = &input.name {
        object.key("name").string(var_35.as_str());
    }
    if let Some(var_36) = &input.rendering_configuration {
        let mut object_37 = object.key("renderingConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_rendering_configuration(&mut object_37, var_36)?;
        object_37.finish();
    }
    if let Some(var_38) = &input.server_side_encryption_configuration {
        let mut object_39 = object.key("serverSideEncryptionConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_server_side_encryption_configuration(&mut object_39, var_38)?;
        object_39.finish();
    }
    if let Some(var_40) = &input.source_configuration {
        let mut object_41 = object.key("sourceConfiguration").start_object();
        crate::json_ser::serialize_union_crate_model_source_configuration(&mut object_41, var_40)?;
        object_41.finish();
    }
    if let Some(var_42) = &input.tags {
        let mut object_43 = object.key("tags").start_object();
        for (key_44, value_45) in var_42 {
             {
                object_43.key(key_44).string(value_45.as_str());
            }
        }
        object_43.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_session_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateSessionInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.client_token {
        object.key("clientToken").string(var_46.as_str());
    }
    if let Some(var_47) = &input.description {
        object.key("description").string(var_47.as_str());
    }
    if let Some(var_48) = &input.name {
        object.key("name").string(var_48.as_str());
    }
    if let Some(var_49) = &input.tags {
        let mut object_50 = object.key("tags").start_object();
        for (key_51, value_52) in var_49 {
             {
                object_50.key(key_51).string(value_52.as_str());
            }
        }
        object_50.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_notify_recommendations_received_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::NotifyRecommendationsReceivedInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_53) = &input.recommendation_ids {
        let mut array_54 = object.key("recommendationIds").start_array();
        for item_55 in var_53 {
             {
                array_54.value().string(item_55.as_str());
            }
        }
        array_54.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_feedback_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutFeedbackInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_56) = &input.feedback {
        let mut object_57 = object.key("feedback").start_object();
        crate::json_ser::serialize_structure_crate_model_feedback_data(&mut object_57, var_56)?;
        object_57.finish();
    }
    if let Some(var_58) = &input.target_id {
        object.key("targetId").string(var_58.as_str());
    }
    if let Some(var_59) = &input.target_type {
        object.key("targetType").string(var_59.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_query_assistant_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::QueryAssistantInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_60) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_60).into()));
    }
    if let Some(var_61) = &input.next_token {
        object.key("nextToken").string(var_61.as_str());
    }
    if let Some(var_62) = &input.query_text {
        object.key("queryText").string(var_62.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_search_content_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SearchContentInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_63) = &input.search_expression {
        let mut object_64 = object.key("searchExpression").start_object();
        crate::json_ser::serialize_structure_crate_model_search_expression(&mut object_64, var_63)?;
        object_64.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_search_sessions_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SearchSessionsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_65) = &input.search_expression {
        let mut object_66 = object.key("searchExpression").start_object();
        crate::json_ser::serialize_structure_crate_model_search_expression(&mut object_66, var_65)?;
        object_66.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_content_upload_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartContentUploadInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_67) = &input.content_type {
        object.key("contentType").string(var_67.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_68) = &input.tags {
        let mut object_69 = object.key("tags").start_object();
        for (key_70, value_71) in var_68 {
             {
                object_69.key(key_70).string(value_71.as_str());
            }
        }
        object_69.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_content_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateContentInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_72) = &input.metadata {
        let mut object_73 = object.key("metadata").start_object();
        for (key_74, value_75) in var_72 {
             {
                object_73.key(key_74).string(value_75.as_str());
            }
        }
        object_73.finish();
    }
    if let Some(var_76) = &input.override_link_out_uri {
        object.key("overrideLinkOutUri").string(var_76.as_str());
    }
    if let Some(var_77) = &input.remove_override_link_out_uri {
        object.key("removeOverrideLinkOutUri").boolean(*var_77);
    }
    if let Some(var_78) = &input.revision_id {
        object.key("revisionId").string(var_78.as_str());
    }
    if let Some(var_79) = &input.title {
        object.key("title").string(var_79.as_str());
    }
    if let Some(var_80) = &input.upload_id {
        object.key("uploadId").string(var_80.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_knowledge_base_template_uri_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateKnowledgeBaseTemplateUriInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_81) = &input.template_uri {
        object.key("templateUri").string(var_81.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_server_side_encryption_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ServerSideEncryptionConfiguration) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_82) = &input.kms_key_id {
        object.key("kmsKeyId").string(var_82.as_str());
    }
    Ok(())
}

pub fn serialize_union_crate_model_assistant_association_input_data(object_12: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AssistantAssociationInputData) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::AssistantAssociationInputData::KnowledgeBaseId(inner) => {
             {
                object_12.key("knowledgeBaseId").string(inner.as_str());
            }
        },
        crate::model::AssistantAssociationInputData::Unknown => return Err(aws_smithy_http::operation::SerializationError::unknown_variant("AssistantAssociationInputData"))
    }
    Ok(())
}

pub fn serialize_structure_crate_model_rendering_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RenderingConfiguration) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_83) = &input.template_uri {
        object.key("templateUri").string(var_83.as_str());
    }
    Ok(())
}

pub fn serialize_union_crate_model_source_configuration(object_41: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SourceConfiguration) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::SourceConfiguration::AppIntegrations(inner) => {
             {
                let mut object_84 = object_41.key("appIntegrations").start_object();
                crate::json_ser::serialize_structure_crate_model_app_integrations_configuration(&mut object_84, inner)?;
                object_84.finish();
            }
        },
        crate::model::SourceConfiguration::Unknown => return Err(aws_smithy_http::operation::SerializationError::unknown_variant("SourceConfiguration"))
    }
    Ok(())
}

pub fn serialize_structure_crate_model_feedback_data(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::FeedbackData) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_85) = &input.relevance {
        object.key("relevance").string(var_85.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_search_expression(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SearchExpression) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_86) = &input.filters {
        let mut array_87 = object.key("filters").start_array();
        for item_88 in var_86 {
             {
                let mut object_89 = array_87.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter(&mut object_89, item_88)?;
                object_89.finish();
            }
        }
        array_87.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_app_integrations_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AppIntegrationsConfiguration) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_90) = &input.app_integration_arn {
        object.key("appIntegrationArn").string(var_90.as_str());
    }
    if let Some(var_91) = &input.object_fields {
        let mut array_92 = object.key("objectFields").start_array();
        for item_93 in var_91 {
             {
                array_92.value().string(item_93.as_str());
            }
        }
        array_92.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Filter) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_94) = &input.field {
        object.key("field").string(var_94.as_str());
    }
    if let Some(var_95) = &input.operator {
        object.key("operator").string(var_95.as_str());
    }
    if let Some(var_96) = &input.value {
        object.key("value").string(var_96.as_str());
    }
    Ok(())
}

