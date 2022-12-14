// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_payload_create_project_input(payload: std::option::Option<aws_smithy_types::Blob>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::BuildError> {
    let payload = match payload {
                                Some(t) => t,
                                None => return Ok(
        Vec::new()
    )};
    Ok(
        payload.into_inner()
    )
}

pub fn serialize_payload_update_project_input(payload: std::option::Option<aws_smithy_types::Blob>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::BuildError> {
    let payload = match payload {
                                Some(t) => t,
                                None => return Ok(
        Vec::new()
    )};
    Ok(
        payload.into_inner()
    )
}

