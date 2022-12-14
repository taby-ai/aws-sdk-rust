// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_managed_endpoint_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateManagedEndpointInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.certificate_arn {
        object.key("certificateArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_token {
        object.key("clientToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.configuration_overrides {
        let mut object_4 = object.key("configurationOverrides").start_object();
        crate::json_ser::serialize_structure_crate_model_configuration_overrides(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.execution_role_arn {
        object.key("executionRoleArn").string(var_5.as_str());
    }
    if let Some(var_6) = &input.name {
        object.key("name").string(var_6.as_str());
    }
    if let Some(var_7) = &input.release_label {
        object.key("releaseLabel").string(var_7.as_str());
    }
    if let Some(var_8) = &input.tags {
        let mut object_9 = object.key("tags").start_object();
        for (key_10, value_11) in var_8 {
             {
                object_9.key(key_10).string(value_11.as_str());
            }
        }
        object_9.finish();
    }
    if let Some(var_12) = &input.r#type {
        object.key("type").string(var_12.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_virtual_cluster_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateVirtualClusterInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_13) = &input.client_token {
        object.key("clientToken").string(var_13.as_str());
    }
    if let Some(var_14) = &input.container_provider {
        let mut object_15 = object.key("containerProvider").start_object();
        crate::json_ser::serialize_structure_crate_model_container_provider(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.name {
        object.key("name").string(var_16.as_str());
    }
    if let Some(var_17) = &input.tags {
        let mut object_18 = object.key("tags").start_object();
        for (key_19, value_20) in var_17 {
             {
                object_18.key(key_19).string(value_20.as_str());
            }
        }
        object_18.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_job_run_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartJobRunInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_21) = &input.client_token {
        object.key("clientToken").string(var_21.as_str());
    }
    if let Some(var_22) = &input.configuration_overrides {
        let mut object_23 = object.key("configurationOverrides").start_object();
        crate::json_ser::serialize_structure_crate_model_configuration_overrides(&mut object_23, var_22)?;
        object_23.finish();
    }
    if let Some(var_24) = &input.execution_role_arn {
        object.key("executionRoleArn").string(var_24.as_str());
    }
    if let Some(var_25) = &input.job_driver {
        let mut object_26 = object.key("jobDriver").start_object();
        crate::json_ser::serialize_structure_crate_model_job_driver(&mut object_26, var_25)?;
        object_26.finish();
    }
    if let Some(var_27) = &input.name {
        object.key("name").string(var_27.as_str());
    }
    if let Some(var_28) = &input.release_label {
        object.key("releaseLabel").string(var_28.as_str());
    }
    if let Some(var_29) = &input.tags {
        let mut object_30 = object.key("tags").start_object();
        for (key_31, value_32) in var_29 {
             {
                object_30.key(key_31).string(value_32.as_str());
            }
        }
        object_30.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_33) = &input.tags {
        let mut object_34 = object.key("tags").start_object();
        for (key_35, value_36) in var_33 {
             {
                object_34.key(key_35).string(value_36.as_str());
            }
        }
        object_34.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_configuration_overrides(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ConfigurationOverrides) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_37) = &input.application_configuration {
        let mut array_38 = object.key("applicationConfiguration").start_array();
        for item_39 in var_37 {
             {
                let mut object_40 = array_38.value().start_object();
                crate::json_ser::serialize_structure_crate_model_configuration(&mut object_40, item_39)?;
                object_40.finish();
            }
        }
        array_38.finish();
    }
    if let Some(var_41) = &input.monitoring_configuration {
        let mut object_42 = object.key("monitoringConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_monitoring_configuration(&mut object_42, var_41)?;
        object_42.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_container_provider(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ContainerProvider) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_43) = &input.r#type {
        object.key("type").string(var_43.as_str());
    }
    if let Some(var_44) = &input.id {
        object.key("id").string(var_44.as_str());
    }
    if let Some(var_45) = &input.info {
        let mut object_46 = object.key("info").start_object();
        crate::json_ser::serialize_union_crate_model_container_info(&mut object_46, var_45)?;
        object_46.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_job_driver(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::JobDriver) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_47) = &input.spark_submit_job_driver {
        let mut object_48 = object.key("sparkSubmitJobDriver").start_object();
        crate::json_ser::serialize_structure_crate_model_spark_submit_job_driver(&mut object_48, var_47)?;
        object_48.finish();
    }
    if let Some(var_49) = &input.spark_sql_job_driver {
        let mut object_50 = object.key("sparkSqlJobDriver").start_object();
        crate::json_ser::serialize_structure_crate_model_spark_sql_job_driver(&mut object_50, var_49)?;
        object_50.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Configuration) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_51) = &input.classification {
        object.key("classification").string(var_51.as_str());
    }
    if let Some(var_52) = &input.properties {
        let mut object_53 = object.key("properties").start_object();
        for (key_54, value_55) in var_52 {
             {
                object_53.key(key_54).string(value_55.as_str());
            }
        }
        object_53.finish();
    }
    if let Some(var_56) = &input.configurations {
        let mut array_57 = object.key("configurations").start_array();
        for item_58 in var_56 {
             {
                let mut object_59 = array_57.value().start_object();
                crate::json_ser::serialize_structure_crate_model_configuration(&mut object_59, item_58)?;
                object_59.finish();
            }
        }
        array_57.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_monitoring_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MonitoringConfiguration) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_60) = &input.persistent_app_ui {
        object.key("persistentAppUI").string(var_60.as_str());
    }
    if let Some(var_61) = &input.cloud_watch_monitoring_configuration {
        let mut object_62 = object.key("cloudWatchMonitoringConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_cloud_watch_monitoring_configuration(&mut object_62, var_61)?;
        object_62.finish();
    }
    if let Some(var_63) = &input.s3_monitoring_configuration {
        let mut object_64 = object.key("s3MonitoringConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_monitoring_configuration(&mut object_64, var_63)?;
        object_64.finish();
    }
    Ok(())
}

pub fn serialize_union_crate_model_container_info(object_46: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ContainerInfo) -> Result<(), aws_smithy_http::operation::SerializationError> {
    match input {
        crate::model::ContainerInfo::EksInfo(inner) => {
             {
                let mut object_65 = object_46.key("eksInfo").start_object();
                crate::json_ser::serialize_structure_crate_model_eks_info(&mut object_65, inner)?;
                object_65.finish();
            }
        },
        crate::model::ContainerInfo::Unknown => return Err(aws_smithy_http::operation::SerializationError::unknown_variant("ContainerInfo"))
    }
    Ok(())
}

pub fn serialize_structure_crate_model_spark_submit_job_driver(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SparkSubmitJobDriver) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_66) = &input.entry_point {
        object.key("entryPoint").string(var_66.as_str());
    }
    if let Some(var_67) = &input.entry_point_arguments {
        let mut array_68 = object.key("entryPointArguments").start_array();
        for item_69 in var_67 {
             {
                array_68.value().string(item_69.as_str());
            }
        }
        array_68.finish();
    }
    if let Some(var_70) = &input.spark_submit_parameters {
        object.key("sparkSubmitParameters").string(var_70.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_spark_sql_job_driver(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SparkSqlJobDriver) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_71) = &input.entry_point {
        object.key("entryPoint").string(var_71.as_str());
    }
    if let Some(var_72) = &input.spark_sql_parameters {
        object.key("sparkSqlParameters").string(var_72.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_cloud_watch_monitoring_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CloudWatchMonitoringConfiguration) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_73) = &input.log_group_name {
        object.key("logGroupName").string(var_73.as_str());
    }
    if let Some(var_74) = &input.log_stream_name_prefix {
        object.key("logStreamNamePrefix").string(var_74.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_monitoring_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::S3MonitoringConfiguration) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_75) = &input.log_uri {
        object.key("logUri").string(var_75.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_eks_info(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EksInfo) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_76) = &input.namespace {
        object.key("namespace").string(var_76.as_str());
    }
    Ok(())
}

