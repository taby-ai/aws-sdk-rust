// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_assets_output_next_token(input: &crate::output::ListAssetsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_packaging_configurations_output_next_token(input: &crate::output::ListPackagingConfigurationsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_packaging_groups_output_next_token(input: &crate::output::ListPackagingGroupsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_assets_output_assets(input: crate::output::ListAssetsOutput) -> std::option::Option<std::vec::Vec<crate::model::AssetShallow>> {
                    let input = match input.assets {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_packaging_configurations_output_packaging_configurations(input: crate::output::ListPackagingConfigurationsOutput) -> std::option::Option<std::vec::Vec<crate::model::PackagingConfiguration>> {
                    let input = match input.packaging_configurations {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_packaging_groups_output_packaging_groups(input: crate::output::ListPackagingGroupsOutput) -> std::option::Option<std::vec::Vec<crate::model::PackagingGroup>> {
                    let input = match input.packaging_groups {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

