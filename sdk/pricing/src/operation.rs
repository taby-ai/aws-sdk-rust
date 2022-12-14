// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DescribeServices`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_services`](crate::client::Client::describe_services).
            ///
            /// See [`crate::client::fluent_builders::DescribeServices`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct DescribeServices {
    _private: ()
}
impl DescribeServices {
    /// Creates a new builder-style object to manufacture [`DescribeServicesInput`](crate::input::DescribeServicesInput).
    pub fn builder() -> crate::input::describe_services_input::Builder {
        crate::input::describe_services_input::Builder::default()
    }
    /// Creates a new `DescribeServices` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeServices {
                type Output = std::result::Result<crate::output::DescribeServicesOutput, crate::error::DescribeServicesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_services_error(response)
                     } else {
                        crate::operation_deser::parse_describe_services_response(response)
                     }
                }
            }

/// Operation shape for `GetAttributeValues`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_attribute_values`](crate::client::Client::get_attribute_values).
            ///
            /// See [`crate::client::fluent_builders::GetAttributeValues`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct GetAttributeValues {
    _private: ()
}
impl GetAttributeValues {
    /// Creates a new builder-style object to manufacture [`GetAttributeValuesInput`](crate::input::GetAttributeValuesInput).
    pub fn builder() -> crate::input::get_attribute_values_input::Builder {
        crate::input::get_attribute_values_input::Builder::default()
    }
    /// Creates a new `GetAttributeValues` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAttributeValues {
                type Output = std::result::Result<crate::output::GetAttributeValuesOutput, crate::error::GetAttributeValuesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_attribute_values_error(response)
                     } else {
                        crate::operation_deser::parse_get_attribute_values_response(response)
                     }
                }
            }

/// Operation shape for `GetProducts`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_products`](crate::client::Client::get_products).
            ///
            /// See [`crate::client::fluent_builders::GetProducts`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct GetProducts {
    _private: ()
}
impl GetProducts {
    /// Creates a new builder-style object to manufacture [`GetProductsInput`](crate::input::GetProductsInput).
    pub fn builder() -> crate::input::get_products_input::Builder {
        crate::input::get_products_input::Builder::default()
    }
    /// Creates a new `GetProducts` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetProducts {
                type Output = std::result::Result<crate::output::GetProductsOutput, crate::error::GetProductsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_products_error(response)
                     } else {
                        crate::operation_deser::parse_get_products_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;

