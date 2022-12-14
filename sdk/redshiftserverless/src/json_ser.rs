// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_convert_recovery_point_to_snapshot_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ConvertRecoveryPointToSnapshotInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.recovery_point_id {
        object.key("recoveryPointId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.snapshot_name {
        object.key("snapshotName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.retention_period {
        object.key("retentionPeriod").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_endpoint_access_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateEndpointAccessInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_4) = &input.endpoint_name {
        object.key("endpointName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.subnet_ids {
        let mut array_6 = object.key("subnetIds").start_array();
        for item_7 in var_5 {
             {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.workgroup_name {
        object.key("workgroupName").string(var_8.as_str());
    }
    if let Some(var_9) = &input.vpc_security_group_ids {
        let mut array_10 = object.key("vpcSecurityGroupIds").start_array();
        for item_11 in var_9 {
             {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_namespace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateNamespaceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_12) = &input.namespace_name {
        object.key("namespaceName").string(var_12.as_str());
    }
    if let Some(var_13) = &input.admin_username {
        object.key("adminUsername").string(var_13.as_str());
    }
    if let Some(var_14) = &input.admin_user_password {
        object.key("adminUserPassword").string(var_14.as_str());
    }
    if let Some(var_15) = &input.db_name {
        object.key("dbName").string(var_15.as_str());
    }
    if let Some(var_16) = &input.kms_key_id {
        object.key("kmsKeyId").string(var_16.as_str());
    }
    if let Some(var_17) = &input.default_iam_role_arn {
        object.key("defaultIamRoleArn").string(var_17.as_str());
    }
    if let Some(var_18) = &input.iam_roles {
        let mut array_19 = object.key("iamRoles").start_array();
        for item_20 in var_18 {
             {
                array_19.value().string(item_20.as_str());
            }
        }
        array_19.finish();
    }
    if let Some(var_21) = &input.log_exports {
        let mut array_22 = object.key("logExports").start_array();
        for item_23 in var_21 {
             {
                array_22.value().string(item_23.as_str());
            }
        }
        array_22.finish();
    }
    if let Some(var_24) = &input.tags {
        let mut array_25 = object.key("tags").start_array();
        for item_26 in var_24 {
             {
                let mut object_27 = array_25.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_27, item_26)?;
                object_27.finish();
            }
        }
        array_25.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_snapshot_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateSnapshotInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.namespace_name {
        object.key("namespaceName").string(var_28.as_str());
    }
    if let Some(var_29) = &input.snapshot_name {
        object.key("snapshotName").string(var_29.as_str());
    }
    if let Some(var_30) = &input.retention_period {
        object.key("retentionPeriod").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_30).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_usage_limit_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateUsageLimitInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_31) = &input.resource_arn {
        object.key("resourceArn").string(var_31.as_str());
    }
    if let Some(var_32) = &input.usage_type {
        object.key("usageType").string(var_32.as_str());
    }
    if let Some(var_33) = &input.amount {
        object.key("amount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_33).into()));
    }
    if let Some(var_34) = &input.period {
        object.key("period").string(var_34.as_str());
    }
    if let Some(var_35) = &input.breach_action {
        object.key("breachAction").string(var_35.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_workgroup_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateWorkgroupInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_36) = &input.workgroup_name {
        object.key("workgroupName").string(var_36.as_str());
    }
    if let Some(var_37) = &input.namespace_name {
        object.key("namespaceName").string(var_37.as_str());
    }
    if let Some(var_38) = &input.base_capacity {
        object.key("baseCapacity").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_38).into()));
    }
    if let Some(var_39) = &input.enhanced_vpc_routing {
        object.key("enhancedVpcRouting").boolean(*var_39);
    }
    if let Some(var_40) = &input.config_parameters {
        let mut array_41 = object.key("configParameters").start_array();
        for item_42 in var_40 {
             {
                let mut object_43 = array_41.value().start_object();
                crate::json_ser::serialize_structure_crate_model_config_parameter(&mut object_43, item_42)?;
                object_43.finish();
            }
        }
        array_41.finish();
    }
    if let Some(var_44) = &input.security_group_ids {
        let mut array_45 = object.key("securityGroupIds").start_array();
        for item_46 in var_44 {
             {
                array_45.value().string(item_46.as_str());
            }
        }
        array_45.finish();
    }
    if let Some(var_47) = &input.subnet_ids {
        let mut array_48 = object.key("subnetIds").start_array();
        for item_49 in var_47 {
             {
                array_48.value().string(item_49.as_str());
            }
        }
        array_48.finish();
    }
    if let Some(var_50) = &input.publicly_accessible {
        object.key("publiclyAccessible").boolean(*var_50);
    }
    if let Some(var_51) = &input.tags {
        let mut array_52 = object.key("tags").start_array();
        for item_53 in var_51 {
             {
                let mut object_54 = array_52.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_54, item_53)?;
                object_54.finish();
            }
        }
        array_52.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_endpoint_access_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteEndpointAccessInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_55) = &input.endpoint_name {
        object.key("endpointName").string(var_55.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_namespace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteNamespaceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_56) = &input.namespace_name {
        object.key("namespaceName").string(var_56.as_str());
    }
    if let Some(var_57) = &input.final_snapshot_name {
        object.key("finalSnapshotName").string(var_57.as_str());
    }
    if let Some(var_58) = &input.final_snapshot_retention_period {
        object.key("finalSnapshotRetentionPeriod").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_58).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_resource_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteResourcePolicyInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_59) = &input.resource_arn {
        object.key("resourceArn").string(var_59.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_snapshot_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteSnapshotInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_60) = &input.snapshot_name {
        object.key("snapshotName").string(var_60.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_usage_limit_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteUsageLimitInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_61) = &input.usage_limit_id {
        object.key("usageLimitId").string(var_61.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_workgroup_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteWorkgroupInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_62) = &input.workgroup_name {
        object.key("workgroupName").string(var_62.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_credentials_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetCredentialsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_63) = &input.workgroup_name {
        object.key("workgroupName").string(var_63.as_str());
    }
    if let Some(var_64) = &input.db_name {
        object.key("dbName").string(var_64.as_str());
    }
    if let Some(var_65) = &input.duration_seconds {
        object.key("durationSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_65).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_endpoint_access_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetEndpointAccessInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_66) = &input.endpoint_name {
        object.key("endpointName").string(var_66.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_namespace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetNamespaceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_67) = &input.namespace_name {
        object.key("namespaceName").string(var_67.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_recovery_point_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetRecoveryPointInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_68) = &input.recovery_point_id {
        object.key("recoveryPointId").string(var_68.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_resource_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetResourcePolicyInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_69) = &input.resource_arn {
        object.key("resourceArn").string(var_69.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_snapshot_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetSnapshotInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_70) = &input.snapshot_name {
        object.key("snapshotName").string(var_70.as_str());
    }
    if let Some(var_71) = &input.owner_account {
        object.key("ownerAccount").string(var_71.as_str());
    }
    if let Some(var_72) = &input.snapshot_arn {
        object.key("snapshotArn").string(var_72.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_usage_limit_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetUsageLimitInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_73) = &input.usage_limit_id {
        object.key("usageLimitId").string(var_73.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_workgroup_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetWorkgroupInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_74) = &input.workgroup_name {
        object.key("workgroupName").string(var_74.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_endpoint_access_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListEndpointAccessInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_75) = &input.next_token {
        object.key("nextToken").string(var_75.as_str());
    }
    if let Some(var_76) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_76).into()));
    }
    if let Some(var_77) = &input.workgroup_name {
        object.key("workgroupName").string(var_77.as_str());
    }
    if let Some(var_78) = &input.vpc_id {
        object.key("vpcId").string(var_78.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_namespaces_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListNamespacesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.next_token {
        object.key("nextToken").string(var_79.as_str());
    }
    if let Some(var_80) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_80).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_recovery_points_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListRecoveryPointsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_81) = &input.next_token {
        object.key("nextToken").string(var_81.as_str());
    }
    if let Some(var_82) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_82).into()));
    }
    if let Some(var_83) = &input.start_time {
        object.key("startTime").date_time(var_83, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_84) = &input.end_time {
        object.key("endTime").date_time(var_84, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_85) = &input.namespace_name {
        object.key("namespaceName").string(var_85.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_snapshots_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListSnapshotsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_86) = &input.next_token {
        object.key("nextToken").string(var_86.as_str());
    }
    if let Some(var_87) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_87).into()));
    }
    if let Some(var_88) = &input.namespace_name {
        object.key("namespaceName").string(var_88.as_str());
    }
    if let Some(var_89) = &input.namespace_arn {
        object.key("namespaceArn").string(var_89.as_str());
    }
    if let Some(var_90) = &input.owner_account {
        object.key("ownerAccount").string(var_90.as_str());
    }
    if let Some(var_91) = &input.start_time {
        object.key("startTime").date_time(var_91, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_92) = &input.end_time {
        object.key("endTime").date_time(var_92, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTagsForResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_93) = &input.resource_arn {
        object.key("resourceArn").string(var_93.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_usage_limits_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListUsageLimitsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_94) = &input.resource_arn {
        object.key("resourceArn").string(var_94.as_str());
    }
    if let Some(var_95) = &input.usage_type {
        object.key("usageType").string(var_95.as_str());
    }
    if let Some(var_96) = &input.next_token {
        object.key("nextToken").string(var_96.as_str());
    }
    if let Some(var_97) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_97).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_workgroups_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListWorkgroupsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_98) = &input.next_token {
        object.key("nextToken").string(var_98.as_str());
    }
    if let Some(var_99) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_99).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_resource_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutResourcePolicyInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_100) = &input.resource_arn {
        object.key("resourceArn").string(var_100.as_str());
    }
    if let Some(var_101) = &input.policy {
        object.key("policy").string(var_101.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_restore_from_recovery_point_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RestoreFromRecoveryPointInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_102) = &input.recovery_point_id {
        object.key("recoveryPointId").string(var_102.as_str());
    }
    if let Some(var_103) = &input.namespace_name {
        object.key("namespaceName").string(var_103.as_str());
    }
    if let Some(var_104) = &input.workgroup_name {
        object.key("workgroupName").string(var_104.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_restore_from_snapshot_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RestoreFromSnapshotInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_105) = &input.namespace_name {
        object.key("namespaceName").string(var_105.as_str());
    }
    if let Some(var_106) = &input.workgroup_name {
        object.key("workgroupName").string(var_106.as_str());
    }
    if let Some(var_107) = &input.snapshot_name {
        object.key("snapshotName").string(var_107.as_str());
    }
    if let Some(var_108) = &input.snapshot_arn {
        object.key("snapshotArn").string(var_108.as_str());
    }
    if let Some(var_109) = &input.owner_account {
        object.key("ownerAccount").string(var_109.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_110) = &input.resource_arn {
        object.key("resourceArn").string(var_110.as_str());
    }
    if let Some(var_111) = &input.tags {
        let mut array_112 = object.key("tags").start_array();
        for item_113 in var_111 {
             {
                let mut object_114 = array_112.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_114, item_113)?;
                object_114.finish();
            }
        }
        array_112.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UntagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_115) = &input.resource_arn {
        object.key("resourceArn").string(var_115.as_str());
    }
    if let Some(var_116) = &input.tag_keys {
        let mut array_117 = object.key("tagKeys").start_array();
        for item_118 in var_116 {
             {
                array_117.value().string(item_118.as_str());
            }
        }
        array_117.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_endpoint_access_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateEndpointAccessInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_119) = &input.endpoint_name {
        object.key("endpointName").string(var_119.as_str());
    }
    if let Some(var_120) = &input.vpc_security_group_ids {
        let mut array_121 = object.key("vpcSecurityGroupIds").start_array();
        for item_122 in var_120 {
             {
                array_121.value().string(item_122.as_str());
            }
        }
        array_121.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_namespace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateNamespaceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_123) = &input.namespace_name {
        object.key("namespaceName").string(var_123.as_str());
    }
    if let Some(var_124) = &input.admin_user_password {
        object.key("adminUserPassword").string(var_124.as_str());
    }
    if let Some(var_125) = &input.admin_username {
        object.key("adminUsername").string(var_125.as_str());
    }
    if let Some(var_126) = &input.kms_key_id {
        object.key("kmsKeyId").string(var_126.as_str());
    }
    if let Some(var_127) = &input.default_iam_role_arn {
        object.key("defaultIamRoleArn").string(var_127.as_str());
    }
    if let Some(var_128) = &input.iam_roles {
        let mut array_129 = object.key("iamRoles").start_array();
        for item_130 in var_128 {
             {
                array_129.value().string(item_130.as_str());
            }
        }
        array_129.finish();
    }
    if let Some(var_131) = &input.log_exports {
        let mut array_132 = object.key("logExports").start_array();
        for item_133 in var_131 {
             {
                array_132.value().string(item_133.as_str());
            }
        }
        array_132.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_snapshot_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateSnapshotInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_134) = &input.snapshot_name {
        object.key("snapshotName").string(var_134.as_str());
    }
    if let Some(var_135) = &input.retention_period {
        object.key("retentionPeriod").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_135).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_usage_limit_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateUsageLimitInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_136) = &input.usage_limit_id {
        object.key("usageLimitId").string(var_136.as_str());
    }
    if let Some(var_137) = &input.amount {
        object.key("amount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_137).into()));
    }
    if let Some(var_138) = &input.breach_action {
        object.key("breachAction").string(var_138.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_workgroup_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateWorkgroupInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_139) = &input.workgroup_name {
        object.key("workgroupName").string(var_139.as_str());
    }
    if let Some(var_140) = &input.base_capacity {
        object.key("baseCapacity").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_140).into()));
    }
    if let Some(var_141) = &input.enhanced_vpc_routing {
        object.key("enhancedVpcRouting").boolean(*var_141);
    }
    if let Some(var_142) = &input.config_parameters {
        let mut array_143 = object.key("configParameters").start_array();
        for item_144 in var_142 {
             {
                let mut object_145 = array_143.value().start_object();
                crate::json_ser::serialize_structure_crate_model_config_parameter(&mut object_145, item_144)?;
                object_145.finish();
            }
        }
        array_143.finish();
    }
    if let Some(var_146) = &input.publicly_accessible {
        object.key("publiclyAccessible").boolean(*var_146);
    }
    if let Some(var_147) = &input.subnet_ids {
        let mut array_148 = object.key("subnetIds").start_array();
        for item_149 in var_147 {
             {
                array_148.value().string(item_149.as_str());
            }
        }
        array_148.finish();
    }
    if let Some(var_150) = &input.security_group_ids {
        let mut array_151 = object.key("securityGroupIds").start_array();
        for item_152 in var_150 {
             {
                array_151.value().string(item_152.as_str());
            }
        }
        array_151.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Tag) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_153) = &input.key {
        object.key("key").string(var_153.as_str());
    }
    if let Some(var_154) = &input.value {
        object.key("value").string(var_154.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_config_parameter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ConfigParameter) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_155) = &input.parameter_key {
        object.key("parameterKey").string(var_155.as_str());
    }
    if let Some(var_156) = &input.parameter_value {
        object.key("parameterValue").string(var_156.as_str());
    }
    Ok(())
}

