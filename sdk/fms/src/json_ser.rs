// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_admin_account_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AssociateAdminAccountInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.admin_account {
        object.key("AdminAccount").string(var_1.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_associate_third_party_firewall_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AssociateThirdPartyFirewallInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_2) = &input.third_party_firewall {
        object.key("ThirdPartyFirewall").string(var_2.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_apps_list_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteAppsListInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_3) = &input.list_id {
        object.key("ListId").string(var_3.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeletePolicyInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_4) = &input.policy_id {
        object.key("PolicyId").string(var_4.as_str());
    }
    if input.delete_all_policy_resources {
        object.key("DeleteAllPolicyResources").boolean(input.delete_all_policy_resources);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_protocols_list_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteProtocolsListInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_5) = &input.list_id {
        object.key("ListId").string(var_5.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disassociate_third_party_firewall_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DisassociateThirdPartyFirewallInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_6) = &input.third_party_firewall {
        object.key("ThirdPartyFirewall").string(var_6.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_apps_list_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetAppsListInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_7) = &input.list_id {
        object.key("ListId").string(var_7.as_str());
    }
    if input.default_list {
        object.key("DefaultList").boolean(input.default_list);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_compliance_detail_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetComplianceDetailInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_8) = &input.policy_id {
        object.key("PolicyId").string(var_8.as_str());
    }
    if let Some(var_9) = &input.member_account {
        object.key("MemberAccount").string(var_9.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetPolicyInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_10) = &input.policy_id {
        object.key("PolicyId").string(var_10.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_protection_status_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetProtectionStatusInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_11) = &input.policy_id {
        object.key("PolicyId").string(var_11.as_str());
    }
    if let Some(var_12) = &input.member_account_id {
        object.key("MemberAccountId").string(var_12.as_str());
    }
    if let Some(var_13) = &input.start_time {
        object.key("StartTime").date_time(var_13, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_14) = &input.end_time {
        object.key("EndTime").date_time(var_14, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_15) = &input.next_token {
        object.key("NextToken").string(var_15.as_str());
    }
    if let Some(var_16) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_16).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_protocols_list_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetProtocolsListInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_17) = &input.list_id {
        object.key("ListId").string(var_17.as_str());
    }
    if input.default_list {
        object.key("DefaultList").boolean(input.default_list);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_third_party_firewall_association_status_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetThirdPartyFirewallAssociationStatusInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_18) = &input.third_party_firewall {
        object.key("ThirdPartyFirewall").string(var_18.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_violation_details_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetViolationDetailsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_19) = &input.policy_id {
        object.key("PolicyId").string(var_19.as_str());
    }
    if let Some(var_20) = &input.member_account {
        object.key("MemberAccount").string(var_20.as_str());
    }
    if let Some(var_21) = &input.resource_id {
        object.key("ResourceId").string(var_21.as_str());
    }
    if let Some(var_22) = &input.resource_type {
        object.key("ResourceType").string(var_22.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_apps_lists_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListAppsListsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.default_lists {
        object.key("DefaultLists").boolean(input.default_lists);
    }
    if let Some(var_23) = &input.next_token {
        object.key("NextToken").string(var_23.as_str());
    }
    if let Some(var_24) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_24).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_compliance_status_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListComplianceStatusInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_25) = &input.policy_id {
        object.key("PolicyId").string(var_25.as_str());
    }
    if let Some(var_26) = &input.next_token {
        object.key("NextToken").string(var_26.as_str());
    }
    if let Some(var_27) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_27).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_member_accounts_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListMemberAccountsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.next_token {
        object.key("NextToken").string(var_28.as_str());
    }
    if let Some(var_29) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_29).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_policies_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListPoliciesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_30) = &input.next_token {
        object.key("NextToken").string(var_30.as_str());
    }
    if let Some(var_31) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_31).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_protocols_lists_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListProtocolsListsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.default_lists {
        object.key("DefaultLists").boolean(input.default_lists);
    }
    if let Some(var_32) = &input.next_token {
        object.key("NextToken").string(var_32.as_str());
    }
    if let Some(var_33) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_33).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTagsForResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_34) = &input.resource_arn {
        object.key("ResourceArn").string(var_34.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_third_party_firewall_firewall_policies_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListThirdPartyFirewallFirewallPoliciesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_35) = &input.third_party_firewall {
        object.key("ThirdPartyFirewall").string(var_35.as_str());
    }
    if let Some(var_36) = &input.next_token {
        object.key("NextToken").string(var_36.as_str());
    }
    if let Some(var_37) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_37).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_apps_list_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutAppsListInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_38) = &input.apps_list {
        let mut object_39 = object.key("AppsList").start_object();
        crate::json_ser::serialize_structure_crate_model_apps_list_data(&mut object_39, var_38)?;
        object_39.finish();
    }
    if let Some(var_40) = &input.tag_list {
        let mut array_41 = object.key("TagList").start_array();
        for item_42 in var_40 {
             {
                let mut object_43 = array_41.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_43, item_42)?;
                object_43.finish();
            }
        }
        array_41.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_notification_channel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutNotificationChannelInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.sns_topic_arn {
        object.key("SnsTopicArn").string(var_44.as_str());
    }
    if let Some(var_45) = &input.sns_role_name {
        object.key("SnsRoleName").string(var_45.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutPolicyInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.policy {
        let mut object_47 = object.key("Policy").start_object();
        crate::json_ser::serialize_structure_crate_model_policy(&mut object_47, var_46)?;
        object_47.finish();
    }
    if let Some(var_48) = &input.tag_list {
        let mut array_49 = object.key("TagList").start_array();
        for item_50 in var_48 {
             {
                let mut object_51 = array_49.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_51, item_50)?;
                object_51.finish();
            }
        }
        array_49.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_protocols_list_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutProtocolsListInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_52) = &input.protocols_list {
        let mut object_53 = object.key("ProtocolsList").start_object();
        crate::json_ser::serialize_structure_crate_model_protocols_list_data(&mut object_53, var_52)?;
        object_53.finish();
    }
    if let Some(var_54) = &input.tag_list {
        let mut array_55 = object.key("TagList").start_array();
        for item_56 in var_54 {
             {
                let mut object_57 = array_55.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_57, item_56)?;
                object_57.finish();
            }
        }
        array_55.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_58) = &input.resource_arn {
        object.key("ResourceArn").string(var_58.as_str());
    }
    if let Some(var_59) = &input.tag_list {
        let mut array_60 = object.key("TagList").start_array();
        for item_61 in var_59 {
             {
                let mut object_62 = array_60.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_62, item_61)?;
                object_62.finish();
            }
        }
        array_60.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UntagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_63) = &input.resource_arn {
        object.key("ResourceArn").string(var_63.as_str());
    }
    if let Some(var_64) = &input.tag_keys {
        let mut array_65 = object.key("TagKeys").start_array();
        for item_66 in var_64 {
             {
                array_65.value().string(item_66.as_str());
            }
        }
        array_65.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_apps_list_data(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AppsListData) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_67) = &input.list_id {
        object.key("ListId").string(var_67.as_str());
    }
    if let Some(var_68) = &input.list_name {
        object.key("ListName").string(var_68.as_str());
    }
    if let Some(var_69) = &input.list_update_token {
        object.key("ListUpdateToken").string(var_69.as_str());
    }
    if let Some(var_70) = &input.create_time {
        object.key("CreateTime").date_time(var_70, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_71) = &input.last_update_time {
        object.key("LastUpdateTime").date_time(var_71, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_72) = &input.apps_list {
        let mut array_73 = object.key("AppsList").start_array();
        for item_74 in var_72 {
             {
                let mut object_75 = array_73.value().start_object();
                crate::json_ser::serialize_structure_crate_model_app(&mut object_75, item_74)?;
                object_75.finish();
            }
        }
        array_73.finish();
    }
    if let Some(var_76) = &input.previous_apps_list {
        let mut object_77 = object.key("PreviousAppsList").start_object();
        for (key_78, value_79) in var_76 {
             {
                let mut array_80 = object_77.key(key_78).start_array();
                for item_81 in value_79 {
                     {
                        let mut object_82 = array_80.value().start_object();
                        crate::json_ser::serialize_structure_crate_model_app(&mut object_82, item_81)?;
                        object_82.finish();
                    }
                }
                array_80.finish();
            }
        }
        object_77.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Tag) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_83) = &input.key {
        object.key("Key").string(var_83.as_str());
    }
    if let Some(var_84) = &input.value {
        object.key("Value").string(var_84.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_policy(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Policy) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_85) = &input.policy_id {
        object.key("PolicyId").string(var_85.as_str());
    }
    if let Some(var_86) = &input.policy_name {
        object.key("PolicyName").string(var_86.as_str());
    }
    if let Some(var_87) = &input.policy_update_token {
        object.key("PolicyUpdateToken").string(var_87.as_str());
    }
    if let Some(var_88) = &input.security_service_policy_data {
        let mut object_89 = object.key("SecurityServicePolicyData").start_object();
        crate::json_ser::serialize_structure_crate_model_security_service_policy_data(&mut object_89, var_88)?;
        object_89.finish();
    }
    if let Some(var_90) = &input.resource_type {
        object.key("ResourceType").string(var_90.as_str());
    }
    if let Some(var_91) = &input.resource_type_list {
        let mut array_92 = object.key("ResourceTypeList").start_array();
        for item_93 in var_91 {
             {
                array_92.value().string(item_93.as_str());
            }
        }
        array_92.finish();
    }
    if let Some(var_94) = &input.resource_tags {
        let mut array_95 = object.key("ResourceTags").start_array();
        for item_96 in var_94 {
             {
                let mut object_97 = array_95.value().start_object();
                crate::json_ser::serialize_structure_crate_model_resource_tag(&mut object_97, item_96)?;
                object_97.finish();
            }
        }
        array_95.finish();
    }
     {
        object.key("ExcludeResourceTags").boolean(input.exclude_resource_tags);
    }
     {
        object.key("RemediationEnabled").boolean(input.remediation_enabled);
    }
    if input.delete_unused_fm_managed_resources {
        object.key("DeleteUnusedFMManagedResources").boolean(input.delete_unused_fm_managed_resources);
    }
    if let Some(var_98) = &input.include_map {
        let mut object_99 = object.key("IncludeMap").start_object();
        for (key_100, value_101) in var_98 {
             {
                let mut array_102 = object_99.key(key_100.as_str()).start_array();
                for item_103 in value_101 {
                     {
                        array_102.value().string(item_103.as_str());
                    }
                }
                array_102.finish();
            }
        }
        object_99.finish();
    }
    if let Some(var_104) = &input.exclude_map {
        let mut object_105 = object.key("ExcludeMap").start_object();
        for (key_106, value_107) in var_104 {
             {
                let mut array_108 = object_105.key(key_106.as_str()).start_array();
                for item_109 in value_107 {
                     {
                        array_108.value().string(item_109.as_str());
                    }
                }
                array_108.finish();
            }
        }
        object_105.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_protocols_list_data(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ProtocolsListData) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_110) = &input.list_id {
        object.key("ListId").string(var_110.as_str());
    }
    if let Some(var_111) = &input.list_name {
        object.key("ListName").string(var_111.as_str());
    }
    if let Some(var_112) = &input.list_update_token {
        object.key("ListUpdateToken").string(var_112.as_str());
    }
    if let Some(var_113) = &input.create_time {
        object.key("CreateTime").date_time(var_113, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_114) = &input.last_update_time {
        object.key("LastUpdateTime").date_time(var_114, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_115) = &input.protocols_list {
        let mut array_116 = object.key("ProtocolsList").start_array();
        for item_117 in var_115 {
             {
                array_116.value().string(item_117.as_str());
            }
        }
        array_116.finish();
    }
    if let Some(var_118) = &input.previous_protocols_list {
        let mut object_119 = object.key("PreviousProtocolsList").start_object();
        for (key_120, value_121) in var_118 {
             {
                let mut array_122 = object_119.key(key_120).start_array();
                for item_123 in value_121 {
                     {
                        array_122.value().string(item_123.as_str());
                    }
                }
                array_122.finish();
            }
        }
        object_119.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_app(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::App) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_124) = &input.app_name {
        object.key("AppName").string(var_124.as_str());
    }
    if let Some(var_125) = &input.protocol {
        object.key("Protocol").string(var_125.as_str());
    }
    if let Some(var_126) = &input.port {
        object.key("Port").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_126).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_security_service_policy_data(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SecurityServicePolicyData) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_127) = &input.r#type {
        object.key("Type").string(var_127.as_str());
    }
    if let Some(var_128) = &input.managed_service_data {
        object.key("ManagedServiceData").string(var_128.as_str());
    }
    if let Some(var_129) = &input.policy_option {
        let mut object_130 = object.key("PolicyOption").start_object();
        crate::json_ser::serialize_structure_crate_model_policy_option(&mut object_130, var_129)?;
        object_130.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_resource_tag(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ResourceTag) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_131) = &input.key {
        object.key("Key").string(var_131.as_str());
    }
    if let Some(var_132) = &input.value {
        object.key("Value").string(var_132.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_policy_option(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PolicyOption) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_133) = &input.network_firewall_policy {
        let mut object_134 = object.key("NetworkFirewallPolicy").start_object();
        crate::json_ser::serialize_structure_crate_model_network_firewall_policy(&mut object_134, var_133)?;
        object_134.finish();
    }
    if let Some(var_135) = &input.third_party_firewall_policy {
        let mut object_136 = object.key("ThirdPartyFirewallPolicy").start_object();
        crate::json_ser::serialize_structure_crate_model_third_party_firewall_policy(&mut object_136, var_135)?;
        object_136.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_network_firewall_policy(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NetworkFirewallPolicy) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_137) = &input.firewall_deployment_model {
        object.key("FirewallDeploymentModel").string(var_137.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_third_party_firewall_policy(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ThirdPartyFirewallPolicy) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_138) = &input.firewall_deployment_model {
        object.key("FirewallDeploymentModel").string(var_138.as_str());
    }
    Ok(())
}

