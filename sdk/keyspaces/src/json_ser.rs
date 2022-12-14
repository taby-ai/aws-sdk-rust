// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_keyspace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateKeyspaceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.keyspace_name {
        object.key("keyspaceName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tags {
        let mut array_3 = object.key("tags").start_array();
        for item_4 in var_2 {
             {
                let mut object_5 = array_3.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_table_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateTableInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_6) = &input.keyspace_name {
        object.key("keyspaceName").string(var_6.as_str());
    }
    if let Some(var_7) = &input.table_name {
        object.key("tableName").string(var_7.as_str());
    }
    if let Some(var_8) = &input.schema_definition {
        let mut object_9 = object.key("schemaDefinition").start_object();
        crate::json_ser::serialize_structure_crate_model_schema_definition(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.comment {
        let mut object_11 = object.key("comment").start_object();
        crate::json_ser::serialize_structure_crate_model_comment(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.capacity_specification {
        let mut object_13 = object.key("capacitySpecification").start_object();
        crate::json_ser::serialize_structure_crate_model_capacity_specification(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.encryption_specification {
        let mut object_15 = object.key("encryptionSpecification").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_specification(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.point_in_time_recovery {
        let mut object_17 = object.key("pointInTimeRecovery").start_object();
        crate::json_ser::serialize_structure_crate_model_point_in_time_recovery(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.ttl {
        let mut object_19 = object.key("ttl").start_object();
        crate::json_ser::serialize_structure_crate_model_time_to_live(&mut object_19, var_18)?;
        object_19.finish();
    }
    if let Some(var_20) = &input.default_time_to_live {
        object.key("defaultTimeToLive").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_20).into()));
    }
    if let Some(var_21) = &input.tags {
        let mut array_22 = object.key("tags").start_array();
        for item_23 in var_21 {
             {
                let mut object_24 = array_22.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_24, item_23)?;
                object_24.finish();
            }
        }
        array_22.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_keyspace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteKeyspaceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_25) = &input.keyspace_name {
        object.key("keyspaceName").string(var_25.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_table_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteTableInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_26) = &input.keyspace_name {
        object.key("keyspaceName").string(var_26.as_str());
    }
    if let Some(var_27) = &input.table_name {
        object.key("tableName").string(var_27.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_keyspace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetKeyspaceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.keyspace_name {
        object.key("keyspaceName").string(var_28.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_table_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetTableInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_29) = &input.keyspace_name {
        object.key("keyspaceName").string(var_29.as_str());
    }
    if let Some(var_30) = &input.table_name {
        object.key("tableName").string(var_30.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_keyspaces_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListKeyspacesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_31) = &input.next_token {
        object.key("nextToken").string(var_31.as_str());
    }
    if let Some(var_32) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_32).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tables_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTablesInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.next_token {
        object.key("nextToken").string(var_33.as_str());
    }
    if let Some(var_34) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_34).into()));
    }
    if let Some(var_35) = &input.keyspace_name {
        object.key("keyspaceName").string(var_35.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTagsForResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_36) = &input.resource_arn {
        object.key("resourceArn").string(var_36.as_str());
    }
    if let Some(var_37) = &input.next_token {
        object.key("nextToken").string(var_37.as_str());
    }
    if let Some(var_38) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_38).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_restore_table_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RestoreTableInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_39) = &input.source_keyspace_name {
        object.key("sourceKeyspaceName").string(var_39.as_str());
    }
    if let Some(var_40) = &input.source_table_name {
        object.key("sourceTableName").string(var_40.as_str());
    }
    if let Some(var_41) = &input.target_keyspace_name {
        object.key("targetKeyspaceName").string(var_41.as_str());
    }
    if let Some(var_42) = &input.target_table_name {
        object.key("targetTableName").string(var_42.as_str());
    }
    if let Some(var_43) = &input.restore_timestamp {
        object.key("restoreTimestamp").date_time(var_43, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_44) = &input.capacity_specification_override {
        let mut object_45 = object.key("capacitySpecificationOverride").start_object();
        crate::json_ser::serialize_structure_crate_model_capacity_specification(&mut object_45, var_44)?;
        object_45.finish();
    }
    if let Some(var_46) = &input.encryption_specification_override {
        let mut object_47 = object.key("encryptionSpecificationOverride").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_specification(&mut object_47, var_46)?;
        object_47.finish();
    }
    if let Some(var_48) = &input.point_in_time_recovery_override {
        let mut object_49 = object.key("pointInTimeRecoveryOverride").start_object();
        crate::json_ser::serialize_structure_crate_model_point_in_time_recovery(&mut object_49, var_48)?;
        object_49.finish();
    }
    if let Some(var_50) = &input.tags_override {
        let mut array_51 = object.key("tagsOverride").start_array();
        for item_52 in var_50 {
             {
                let mut object_53 = array_51.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_53, item_52)?;
                object_53.finish();
            }
        }
        array_51.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_54) = &input.resource_arn {
        object.key("resourceArn").string(var_54.as_str());
    }
    if let Some(var_55) = &input.tags {
        let mut array_56 = object.key("tags").start_array();
        for item_57 in var_55 {
             {
                let mut object_58 = array_56.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_58, item_57)?;
                object_58.finish();
            }
        }
        array_56.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UntagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_59) = &input.resource_arn {
        object.key("resourceArn").string(var_59.as_str());
    }
    if let Some(var_60) = &input.tags {
        let mut array_61 = object.key("tags").start_array();
        for item_62 in var_60 {
             {
                let mut object_63 = array_61.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_63, item_62)?;
                object_63.finish();
            }
        }
        array_61.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_table_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateTableInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_64) = &input.keyspace_name {
        object.key("keyspaceName").string(var_64.as_str());
    }
    if let Some(var_65) = &input.table_name {
        object.key("tableName").string(var_65.as_str());
    }
    if let Some(var_66) = &input.add_columns {
        let mut array_67 = object.key("addColumns").start_array();
        for item_68 in var_66 {
             {
                let mut object_69 = array_67.value().start_object();
                crate::json_ser::serialize_structure_crate_model_column_definition(&mut object_69, item_68)?;
                object_69.finish();
            }
        }
        array_67.finish();
    }
    if let Some(var_70) = &input.capacity_specification {
        let mut object_71 = object.key("capacitySpecification").start_object();
        crate::json_ser::serialize_structure_crate_model_capacity_specification(&mut object_71, var_70)?;
        object_71.finish();
    }
    if let Some(var_72) = &input.encryption_specification {
        let mut object_73 = object.key("encryptionSpecification").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_specification(&mut object_73, var_72)?;
        object_73.finish();
    }
    if let Some(var_74) = &input.point_in_time_recovery {
        let mut object_75 = object.key("pointInTimeRecovery").start_object();
        crate::json_ser::serialize_structure_crate_model_point_in_time_recovery(&mut object_75, var_74)?;
        object_75.finish();
    }
    if let Some(var_76) = &input.ttl {
        let mut object_77 = object.key("ttl").start_object();
        crate::json_ser::serialize_structure_crate_model_time_to_live(&mut object_77, var_76)?;
        object_77.finish();
    }
    if let Some(var_78) = &input.default_time_to_live {
        object.key("defaultTimeToLive").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_78).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Tag) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_79) = &input.key {
        object.key("key").string(var_79.as_str());
    }
    if let Some(var_80) = &input.value {
        object.key("value").string(var_80.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_schema_definition(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SchemaDefinition) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_81) = &input.all_columns {
        let mut array_82 = object.key("allColumns").start_array();
        for item_83 in var_81 {
             {
                let mut object_84 = array_82.value().start_object();
                crate::json_ser::serialize_structure_crate_model_column_definition(&mut object_84, item_83)?;
                object_84.finish();
            }
        }
        array_82.finish();
    }
    if let Some(var_85) = &input.partition_keys {
        let mut array_86 = object.key("partitionKeys").start_array();
        for item_87 in var_85 {
             {
                let mut object_88 = array_86.value().start_object();
                crate::json_ser::serialize_structure_crate_model_partition_key(&mut object_88, item_87)?;
                object_88.finish();
            }
        }
        array_86.finish();
    }
    if let Some(var_89) = &input.clustering_keys {
        let mut array_90 = object.key("clusteringKeys").start_array();
        for item_91 in var_89 {
             {
                let mut object_92 = array_90.value().start_object();
                crate::json_ser::serialize_structure_crate_model_clustering_key(&mut object_92, item_91)?;
                object_92.finish();
            }
        }
        array_90.finish();
    }
    if let Some(var_93) = &input.static_columns {
        let mut array_94 = object.key("staticColumns").start_array();
        for item_95 in var_93 {
             {
                let mut object_96 = array_94.value().start_object();
                crate::json_ser::serialize_structure_crate_model_static_column(&mut object_96, item_95)?;
                object_96.finish();
            }
        }
        array_94.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_comment(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Comment) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_97) = &input.message {
        object.key("message").string(var_97.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_capacity_specification(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CapacitySpecification) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_98) = &input.throughput_mode {
        object.key("throughputMode").string(var_98.as_str());
    }
    if let Some(var_99) = &input.read_capacity_units {
        object.key("readCapacityUnits").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_99).into()));
    }
    if let Some(var_100) = &input.write_capacity_units {
        object.key("writeCapacityUnits").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_100).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_encryption_specification(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EncryptionSpecification) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_101) = &input.r#type {
        object.key("type").string(var_101.as_str());
    }
    if let Some(var_102) = &input.kms_key_identifier {
        object.key("kmsKeyIdentifier").string(var_102.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_point_in_time_recovery(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PointInTimeRecovery) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_103) = &input.status {
        object.key("status").string(var_103.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_time_to_live(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TimeToLive) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_104) = &input.status {
        object.key("status").string(var_104.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_column_definition(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ColumnDefinition) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_105) = &input.name {
        object.key("name").string(var_105.as_str());
    }
    if let Some(var_106) = &input.r#type {
        object.key("type").string(var_106.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_partition_key(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PartitionKey) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_107) = &input.name {
        object.key("name").string(var_107.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_clustering_key(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ClusteringKey) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_108) = &input.name {
        object.key("name").string(var_108.as_str());
    }
    if let Some(var_109) = &input.order_by {
        object.key("orderBy").string(var_109.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_static_column(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StaticColumn) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_110) = &input.name {
        object.key("name").string(var_110.as_str());
    }
    Ok(())
}

