// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_chat_token_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateChatTokenInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.attributes {
        let mut object_2 = object.key("attributes").start_object();
        for (key_3, value_4) in var_1 {
             {
                object_2.key(key_3).string(value_4.as_str());
            }
        }
        object_2.finish();
    }
    if let Some(var_5) = &input.capabilities {
        let mut array_6 = object.key("capabilities").start_array();
        for item_7 in var_5 {
             {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.room_identifier {
        object.key("roomIdentifier").string(var_8.as_str());
    }
    if input.session_duration_in_minutes != 0 {
        object.key("sessionDurationInMinutes").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.session_duration_in_minutes).into()));
    }
    if let Some(var_9) = &input.user_id {
        object.key("userId").string(var_9.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_room_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateRoomInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.maximum_message_length != 0 {
        object.key("maximumMessageLength").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.maximum_message_length).into()));
    }
    if input.maximum_message_rate_per_second != 0 {
        object.key("maximumMessageRatePerSecond").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.maximum_message_rate_per_second).into()));
    }
    if let Some(var_10) = &input.message_review_handler {
        let mut object_11 = object.key("messageReviewHandler").start_object();
        crate::json_ser::serialize_structure_crate_model_message_review_handler(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.name {
        object.key("name").string(var_12.as_str());
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

pub fn serialize_structure_crate_input_delete_message_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteMessageInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_17) = &input.id {
        object.key("id").string(var_17.as_str());
    }
    if let Some(var_18) = &input.reason {
        object.key("reason").string(var_18.as_str());
    }
    if let Some(var_19) = &input.room_identifier {
        object.key("roomIdentifier").string(var_19.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_room_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteRoomInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_20) = &input.identifier {
        object.key("identifier").string(var_20.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disconnect_user_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DisconnectUserInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_21) = &input.reason {
        object.key("reason").string(var_21.as_str());
    }
    if let Some(var_22) = &input.room_identifier {
        object.key("roomIdentifier").string(var_22.as_str());
    }
    if let Some(var_23) = &input.user_id {
        object.key("userId").string(var_23.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_room_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetRoomInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_24) = &input.identifier {
        object.key("identifier").string(var_24.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_rooms_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListRoomsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.max_results != 0 {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_results).into()));
    }
    if let Some(var_25) = &input.message_review_handler_uri {
        object.key("messageReviewHandlerUri").string(var_25.as_str());
    }
    if let Some(var_26) = &input.name {
        object.key("name").string(var_26.as_str());
    }
    if let Some(var_27) = &input.next_token {
        object.key("nextToken").string(var_27.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_send_event_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SendEventInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.attributes {
        let mut object_29 = object.key("attributes").start_object();
        for (key_30, value_31) in var_28 {
             {
                object_29.key(key_30).string(value_31.as_str());
            }
        }
        object_29.finish();
    }
    if let Some(var_32) = &input.event_name {
        object.key("eventName").string(var_32.as_str());
    }
    if let Some(var_33) = &input.room_identifier {
        object.key("roomIdentifier").string(var_33.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_34) = &input.tags {
        let mut object_35 = object.key("tags").start_object();
        for (key_36, value_37) in var_34 {
             {
                object_35.key(key_36).string(value_37.as_str());
            }
        }
        object_35.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_room_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateRoomInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_38) = &input.identifier {
        object.key("identifier").string(var_38.as_str());
    }
    if input.maximum_message_length != 0 {
        object.key("maximumMessageLength").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.maximum_message_length).into()));
    }
    if input.maximum_message_rate_per_second != 0 {
        object.key("maximumMessageRatePerSecond").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.maximum_message_rate_per_second).into()));
    }
    if let Some(var_39) = &input.message_review_handler {
        let mut object_40 = object.key("messageReviewHandler").start_object();
        crate::json_ser::serialize_structure_crate_model_message_review_handler(&mut object_40, var_39)?;
        object_40.finish();
    }
    if let Some(var_41) = &input.name {
        object.key("name").string(var_41.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_message_review_handler(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MessageReviewHandler) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_42) = &input.uri {
        object.key("uri").string(var_42.as_str());
    }
    if let Some(var_43) = &input.fallback_result {
        object.key("fallbackResult").string(var_43.as_str());
    }
    Ok(())
}

