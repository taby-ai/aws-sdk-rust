// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_get_app_monitor_data_output_next_token(input: &crate::output::GetAppMonitorDataOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_app_monitors_output_next_token(input: &crate::output::ListAppMonitorsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_get_app_monitor_data_output_events(input: crate::output::GetAppMonitorDataOutput) -> std::option::Option<std::vec::Vec<std::string::String>> {
                    let input = match input.events {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_app_monitors_output_app_monitor_summaries(input: crate::output::ListAppMonitorsOutput) -> std::option::Option<std::vec::Vec<crate::model::AppMonitorSummary>> {
                    let input = match input.app_monitor_summaries {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

