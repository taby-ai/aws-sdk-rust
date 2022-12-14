// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_add_notification_channels_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AddNotificationChannelsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.channels {
        let mut array_2 = object.key("channels").start_array();
        for item_3 in var_1 {
             {
                let mut object_4 = array_2.value().start_object();
                crate::json_ser::serialize_structure_crate_model_channel(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_get_frame_metric_data_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::BatchGetFrameMetricDataInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_5) = &input.frame_metrics {
        let mut array_6 = object.key("frameMetrics").start_array();
        for item_7 in var_5 {
             {
                let mut object_8 = array_6.value().start_object();
                crate::json_ser::serialize_structure_crate_model_frame_metric(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_configure_agent_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ConfigureAgentInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_9) = &input.fleet_instance_id {
        object.key("fleetInstanceId").string(var_9.as_str());
    }
    if let Some(var_10) = &input.metadata {
        let mut object_11 = object.key("metadata").start_object();
        for (key_12, value_13) in var_10 {
             {
                object_11.key(key_12.as_str()).string(value_13.as_str());
            }
        }
        object_11.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_profiling_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateProfilingGroupInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_14) = &input.agent_orchestration_config {
        let mut object_15 = object.key("agentOrchestrationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_agent_orchestration_config(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.compute_platform {
        object.key("computePlatform").string(var_16.as_str());
    }
    if let Some(var_17) = &input.profiling_group_name {
        object.key("profilingGroupName").string(var_17.as_str());
    }
    if let Some(var_18) = &input.tags {
        let mut object_19 = object.key("tags").start_object();
        for (key_20, value_21) in var_18 {
             {
                object_19.key(key_20).string(value_21.as_str());
            }
        }
        object_19.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_permission_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutPermissionInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_22) = &input.principals {
        let mut array_23 = object.key("principals").start_array();
        for item_24 in var_22 {
             {
                array_23.value().string(item_24.as_str());
            }
        }
        array_23.finish();
    }
    if let Some(var_25) = &input.revision_id {
        object.key("revisionId").string(var_25.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_submit_feedback_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SubmitFeedbackInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_26) = &input.comment {
        object.key("comment").string(var_26.as_str());
    }
    if let Some(var_27) = &input.r#type {
        object.key("type").string(var_27.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.tags {
        let mut object_29 = object.key("tags").start_object();
        for (key_30, value_31) in var_28 {
             {
                object_29.key(key_30).string(value_31.as_str());
            }
        }
        object_29.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_profiling_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateProfilingGroupInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_32) = &input.agent_orchestration_config {
        let mut object_33 = object.key("agentOrchestrationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_agent_orchestration_config(&mut object_33, var_32)?;
        object_33.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_channel(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Channel) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_34) = &input.id {
        object.key("id").string(var_34.as_str());
    }
    if let Some(var_35) = &input.uri {
        object.key("uri").string(var_35.as_str());
    }
    if let Some(var_36) = &input.event_publishers {
        let mut array_37 = object.key("eventPublishers").start_array();
        for item_38 in var_36 {
             {
                array_37.value().string(item_38.as_str());
            }
        }
        array_37.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_frame_metric(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::FrameMetric) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_39) = &input.frame_name {
        object.key("frameName").string(var_39.as_str());
    }
    if let Some(var_40) = &input.r#type {
        object.key("type").string(var_40.as_str());
    }
    if let Some(var_41) = &input.thread_states {
        let mut array_42 = object.key("threadStates").start_array();
        for item_43 in var_41 {
             {
                array_42.value().string(item_43.as_str());
            }
        }
        array_42.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_agent_orchestration_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AgentOrchestrationConfig) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.profiling_enabled {
        object.key("profilingEnabled").boolean(*var_44);
    }
    Ok(())
}

