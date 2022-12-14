// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_http_namespace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateHttpNamespaceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.creator_request_id {
        object.key("CreatorRequestId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.tags {
        let mut array_5 = object.key("Tags").start_array();
        for item_6 in var_4 {
             {
                let mut object_7 = array_5.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_private_dns_namespace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreatePrivateDnsNamespaceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_8) = &input.name {
        object.key("Name").string(var_8.as_str());
    }
    if let Some(var_9) = &input.creator_request_id {
        object.key("CreatorRequestId").string(var_9.as_str());
    }
    if let Some(var_10) = &input.description {
        object.key("Description").string(var_10.as_str());
    }
    if let Some(var_11) = &input.vpc {
        object.key("Vpc").string(var_11.as_str());
    }
    if let Some(var_12) = &input.tags {
        let mut array_13 = object.key("Tags").start_array();
        for item_14 in var_12 {
             {
                let mut object_15 = array_13.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_15, item_14)?;
                object_15.finish();
            }
        }
        array_13.finish();
    }
    if let Some(var_16) = &input.properties {
        let mut object_17 = object.key("Properties").start_object();
        crate::json_ser::serialize_structure_crate_model_private_dns_namespace_properties(&mut object_17, var_16)?;
        object_17.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_public_dns_namespace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreatePublicDnsNamespaceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_18) = &input.name {
        object.key("Name").string(var_18.as_str());
    }
    if let Some(var_19) = &input.creator_request_id {
        object.key("CreatorRequestId").string(var_19.as_str());
    }
    if let Some(var_20) = &input.description {
        object.key("Description").string(var_20.as_str());
    }
    if let Some(var_21) = &input.tags {
        let mut array_22 = object.key("Tags").start_array();
        for item_23 in var_21 {
             {
                let mut object_24 = array_22.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_24, item_23)?;
                object_24.finish();
            }
        }
        array_22.finish();
    }
    if let Some(var_25) = &input.properties {
        let mut object_26 = object.key("Properties").start_object();
        crate::json_ser::serialize_structure_crate_model_public_dns_namespace_properties(&mut object_26, var_25)?;
        object_26.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_service_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateServiceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_27) = &input.name {
        object.key("Name").string(var_27.as_str());
    }
    if let Some(var_28) = &input.namespace_id {
        object.key("NamespaceId").string(var_28.as_str());
    }
    if let Some(var_29) = &input.creator_request_id {
        object.key("CreatorRequestId").string(var_29.as_str());
    }
    if let Some(var_30) = &input.description {
        object.key("Description").string(var_30.as_str());
    }
    if let Some(var_31) = &input.dns_config {
        let mut object_32 = object.key("DnsConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_dns_config(&mut object_32, var_31)?;
        object_32.finish();
    }
    if let Some(var_33) = &input.health_check_config {
        let mut object_34 = object.key("HealthCheckConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_health_check_config(&mut object_34, var_33)?;
        object_34.finish();
    }
    if let Some(var_35) = &input.health_check_custom_config {
        let mut object_36 = object.key("HealthCheckCustomConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_health_check_custom_config(&mut object_36, var_35)?;
        object_36.finish();
    }
    if let Some(var_37) = &input.tags {
        let mut array_38 = object.key("Tags").start_array();
        for item_39 in var_37 {
             {
                let mut object_40 = array_38.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_40, item_39)?;
                object_40.finish();
            }
        }
        array_38.finish();
    }
    if let Some(var_41) = &input.r#type {
        object.key("Type").string(var_41.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_namespace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteNamespaceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_42) = &input.id {
        object.key("Id").string(var_42.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_service_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteServiceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_43) = &input.id {
        object.key("Id").string(var_43.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_deregister_instance_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeregisterInstanceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.service_id {
        object.key("ServiceId").string(var_44.as_str());
    }
    if let Some(var_45) = &input.instance_id {
        object.key("InstanceId").string(var_45.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_discover_instances_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DiscoverInstancesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.namespace_name {
        object.key("NamespaceName").string(var_46.as_str());
    }
    if let Some(var_47) = &input.service_name {
        object.key("ServiceName").string(var_47.as_str());
    }
    if let Some(var_48) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_48).into()));
    }
    if let Some(var_49) = &input.query_parameters {
        let mut object_50 = object.key("QueryParameters").start_object();
        for (key_51, value_52) in var_49 {
             {
                object_50.key(key_51).string(value_52.as_str());
            }
        }
        object_50.finish();
    }
    if let Some(var_53) = &input.optional_parameters {
        let mut object_54 = object.key("OptionalParameters").start_object();
        for (key_55, value_56) in var_53 {
             {
                object_54.key(key_55).string(value_56.as_str());
            }
        }
        object_54.finish();
    }
    if let Some(var_57) = &input.health_status {
        object.key("HealthStatus").string(var_57.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_instance_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetInstanceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_58) = &input.service_id {
        object.key("ServiceId").string(var_58.as_str());
    }
    if let Some(var_59) = &input.instance_id {
        object.key("InstanceId").string(var_59.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_instances_health_status_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetInstancesHealthStatusInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_60) = &input.service_id {
        object.key("ServiceId").string(var_60.as_str());
    }
    if let Some(var_61) = &input.instances {
        let mut array_62 = object.key("Instances").start_array();
        for item_63 in var_61 {
             {
                array_62.value().string(item_63.as_str());
            }
        }
        array_62.finish();
    }
    if let Some(var_64) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_64).into()));
    }
    if let Some(var_65) = &input.next_token {
        object.key("NextToken").string(var_65.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_namespace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetNamespaceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_66) = &input.id {
        object.key("Id").string(var_66.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_operation_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetOperationInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_67) = &input.operation_id {
        object.key("OperationId").string(var_67.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_service_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetServiceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_68) = &input.id {
        object.key("Id").string(var_68.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_instances_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListInstancesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_69) = &input.service_id {
        object.key("ServiceId").string(var_69.as_str());
    }
    if let Some(var_70) = &input.next_token {
        object.key("NextToken").string(var_70.as_str());
    }
    if let Some(var_71) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_71).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_namespaces_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListNamespacesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_72) = &input.next_token {
        object.key("NextToken").string(var_72.as_str());
    }
    if let Some(var_73) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_73).into()));
    }
    if let Some(var_74) = &input.filters {
        let mut array_75 = object.key("Filters").start_array();
        for item_76 in var_74 {
             {
                let mut object_77 = array_75.value().start_object();
                crate::json_ser::serialize_structure_crate_model_namespace_filter(&mut object_77, item_76)?;
                object_77.finish();
            }
        }
        array_75.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_operations_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListOperationsInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_78) = &input.next_token {
        object.key("NextToken").string(var_78.as_str());
    }
    if let Some(var_79) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_79).into()));
    }
    if let Some(var_80) = &input.filters {
        let mut array_81 = object.key("Filters").start_array();
        for item_82 in var_80 {
             {
                let mut object_83 = array_81.value().start_object();
                crate::json_ser::serialize_structure_crate_model_operation_filter(&mut object_83, item_82)?;
                object_83.finish();
            }
        }
        array_81.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_services_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListServicesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_84) = &input.next_token {
        object.key("NextToken").string(var_84.as_str());
    }
    if let Some(var_85) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_85).into()));
    }
    if let Some(var_86) = &input.filters {
        let mut array_87 = object.key("Filters").start_array();
        for item_88 in var_86 {
             {
                let mut object_89 = array_87.value().start_object();
                crate::json_ser::serialize_structure_crate_model_service_filter(&mut object_89, item_88)?;
                object_89.finish();
            }
        }
        array_87.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTagsForResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_90) = &input.resource_arn {
        object.key("ResourceARN").string(var_90.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_register_instance_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RegisterInstanceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_91) = &input.service_id {
        object.key("ServiceId").string(var_91.as_str());
    }
    if let Some(var_92) = &input.instance_id {
        object.key("InstanceId").string(var_92.as_str());
    }
    if let Some(var_93) = &input.creator_request_id {
        object.key("CreatorRequestId").string(var_93.as_str());
    }
    if let Some(var_94) = &input.attributes {
        let mut object_95 = object.key("Attributes").start_object();
        for (key_96, value_97) in var_94 {
             {
                object_95.key(key_96).string(value_97.as_str());
            }
        }
        object_95.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_98) = &input.resource_arn {
        object.key("ResourceARN").string(var_98.as_str());
    }
    if let Some(var_99) = &input.tags {
        let mut array_100 = object.key("Tags").start_array();
        for item_101 in var_99 {
             {
                let mut object_102 = array_100.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_102, item_101)?;
                object_102.finish();
            }
        }
        array_100.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UntagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_103) = &input.resource_arn {
        object.key("ResourceARN").string(var_103.as_str());
    }
    if let Some(var_104) = &input.tag_keys {
        let mut array_105 = object.key("TagKeys").start_array();
        for item_106 in var_104 {
             {
                array_105.value().string(item_106.as_str());
            }
        }
        array_105.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_http_namespace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateHttpNamespaceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_107) = &input.id {
        object.key("Id").string(var_107.as_str());
    }
    if let Some(var_108) = &input.updater_request_id {
        object.key("UpdaterRequestId").string(var_108.as_str());
    }
    if let Some(var_109) = &input.namespace {
        let mut object_110 = object.key("Namespace").start_object();
        crate::json_ser::serialize_structure_crate_model_http_namespace_change(&mut object_110, var_109)?;
        object_110.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_instance_custom_health_status_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateInstanceCustomHealthStatusInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_111) = &input.service_id {
        object.key("ServiceId").string(var_111.as_str());
    }
    if let Some(var_112) = &input.instance_id {
        object.key("InstanceId").string(var_112.as_str());
    }
    if let Some(var_113) = &input.status {
        object.key("Status").string(var_113.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_private_dns_namespace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdatePrivateDnsNamespaceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_114) = &input.id {
        object.key("Id").string(var_114.as_str());
    }
    if let Some(var_115) = &input.updater_request_id {
        object.key("UpdaterRequestId").string(var_115.as_str());
    }
    if let Some(var_116) = &input.namespace {
        let mut object_117 = object.key("Namespace").start_object();
        crate::json_ser::serialize_structure_crate_model_private_dns_namespace_change(&mut object_117, var_116)?;
        object_117.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_public_dns_namespace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdatePublicDnsNamespaceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_118) = &input.id {
        object.key("Id").string(var_118.as_str());
    }
    if let Some(var_119) = &input.updater_request_id {
        object.key("UpdaterRequestId").string(var_119.as_str());
    }
    if let Some(var_120) = &input.namespace {
        let mut object_121 = object.key("Namespace").start_object();
        crate::json_ser::serialize_structure_crate_model_public_dns_namespace_change(&mut object_121, var_120)?;
        object_121.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_service_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateServiceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_122) = &input.id {
        object.key("Id").string(var_122.as_str());
    }
    if let Some(var_123) = &input.service {
        let mut object_124 = object.key("Service").start_object();
        crate::json_ser::serialize_structure_crate_model_service_change(&mut object_124, var_123)?;
        object_124.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Tag) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_125) = &input.key {
        object.key("Key").string(var_125.as_str());
    }
    if let Some(var_126) = &input.value {
        object.key("Value").string(var_126.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_private_dns_namespace_properties(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PrivateDnsNamespaceProperties) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_127) = &input.dns_properties {
        let mut object_128 = object.key("DnsProperties").start_object();
        crate::json_ser::serialize_structure_crate_model_private_dns_properties_mutable(&mut object_128, var_127)?;
        object_128.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_public_dns_namespace_properties(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PublicDnsNamespaceProperties) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_129) = &input.dns_properties {
        let mut object_130 = object.key("DnsProperties").start_object();
        crate::json_ser::serialize_structure_crate_model_public_dns_properties_mutable(&mut object_130, var_129)?;
        object_130.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dns_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DnsConfig) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_131) = &input.namespace_id {
        object.key("NamespaceId").string(var_131.as_str());
    }
    if let Some(var_132) = &input.routing_policy {
        object.key("RoutingPolicy").string(var_132.as_str());
    }
    if let Some(var_133) = &input.dns_records {
        let mut array_134 = object.key("DnsRecords").start_array();
        for item_135 in var_133 {
             {
                let mut object_136 = array_134.value().start_object();
                crate::json_ser::serialize_structure_crate_model_dns_record(&mut object_136, item_135)?;
                object_136.finish();
            }
        }
        array_134.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_health_check_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::HealthCheckConfig) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_137) = &input.r#type {
        object.key("Type").string(var_137.as_str());
    }
    if let Some(var_138) = &input.resource_path {
        object.key("ResourcePath").string(var_138.as_str());
    }
    if let Some(var_139) = &input.failure_threshold {
        object.key("FailureThreshold").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_139).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_health_check_custom_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::HealthCheckCustomConfig) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_140) = &input.failure_threshold {
        object.key("FailureThreshold").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_140).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_namespace_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NamespaceFilter) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_141) = &input.name {
        object.key("Name").string(var_141.as_str());
    }
    if let Some(var_142) = &input.values {
        let mut array_143 = object.key("Values").start_array();
        for item_144 in var_142 {
             {
                array_143.value().string(item_144.as_str());
            }
        }
        array_143.finish();
    }
    if let Some(var_145) = &input.condition {
        object.key("Condition").string(var_145.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_operation_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::OperationFilter) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_146) = &input.name {
        object.key("Name").string(var_146.as_str());
    }
    if let Some(var_147) = &input.values {
        let mut array_148 = object.key("Values").start_array();
        for item_149 in var_147 {
             {
                array_148.value().string(item_149.as_str());
            }
        }
        array_148.finish();
    }
    if let Some(var_150) = &input.condition {
        object.key("Condition").string(var_150.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_service_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ServiceFilter) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_151) = &input.name {
        object.key("Name").string(var_151.as_str());
    }
    if let Some(var_152) = &input.values {
        let mut array_153 = object.key("Values").start_array();
        for item_154 in var_152 {
             {
                array_153.value().string(item_154.as_str());
            }
        }
        array_153.finish();
    }
    if let Some(var_155) = &input.condition {
        object.key("Condition").string(var_155.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_http_namespace_change(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::HttpNamespaceChange) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_156) = &input.description {
        object.key("Description").string(var_156.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_private_dns_namespace_change(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PrivateDnsNamespaceChange) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_157) = &input.description {
        object.key("Description").string(var_157.as_str());
    }
    if let Some(var_158) = &input.properties {
        let mut object_159 = object.key("Properties").start_object();
        crate::json_ser::serialize_structure_crate_model_private_dns_namespace_properties_change(&mut object_159, var_158)?;
        object_159.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_public_dns_namespace_change(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PublicDnsNamespaceChange) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_160) = &input.description {
        object.key("Description").string(var_160.as_str());
    }
    if let Some(var_161) = &input.properties {
        let mut object_162 = object.key("Properties").start_object();
        crate::json_ser::serialize_structure_crate_model_public_dns_namespace_properties_change(&mut object_162, var_161)?;
        object_162.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_service_change(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ServiceChange) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_163) = &input.description {
        object.key("Description").string(var_163.as_str());
    }
    if let Some(var_164) = &input.dns_config {
        let mut object_165 = object.key("DnsConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_dns_config_change(&mut object_165, var_164)?;
        object_165.finish();
    }
    if let Some(var_166) = &input.health_check_config {
        let mut object_167 = object.key("HealthCheckConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_health_check_config(&mut object_167, var_166)?;
        object_167.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_private_dns_properties_mutable(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PrivateDnsPropertiesMutable) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_168) = &input.soa {
        let mut object_169 = object.key("SOA").start_object();
        crate::json_ser::serialize_structure_crate_model_soa(&mut object_169, var_168)?;
        object_169.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_public_dns_properties_mutable(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PublicDnsPropertiesMutable) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_170) = &input.soa {
        let mut object_171 = object.key("SOA").start_object();
        crate::json_ser::serialize_structure_crate_model_soa(&mut object_171, var_170)?;
        object_171.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dns_record(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DnsRecord) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_172) = &input.r#type {
        object.key("Type").string(var_172.as_str());
    }
    if let Some(var_173) = &input.ttl {
        object.key("TTL").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_173).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_private_dns_namespace_properties_change(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PrivateDnsNamespacePropertiesChange) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_174) = &input.dns_properties {
        let mut object_175 = object.key("DnsProperties").start_object();
        crate::json_ser::serialize_structure_crate_model_private_dns_properties_mutable_change(&mut object_175, var_174)?;
        object_175.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_public_dns_namespace_properties_change(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PublicDnsNamespacePropertiesChange) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_176) = &input.dns_properties {
        let mut object_177 = object.key("DnsProperties").start_object();
        crate::json_ser::serialize_structure_crate_model_public_dns_properties_mutable_change(&mut object_177, var_176)?;
        object_177.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dns_config_change(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DnsConfigChange) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_178) = &input.dns_records {
        let mut array_179 = object.key("DnsRecords").start_array();
        for item_180 in var_178 {
             {
                let mut object_181 = array_179.value().start_object();
                crate::json_ser::serialize_structure_crate_model_dns_record(&mut object_181, item_180)?;
                object_181.finish();
            }
        }
        array_179.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_soa(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Soa) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_182) = &input.ttl {
        object.key("TTL").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_182).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_private_dns_properties_mutable_change(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PrivateDnsPropertiesMutableChange) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_183) = &input.soa {
        let mut object_184 = object.key("SOA").start_object();
        crate::json_ser::serialize_structure_crate_model_soa_change(&mut object_184, var_183)?;
        object_184.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_public_dns_properties_mutable_change(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PublicDnsPropertiesMutableChange) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_185) = &input.soa {
        let mut object_186 = object.key("SOA").start_object();
        crate::json_ser::serialize_structure_crate_model_soa_change(&mut object_186, var_185)?;
        object_186.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_soa_change(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SoaChange) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_187) = &input.ttl {
        object.key("TTL").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_187).into()));
    }
    Ok(())
}

