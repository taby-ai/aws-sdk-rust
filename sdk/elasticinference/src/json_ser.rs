// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_describe_accelerator_offerings_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeAcceleratorOfferingsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.accelerator_types {
        let mut array_2 = object.key("acceleratorTypes").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.location_type {
        object.key("locationType").string(var_4.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_accelerators_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeAcceleratorsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_5) = &input.accelerator_ids {
        let mut array_6 = object.key("acceleratorIds").start_array();
        for item_7 in var_5 {
             {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.filters {
        let mut array_9 = object.key("filters").start_array();
        for item_10 in var_8 {
             {
                let mut object_11 = array_9.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if input.max_results != 0 {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_results).into()));
    }
    if let Some(var_12) = &input.next_token {
        object.key("nextToken").string(var_12.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
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

pub fn serialize_structure_crate_model_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Filter) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_17) = &input.name {
        object.key("name").string(var_17.as_str());
    }
    if let Some(var_18) = &input.values {
        let mut array_19 = object.key("values").start_array();
        for item_20 in var_18 {
             {
                array_19.value().string(item_20.as_str());
            }
        }
        array_19.finish();
    }
    Ok(())
}

