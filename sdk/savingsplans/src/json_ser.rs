// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_savings_plan_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateSavingsPlanInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.commitment {
        object.key("commitment").string(var_2.as_str());
    }
    if let Some(var_3) = &input.purchase_time {
        object.key("purchaseTime").date_time(var_3, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_4) = &input.savings_plan_offering_id {
        object.key("savingsPlanOfferingId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.tags {
        let mut object_6 = object.key("tags").start_object();
        for (key_7, value_8) in var_5 {
             {
                object_6.key(key_7).string(value_8.as_str());
            }
        }
        object_6.finish();
    }
    if let Some(var_9) = &input.upfront_payment_amount {
        object.key("upfrontPaymentAmount").string(var_9.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_queued_savings_plan_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteQueuedSavingsPlanInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_10) = &input.savings_plan_id {
        object.key("savingsPlanId").string(var_10.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_savings_plan_rates_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeSavingsPlanRatesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_11) = &input.filters {
        let mut array_12 = object.key("filters").start_array();
        for item_13 in var_11 {
             {
                let mut object_14 = array_12.value().start_object();
                crate::json_ser::serialize_structure_crate_model_savings_plan_rate_filter(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    if let Some(var_15) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_15).into()));
    }
    if let Some(var_16) = &input.next_token {
        object.key("nextToken").string(var_16.as_str());
    }
    if let Some(var_17) = &input.savings_plan_id {
        object.key("savingsPlanId").string(var_17.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_savings_plans_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeSavingsPlansInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_18) = &input.filters {
        let mut array_19 = object.key("filters").start_array();
        for item_20 in var_18 {
             {
                let mut object_21 = array_19.value().start_object();
                crate::json_ser::serialize_structure_crate_model_savings_plan_filter(&mut object_21, item_20)?;
                object_21.finish();
            }
        }
        array_19.finish();
    }
    if let Some(var_22) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_22).into()));
    }
    if let Some(var_23) = &input.next_token {
        object.key("nextToken").string(var_23.as_str());
    }
    if let Some(var_24) = &input.savings_plan_arns {
        let mut array_25 = object.key("savingsPlanArns").start_array();
        for item_26 in var_24 {
             {
                array_25.value().string(item_26.as_str());
            }
        }
        array_25.finish();
    }
    if let Some(var_27) = &input.savings_plan_ids {
        let mut array_28 = object.key("savingsPlanIds").start_array();
        for item_29 in var_27 {
             {
                array_28.value().string(item_29.as_str());
            }
        }
        array_28.finish();
    }
    if let Some(var_30) = &input.states {
        let mut array_31 = object.key("states").start_array();
        for item_32 in var_30 {
             {
                array_31.value().string(item_32.as_str());
            }
        }
        array_31.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_savings_plans_offering_rates_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeSavingsPlansOfferingRatesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.filters {
        let mut array_34 = object.key("filters").start_array();
        for item_35 in var_33 {
             {
                let mut object_36 = array_34.value().start_object();
                crate::json_ser::serialize_structure_crate_model_savings_plan_offering_rate_filter_element(&mut object_36, item_35)?;
                object_36.finish();
            }
        }
        array_34.finish();
    }
    if input.max_results != 0 {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_results).into()));
    }
    if let Some(var_37) = &input.next_token {
        object.key("nextToken").string(var_37.as_str());
    }
    if let Some(var_38) = &input.operations {
        let mut array_39 = object.key("operations").start_array();
        for item_40 in var_38 {
             {
                array_39.value().string(item_40.as_str());
            }
        }
        array_39.finish();
    }
    if let Some(var_41) = &input.products {
        let mut array_42 = object.key("products").start_array();
        for item_43 in var_41 {
             {
                array_42.value().string(item_43.as_str());
            }
        }
        array_42.finish();
    }
    if let Some(var_44) = &input.savings_plan_offering_ids {
        let mut array_45 = object.key("savingsPlanOfferingIds").start_array();
        for item_46 in var_44 {
             {
                array_45.value().string(item_46.as_str());
            }
        }
        array_45.finish();
    }
    if let Some(var_47) = &input.savings_plan_payment_options {
        let mut array_48 = object.key("savingsPlanPaymentOptions").start_array();
        for item_49 in var_47 {
             {
                array_48.value().string(item_49.as_str());
            }
        }
        array_48.finish();
    }
    if let Some(var_50) = &input.savings_plan_types {
        let mut array_51 = object.key("savingsPlanTypes").start_array();
        for item_52 in var_50 {
             {
                array_51.value().string(item_52.as_str());
            }
        }
        array_51.finish();
    }
    if let Some(var_53) = &input.service_codes {
        let mut array_54 = object.key("serviceCodes").start_array();
        for item_55 in var_53 {
             {
                array_54.value().string(item_55.as_str());
            }
        }
        array_54.finish();
    }
    if let Some(var_56) = &input.usage_types {
        let mut array_57 = object.key("usageTypes").start_array();
        for item_58 in var_56 {
             {
                array_57.value().string(item_58.as_str());
            }
        }
        array_57.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_savings_plans_offerings_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeSavingsPlansOfferingsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_59) = &input.currencies {
        let mut array_60 = object.key("currencies").start_array();
        for item_61 in var_59 {
             {
                array_60.value().string(item_61.as_str());
            }
        }
        array_60.finish();
    }
    if let Some(var_62) = &input.descriptions {
        let mut array_63 = object.key("descriptions").start_array();
        for item_64 in var_62 {
             {
                array_63.value().string(item_64.as_str());
            }
        }
        array_63.finish();
    }
    if let Some(var_65) = &input.durations {
        let mut array_66 = object.key("durations").start_array();
        for item_67 in var_65 {
             {
                array_66.value().number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*item_67).into()));
            }
        }
        array_66.finish();
    }
    if let Some(var_68) = &input.filters {
        let mut array_69 = object.key("filters").start_array();
        for item_70 in var_68 {
             {
                let mut object_71 = array_69.value().start_object();
                crate::json_ser::serialize_structure_crate_model_savings_plan_offering_filter_element(&mut object_71, item_70)?;
                object_71.finish();
            }
        }
        array_69.finish();
    }
    if input.max_results != 0 {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_results).into()));
    }
    if let Some(var_72) = &input.next_token {
        object.key("nextToken").string(var_72.as_str());
    }
    if let Some(var_73) = &input.offering_ids {
        let mut array_74 = object.key("offeringIds").start_array();
        for item_75 in var_73 {
             {
                array_74.value().string(item_75.as_str());
            }
        }
        array_74.finish();
    }
    if let Some(var_76) = &input.operations {
        let mut array_77 = object.key("operations").start_array();
        for item_78 in var_76 {
             {
                array_77.value().string(item_78.as_str());
            }
        }
        array_77.finish();
    }
    if let Some(var_79) = &input.payment_options {
        let mut array_80 = object.key("paymentOptions").start_array();
        for item_81 in var_79 {
             {
                array_80.value().string(item_81.as_str());
            }
        }
        array_80.finish();
    }
    if let Some(var_82) = &input.plan_types {
        let mut array_83 = object.key("planTypes").start_array();
        for item_84 in var_82 {
             {
                array_83.value().string(item_84.as_str());
            }
        }
        array_83.finish();
    }
    if let Some(var_85) = &input.product_type {
        object.key("productType").string(var_85.as_str());
    }
    if let Some(var_86) = &input.service_codes {
        let mut array_87 = object.key("serviceCodes").start_array();
        for item_88 in var_86 {
             {
                array_87.value().string(item_88.as_str());
            }
        }
        array_87.finish();
    }
    if let Some(var_89) = &input.usage_types {
        let mut array_90 = object.key("usageTypes").start_array();
        for item_91 in var_89 {
             {
                array_90.value().string(item_91.as_str());
            }
        }
        array_90.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTagsForResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_92) = &input.resource_arn {
        object.key("resourceArn").string(var_92.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_93) = &input.resource_arn {
        object.key("resourceArn").string(var_93.as_str());
    }
    if let Some(var_94) = &input.tags {
        let mut object_95 = object.key("tags").start_object();
        for (key_96, value_97) in var_94 {
             {
                object_95.key(key_96).string(value_97.as_str());
            }
        }
        object_95.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UntagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_98) = &input.resource_arn {
        object.key("resourceArn").string(var_98.as_str());
    }
    if let Some(var_99) = &input.tag_keys {
        let mut array_100 = object.key("tagKeys").start_array();
        for item_101 in var_99 {
             {
                array_100.value().string(item_101.as_str());
            }
        }
        array_100.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_savings_plan_rate_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SavingsPlanRateFilter) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_102) = &input.name {
        object.key("name").string(var_102.as_str());
    }
    if let Some(var_103) = &input.values {
        let mut array_104 = object.key("values").start_array();
        for item_105 in var_103 {
             {
                array_104.value().string(item_105.as_str());
            }
        }
        array_104.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_savings_plan_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SavingsPlanFilter) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_106) = &input.name {
        object.key("name").string(var_106.as_str());
    }
    if let Some(var_107) = &input.values {
        let mut array_108 = object.key("values").start_array();
        for item_109 in var_107 {
             {
                array_108.value().string(item_109.as_str());
            }
        }
        array_108.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_savings_plan_offering_rate_filter_element(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SavingsPlanOfferingRateFilterElement) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_110) = &input.name {
        object.key("name").string(var_110.as_str());
    }
    if let Some(var_111) = &input.values {
        let mut array_112 = object.key("values").start_array();
        for item_113 in var_111 {
             {
                array_112.value().string(item_113.as_str());
            }
        }
        array_112.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_savings_plan_offering_filter_element(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SavingsPlanOfferingFilterElement) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_114) = &input.name {
        object.key("name").string(var_114.as_str());
    }
    if let Some(var_115) = &input.values {
        let mut array_116 = object.key("values").start_array();
        for item_117 in var_115 {
             {
                array_116.value().string(item_117.as_str());
            }
        }
        array_116.finish();
    }
    Ok(())
}

