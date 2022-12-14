// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_crls_output_next_token(input: &crate::output::ListCrlsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_profiles_output_next_token(input: &crate::output::ListProfilesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_subjects_output_next_token(input: &crate::output::ListSubjectsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_trust_anchors_output_next_token(input: &crate::output::ListTrustAnchorsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_crls_output_crls(input: crate::output::ListCrlsOutput) -> std::option::Option<std::vec::Vec<crate::model::CrlDetail>> {
                    let input = match input.crls {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_profiles_output_profiles(input: crate::output::ListProfilesOutput) -> std::option::Option<std::vec::Vec<crate::model::ProfileDetail>> {
                    let input = match input.profiles {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_subjects_output_subjects(input: crate::output::ListSubjectsOutput) -> std::option::Option<std::vec::Vec<crate::model::SubjectSummary>> {
                    let input = match input.subjects {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_trust_anchors_output_trust_anchors(input: crate::output::ListTrustAnchorsOutput) -> std::option::Option<std::vec::Vec<crate::model::TrustAnchorDetail>> {
                    let input = match input.trust_anchors {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

