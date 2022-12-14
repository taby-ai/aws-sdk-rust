// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_rule_groups_namespaces_output_next_token(input: &crate::output::ListRuleGroupsNamespacesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_workspaces_output_next_token(input: &crate::output::ListWorkspacesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_rule_groups_namespaces_output_rule_groups_namespaces(input: crate::output::ListRuleGroupsNamespacesOutput) -> std::option::Option<std::vec::Vec<crate::model::RuleGroupsNamespaceSummary>> {
                    let input = match input.rule_groups_namespaces {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_workspaces_output_workspaces(input: crate::output::ListWorkspacesOutput) -> std::option::Option<std::vec::Vec<crate::model::WorkspaceSummary>> {
                    let input = match input.workspaces {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

