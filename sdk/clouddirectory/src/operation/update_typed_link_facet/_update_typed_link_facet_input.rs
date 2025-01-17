// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateTypedLinkFacetInput {
    /// <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <code>arns</code>.</p>
    #[doc(hidden)]
    pub schema_arn: std::option::Option<std::string::String>,
    /// <p>The unique name of the typed link facet.</p>
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// <p>Attributes update structure.</p>
    #[doc(hidden)]
    pub attribute_updates:
        std::option::Option<std::vec::Vec<crate::types::TypedLinkFacetAttributeUpdate>>,
    /// <p>The order of identity attributes for the facet, from most significant to least significant. The ability to filter typed links considers the order that the attributes are defined on the typed link facet. When providing ranges to a typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range. Filters are interpreted in the order of the attributes on the typed link facet, not the order in which they are supplied to any API calls. For more information about identity attributes, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    #[doc(hidden)]
    pub identity_attribute_order: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl UpdateTypedLinkFacetInput {
    /// <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <code>arns</code>.</p>
    pub fn schema_arn(&self) -> std::option::Option<&str> {
        self.schema_arn.as_deref()
    }
    /// <p>The unique name of the typed link facet.</p>
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Attributes update structure.</p>
    pub fn attribute_updates(
        &self,
    ) -> std::option::Option<&[crate::types::TypedLinkFacetAttributeUpdate]> {
        self.attribute_updates.as_deref()
    }
    /// <p>The order of identity attributes for the facet, from most significant to least significant. The ability to filter typed links considers the order that the attributes are defined on the typed link facet. When providing ranges to a typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range. Filters are interpreted in the order of the attributes on the typed link facet, not the order in which they are supplied to any API calls. For more information about identity attributes, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    pub fn identity_attribute_order(&self) -> std::option::Option<&[std::string::String]> {
        self.identity_attribute_order.as_deref()
    }
}
impl UpdateTypedLinkFacetInput {
    /// Creates a new builder-style object to manufacture [`UpdateTypedLinkFacetInput`](crate::operation::update_typed_link_facet::UpdateTypedLinkFacetInput).
    pub fn builder(
    ) -> crate::operation::update_typed_link_facet::builders::UpdateTypedLinkFacetInputBuilder {
        crate::operation::update_typed_link_facet::builders::UpdateTypedLinkFacetInputBuilder::default()
    }
}

/// A builder for [`UpdateTypedLinkFacetInput`](crate::operation::update_typed_link_facet::UpdateTypedLinkFacetInput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct UpdateTypedLinkFacetInputBuilder {
    pub(crate) schema_arn: std::option::Option<std::string::String>,
    pub(crate) name: std::option::Option<std::string::String>,
    pub(crate) attribute_updates:
        std::option::Option<std::vec::Vec<crate::types::TypedLinkFacetAttributeUpdate>>,
    pub(crate) identity_attribute_order: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl UpdateTypedLinkFacetInputBuilder {
    /// <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <code>arns</code>.</p>
    pub fn schema_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.schema_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that is associated with the schema. For more information, see <code>arns</code>.</p>
    pub fn set_schema_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.schema_arn = input;
        self
    }
    /// <p>The unique name of the typed link facet.</p>
    pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
        self.name = Some(input.into());
        self
    }
    /// <p>The unique name of the typed link facet.</p>
    pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Appends an item to `attribute_updates`.
    ///
    /// To override the contents of this collection use [`set_attribute_updates`](Self::set_attribute_updates).
    ///
    /// <p>Attributes update structure.</p>
    pub fn attribute_updates(mut self, input: crate::types::TypedLinkFacetAttributeUpdate) -> Self {
        let mut v = self.attribute_updates.unwrap_or_default();
        v.push(input);
        self.attribute_updates = Some(v);
        self
    }
    /// <p>Attributes update structure.</p>
    pub fn set_attribute_updates(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TypedLinkFacetAttributeUpdate>>,
    ) -> Self {
        self.attribute_updates = input;
        self
    }
    /// Appends an item to `identity_attribute_order`.
    ///
    /// To override the contents of this collection use [`set_identity_attribute_order`](Self::set_identity_attribute_order).
    ///
    /// <p>The order of identity attributes for the facet, from most significant to least significant. The ability to filter typed links considers the order that the attributes are defined on the typed link facet. When providing ranges to a typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range. Filters are interpreted in the order of the attributes on the typed link facet, not the order in which they are supplied to any API calls. For more information about identity attributes, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    pub fn identity_attribute_order(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.identity_attribute_order.unwrap_or_default();
        v.push(input.into());
        self.identity_attribute_order = Some(v);
        self
    }
    /// <p>The order of identity attributes for the facet, from most significant to least significant. The ability to filter typed links considers the order that the attributes are defined on the typed link facet. When providing ranges to a typed link selection, any inexact ranges must be specified at the end. Any attributes that do not have a range specified are presumed to match the entire range. Filters are interpreted in the order of the attributes on the typed link facet, not the order in which they are supplied to any API calls. For more information about identity attributes, see <a href="https://docs.aws.amazon.com/clouddirectory/latest/developerguide/directory_objects_links.html#directory_objects_links_typedlink">Typed Links</a>.</p>
    pub fn set_identity_attribute_order(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.identity_attribute_order = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateTypedLinkFacetInput`](crate::operation::update_typed_link_facet::UpdateTypedLinkFacetInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::update_typed_link_facet::UpdateTypedLinkFacetInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::update_typed_link_facet::UpdateTypedLinkFacetInput {
                schema_arn: self.schema_arn,
                name: self.name,
                attribute_updates: self.attribute_updates,
                identity_attribute_order: self.identity_attribute_order,
            },
        )
    }
}
