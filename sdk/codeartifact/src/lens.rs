// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_domains_output_next_token(input: &crate::output::ListDomainsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_packages_output_next_token(input: &crate::output::ListPackagesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_package_version_assets_output_next_token(input: &crate::output::ListPackageVersionAssetsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_package_versions_output_next_token(input: &crate::output::ListPackageVersionsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_repositories_output_next_token(input: &crate::output::ListRepositoriesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_repositories_in_domain_output_next_token(input: &crate::output::ListRepositoriesInDomainOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_domains_output_domains(input: crate::output::ListDomainsOutput) -> std::option::Option<std::vec::Vec<crate::model::DomainSummary>> {
                    let input = match input.domains {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_packages_output_packages(input: crate::output::ListPackagesOutput) -> std::option::Option<std::vec::Vec<crate::model::PackageSummary>> {
                    let input = match input.packages {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_package_version_assets_output_assets(input: crate::output::ListPackageVersionAssetsOutput) -> std::option::Option<std::vec::Vec<crate::model::AssetSummary>> {
                    let input = match input.assets {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_package_versions_output_versions(input: crate::output::ListPackageVersionsOutput) -> std::option::Option<std::vec::Vec<crate::model::PackageVersionSummary>> {
                    let input = match input.versions {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_repositories_output_repositories(input: crate::output::ListRepositoriesOutput) -> std::option::Option<std::vec::Vec<crate::model::RepositorySummary>> {
                    let input = match input.repositories {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_repositories_in_domain_output_repositories(input: crate::output::ListRepositoriesInDomainOutput) -> std::option::Option<std::vec::Vec<crate::model::RepositorySummary>> {
                    let input = match input.repositories {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

