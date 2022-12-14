// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_post_text_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PostTextInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.active_contexts {
        let mut array_2 = object.key("activeContexts").start_array();
        for item_3 in var_1 {
             {
                let mut object_4 = array_2.value().start_object();
                crate::json_ser::serialize_structure_crate_model_active_context(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.input_text {
        object.key("inputText").string(var_5.as_str());
    }
    if let Some(var_6) = &input.request_attributes {
        let mut object_7 = object.key("requestAttributes").start_object();
        for (key_8, value_9) in var_6 {
             {
                object_7.key(key_8).string(value_9.as_str());
            }
        }
        object_7.finish();
    }
    if let Some(var_10) = &input.session_attributes {
        let mut object_11 = object.key("sessionAttributes").start_object();
        for (key_12, value_13) in var_10 {
             {
                object_11.key(key_12).string(value_13.as_str());
            }
        }
        object_11.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_session_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutSessionInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_14) = &input.active_contexts {
        let mut array_15 = object.key("activeContexts").start_array();
        for item_16 in var_14 {
             {
                let mut object_17 = array_15.value().start_object();
                crate::json_ser::serialize_structure_crate_model_active_context(&mut object_17, item_16)?;
                object_17.finish();
            }
        }
        array_15.finish();
    }
    if let Some(var_18) = &input.dialog_action {
        let mut object_19 = object.key("dialogAction").start_object();
        crate::json_ser::serialize_structure_crate_model_dialog_action(&mut object_19, var_18)?;
        object_19.finish();
    }
    if let Some(var_20) = &input.recent_intent_summary_view {
        let mut array_21 = object.key("recentIntentSummaryView").start_array();
        for item_22 in var_20 {
             {
                let mut object_23 = array_21.value().start_object();
                crate::json_ser::serialize_structure_crate_model_intent_summary(&mut object_23, item_22)?;
                object_23.finish();
            }
        }
        array_21.finish();
    }
    if let Some(var_24) = &input.session_attributes {
        let mut object_25 = object.key("sessionAttributes").start_object();
        for (key_26, value_27) in var_24 {
             {
                object_25.key(key_26).string(value_27.as_str());
            }
        }
        object_25.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_active_context(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ActiveContext) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.name {
        object.key("name").string(var_28.as_str());
    }
    if let Some(var_29) = &input.time_to_live {
        let mut object_30 = object.key("timeToLive").start_object();
        crate::json_ser::serialize_structure_crate_model_active_context_time_to_live(&mut object_30, var_29)?;
        object_30.finish();
    }
    if let Some(var_31) = &input.parameters {
        let mut object_32 = object.key("parameters").start_object();
        for (key_33, value_34) in var_31 {
             {
                object_32.key(key_33).string(value_34.as_str());
            }
        }
        object_32.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dialog_action(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DialogAction) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_35) = &input.r#type {
        object.key("type").string(var_35.as_str());
    }
    if let Some(var_36) = &input.intent_name {
        object.key("intentName").string(var_36.as_str());
    }
    if let Some(var_37) = &input.slots {
        let mut object_38 = object.key("slots").start_object();
        for (key_39, value_40) in var_37 {
             {
                object_38.key(key_39).string(value_40.as_str());
            }
        }
        object_38.finish();
    }
    if let Some(var_41) = &input.slot_to_elicit {
        object.key("slotToElicit").string(var_41.as_str());
    }
    if let Some(var_42) = &input.fulfillment_state {
        object.key("fulfillmentState").string(var_42.as_str());
    }
    if let Some(var_43) = &input.message {
        object.key("message").string(var_43.as_str());
    }
    if let Some(var_44) = &input.message_format {
        object.key("messageFormat").string(var_44.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_intent_summary(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::IntentSummary) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_45) = &input.intent_name {
        object.key("intentName").string(var_45.as_str());
    }
    if let Some(var_46) = &input.checkpoint_label {
        object.key("checkpointLabel").string(var_46.as_str());
    }
    if let Some(var_47) = &input.slots {
        let mut object_48 = object.key("slots").start_object();
        for (key_49, value_50) in var_47 {
             {
                object_48.key(key_49).string(value_50.as_str());
            }
        }
        object_48.finish();
    }
    if let Some(var_51) = &input.confirmation_status {
        object.key("confirmationStatus").string(var_51.as_str());
    }
    if let Some(var_52) = &input.dialog_action_type {
        object.key("dialogActionType").string(var_52.as_str());
    }
    if let Some(var_53) = &input.fulfillment_state {
        object.key("fulfillmentState").string(var_53.as_str());
    }
    if let Some(var_54) = &input.slot_to_elicit {
        object.key("slotToElicit").string(var_54.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_active_context_time_to_live(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ActiveContextTimeToLive) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_55) = &input.time_to_live_in_seconds {
        object.key("timeToLiveInSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_55).into()));
    }
    if let Some(var_56) = &input.turns_to_live {
        object.key("turnsToLive").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_56).into()));
    }
    Ok(())
}

