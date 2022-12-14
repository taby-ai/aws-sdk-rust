// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_describe_objects_output_marker(input: &crate::output::DescribeObjectsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.marker {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_pipelines_output_marker(input: &crate::output::ListPipelinesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.marker {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_query_objects_output_marker(input: &crate::output::QueryObjectsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.marker {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_describe_objects_output_pipeline_objects(input: crate::output::DescribeObjectsOutput) -> std::option::Option<std::vec::Vec<crate::model::PipelineObject>> {
                    let input = match input.pipeline_objects {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_pipelines_output_pipeline_id_list(input: crate::output::ListPipelinesOutput) -> std::option::Option<std::vec::Vec<crate::model::PipelineIdName>> {
                    let input = match input.pipeline_id_list {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_query_objects_output_ids(input: crate::output::QueryObjectsOutput) -> std::option::Option<std::vec::Vec<std::string::String>> {
                    let input = match input.ids {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

