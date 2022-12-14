// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_jobs_by_pipeline_output_next_page_token(input: &crate::output::ListJobsByPipelineOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_page_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_jobs_by_status_output_next_page_token(input: &crate::output::ListJobsByStatusOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_page_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_pipelines_output_next_page_token(input: &crate::output::ListPipelinesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_page_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_presets_output_next_page_token(input: &crate::output::ListPresetsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_page_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_jobs_by_pipeline_output_jobs(input: crate::output::ListJobsByPipelineOutput) -> std::option::Option<std::vec::Vec<crate::model::Job>> {
                    let input = match input.jobs {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_jobs_by_status_output_jobs(input: crate::output::ListJobsByStatusOutput) -> std::option::Option<std::vec::Vec<crate::model::Job>> {
                    let input = match input.jobs {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_pipelines_output_pipelines(input: crate::output::ListPipelinesOutput) -> std::option::Option<std::vec::Vec<crate::model::Pipeline>> {
                    let input = match input.pipelines {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_presets_output_presets(input: crate::output::ListPresetsOutput) -> std::option::Option<std::vec::Vec<crate::model::Preset>> {
                    let input = match input.presets {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

