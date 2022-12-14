// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct StopProductSubscriptionOutput  {
    /// <p>Metadata that describes the start product subscription operation.</p>
    #[doc(hidden)]pub product_user_summary: std::option::Option<crate::model::ProductUserSummary>,
}
impl StopProductSubscriptionOutput {
    /// <p>Metadata that describes the start product subscription operation.</p>
    pub fn product_user_summary(&self) -> std::option::Option<& crate::model::ProductUserSummary> {
        self.product_user_summary.as_ref()
    }
}
impl  std::fmt::Debug for StopProductSubscriptionOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StopProductSubscriptionOutput");
        formatter.field("product_user_summary", &self.product_user_summary);
        formatter.finish()
    }
}
/// See [`StopProductSubscriptionOutput`](crate::output::StopProductSubscriptionOutput).
pub mod stop_product_subscription_output {
    
    /// A builder for [`StopProductSubscriptionOutput`](crate::output::StopProductSubscriptionOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) product_user_summary: std::option::Option<crate::model::ProductUserSummary>,
    }
    impl Builder {
        /// <p>Metadata that describes the start product subscription operation.</p>
        pub fn product_user_summary(mut self, input: crate::model::ProductUserSummary) -> Self {
            self.product_user_summary = Some(input);
            self
        }
        /// <p>Metadata that describes the start product subscription operation.</p>
        pub fn set_product_user_summary(mut self, input: std::option::Option<crate::model::ProductUserSummary>) -> Self {
            self.product_user_summary = input; self
        }
        /// Consumes the builder and constructs a [`StopProductSubscriptionOutput`](crate::output::StopProductSubscriptionOutput).
        pub fn build(self) -> crate::output::StopProductSubscriptionOutput {
            crate::output::StopProductSubscriptionOutput {
                product_user_summary: self.product_user_summary
                ,
            }
        }
    }
    
    
}
impl StopProductSubscriptionOutput {
    /// Creates a new builder-style object to manufacture [`StopProductSubscriptionOutput`](crate::output::StopProductSubscriptionOutput).
    pub fn builder() -> crate::output::stop_product_subscription_output::Builder {
        crate::output::stop_product_subscription_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct StartProductSubscriptionOutput  {
    /// <p>Metadata that describes the start product subscription operation.</p>
    #[doc(hidden)]pub product_user_summary: std::option::Option<crate::model::ProductUserSummary>,
}
impl StartProductSubscriptionOutput {
    /// <p>Metadata that describes the start product subscription operation.</p>
    pub fn product_user_summary(&self) -> std::option::Option<& crate::model::ProductUserSummary> {
        self.product_user_summary.as_ref()
    }
}
impl  std::fmt::Debug for StartProductSubscriptionOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("StartProductSubscriptionOutput");
        formatter.field("product_user_summary", &self.product_user_summary);
        formatter.finish()
    }
}
/// See [`StartProductSubscriptionOutput`](crate::output::StartProductSubscriptionOutput).
pub mod start_product_subscription_output {
    
    /// A builder for [`StartProductSubscriptionOutput`](crate::output::StartProductSubscriptionOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) product_user_summary: std::option::Option<crate::model::ProductUserSummary>,
    }
    impl Builder {
        /// <p>Metadata that describes the start product subscription operation.</p>
        pub fn product_user_summary(mut self, input: crate::model::ProductUserSummary) -> Self {
            self.product_user_summary = Some(input);
            self
        }
        /// <p>Metadata that describes the start product subscription operation.</p>
        pub fn set_product_user_summary(mut self, input: std::option::Option<crate::model::ProductUserSummary>) -> Self {
            self.product_user_summary = input; self
        }
        /// Consumes the builder and constructs a [`StartProductSubscriptionOutput`](crate::output::StartProductSubscriptionOutput).
        pub fn build(self) -> crate::output::StartProductSubscriptionOutput {
            crate::output::StartProductSubscriptionOutput {
                product_user_summary: self.product_user_summary
                ,
            }
        }
    }
    
    
}
impl StartProductSubscriptionOutput {
    /// Creates a new builder-style object to manufacture [`StartProductSubscriptionOutput`](crate::output::StartProductSubscriptionOutput).
    pub fn builder() -> crate::output::start_product_subscription_output::Builder {
        crate::output::start_product_subscription_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct RegisterIdentityProviderOutput  {
    /// <p>Metadata that describes the results of an identity provider operation.</p>
    #[doc(hidden)]pub identity_provider_summary: std::option::Option<crate::model::IdentityProviderSummary>,
}
impl RegisterIdentityProviderOutput {
    /// <p>Metadata that describes the results of an identity provider operation.</p>
    pub fn identity_provider_summary(&self) -> std::option::Option<& crate::model::IdentityProviderSummary> {
        self.identity_provider_summary.as_ref()
    }
}
impl  std::fmt::Debug for RegisterIdentityProviderOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RegisterIdentityProviderOutput");
        formatter.field("identity_provider_summary", &self.identity_provider_summary);
        formatter.finish()
    }
}
/// See [`RegisterIdentityProviderOutput`](crate::output::RegisterIdentityProviderOutput).
pub mod register_identity_provider_output {
    
    /// A builder for [`RegisterIdentityProviderOutput`](crate::output::RegisterIdentityProviderOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) identity_provider_summary: std::option::Option<crate::model::IdentityProviderSummary>,
    }
    impl Builder {
        /// <p>Metadata that describes the results of an identity provider operation.</p>
        pub fn identity_provider_summary(mut self, input: crate::model::IdentityProviderSummary) -> Self {
            self.identity_provider_summary = Some(input);
            self
        }
        /// <p>Metadata that describes the results of an identity provider operation.</p>
        pub fn set_identity_provider_summary(mut self, input: std::option::Option<crate::model::IdentityProviderSummary>) -> Self {
            self.identity_provider_summary = input; self
        }
        /// Consumes the builder and constructs a [`RegisterIdentityProviderOutput`](crate::output::RegisterIdentityProviderOutput).
        pub fn build(self) -> crate::output::RegisterIdentityProviderOutput {
            crate::output::RegisterIdentityProviderOutput {
                identity_provider_summary: self.identity_provider_summary
                ,
            }
        }
    }
    
    
}
impl RegisterIdentityProviderOutput {
    /// Creates a new builder-style object to manufacture [`RegisterIdentityProviderOutput`](crate::output::RegisterIdentityProviderOutput).
    pub fn builder() -> crate::output::register_identity_provider_output::Builder {
        crate::output::register_identity_provider_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct ListUserAssociationsOutput  {
    /// <p>Metadata that describes the list user association operation.</p>
    #[doc(hidden)]pub instance_user_summaries: std::option::Option<std::vec::Vec<crate::model::InstanceUserSummary>>,
    /// <p>Token for the next set of results.</p>
    #[doc(hidden)]pub next_token: std::option::Option<std::string::String>,
}
impl ListUserAssociationsOutput {
    /// <p>Metadata that describes the list user association operation.</p>
    pub fn instance_user_summaries(&self) -> std::option::Option<& [crate::model::InstanceUserSummary]> {
        self.instance_user_summaries.as_deref()
    }
    /// <p>Token for the next set of results.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
impl  std::fmt::Debug for ListUserAssociationsOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListUserAssociationsOutput");
        formatter.field("instance_user_summaries", &self.instance_user_summaries);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListUserAssociationsOutput`](crate::output::ListUserAssociationsOutput).
pub mod list_user_associations_output {
    
    /// A builder for [`ListUserAssociationsOutput`](crate::output::ListUserAssociationsOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) instance_user_summaries: std::option::Option<std::vec::Vec<crate::model::InstanceUserSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `instance_user_summaries`.
        ///
        /// To override the contents of this collection use [`set_instance_user_summaries`](Self::set_instance_user_summaries).
        ///
        /// <p>Metadata that describes the list user association operation.</p>
        pub fn instance_user_summaries(mut self, input: crate::model::InstanceUserSummary) -> Self {
            let mut v = self.instance_user_summaries.unwrap_or_default();
                            v.push(input);
                            self.instance_user_summaries = Some(v);
                            self
        }
        /// <p>Metadata that describes the list user association operation.</p>
        pub fn set_instance_user_summaries(mut self, input: std::option::Option<std::vec::Vec<crate::model::InstanceUserSummary>>) -> Self {
            self.instance_user_summaries = input; self
        }
        /// <p>Token for the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>Token for the next set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Consumes the builder and constructs a [`ListUserAssociationsOutput`](crate::output::ListUserAssociationsOutput).
        pub fn build(self) -> crate::output::ListUserAssociationsOutput {
            crate::output::ListUserAssociationsOutput {
                instance_user_summaries: self.instance_user_summaries
                ,
                next_token: self.next_token
                ,
            }
        }
    }
    
    
}
impl ListUserAssociationsOutput {
    /// Creates a new builder-style object to manufacture [`ListUserAssociationsOutput`](crate::output::ListUserAssociationsOutput).
    pub fn builder() -> crate::output::list_user_associations_output::Builder {
        crate::output::list_user_associations_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct ListProductSubscriptionsOutput  {
    /// <p>Metadata that describes the list product subscriptions operation.</p>
    #[doc(hidden)]pub product_user_summaries: std::option::Option<std::vec::Vec<crate::model::ProductUserSummary>>,
    /// <p>Token for the next set of results.</p>
    #[doc(hidden)]pub next_token: std::option::Option<std::string::String>,
}
impl ListProductSubscriptionsOutput {
    /// <p>Metadata that describes the list product subscriptions operation.</p>
    pub fn product_user_summaries(&self) -> std::option::Option<& [crate::model::ProductUserSummary]> {
        self.product_user_summaries.as_deref()
    }
    /// <p>Token for the next set of results.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
impl  std::fmt::Debug for ListProductSubscriptionsOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListProductSubscriptionsOutput");
        formatter.field("product_user_summaries", &self.product_user_summaries);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListProductSubscriptionsOutput`](crate::output::ListProductSubscriptionsOutput).
pub mod list_product_subscriptions_output {
    
    /// A builder for [`ListProductSubscriptionsOutput`](crate::output::ListProductSubscriptionsOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) product_user_summaries: std::option::Option<std::vec::Vec<crate::model::ProductUserSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `product_user_summaries`.
        ///
        /// To override the contents of this collection use [`set_product_user_summaries`](Self::set_product_user_summaries).
        ///
        /// <p>Metadata that describes the list product subscriptions operation.</p>
        pub fn product_user_summaries(mut self, input: crate::model::ProductUserSummary) -> Self {
            let mut v = self.product_user_summaries.unwrap_or_default();
                            v.push(input);
                            self.product_user_summaries = Some(v);
                            self
        }
        /// <p>Metadata that describes the list product subscriptions operation.</p>
        pub fn set_product_user_summaries(mut self, input: std::option::Option<std::vec::Vec<crate::model::ProductUserSummary>>) -> Self {
            self.product_user_summaries = input; self
        }
        /// <p>Token for the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>Token for the next set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Consumes the builder and constructs a [`ListProductSubscriptionsOutput`](crate::output::ListProductSubscriptionsOutput).
        pub fn build(self) -> crate::output::ListProductSubscriptionsOutput {
            crate::output::ListProductSubscriptionsOutput {
                product_user_summaries: self.product_user_summaries
                ,
                next_token: self.next_token
                ,
            }
        }
    }
    
    
}
impl ListProductSubscriptionsOutput {
    /// Creates a new builder-style object to manufacture [`ListProductSubscriptionsOutput`](crate::output::ListProductSubscriptionsOutput).
    pub fn builder() -> crate::output::list_product_subscriptions_output::Builder {
        crate::output::list_product_subscriptions_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct ListInstancesOutput  {
    /// <p>Metadata that describes the list instances operation.</p>
    #[doc(hidden)]pub instance_summaries: std::option::Option<std::vec::Vec<crate::model::InstanceSummary>>,
    /// <p>Token for the next set of results.</p>
    #[doc(hidden)]pub next_token: std::option::Option<std::string::String>,
}
impl ListInstancesOutput {
    /// <p>Metadata that describes the list instances operation.</p>
    pub fn instance_summaries(&self) -> std::option::Option<& [crate::model::InstanceSummary]> {
        self.instance_summaries.as_deref()
    }
    /// <p>Token for the next set of results.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
impl  std::fmt::Debug for ListInstancesOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListInstancesOutput");
        formatter.field("instance_summaries", &self.instance_summaries);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListInstancesOutput`](crate::output::ListInstancesOutput).
pub mod list_instances_output {
    
    /// A builder for [`ListInstancesOutput`](crate::output::ListInstancesOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) instance_summaries: std::option::Option<std::vec::Vec<crate::model::InstanceSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `instance_summaries`.
        ///
        /// To override the contents of this collection use [`set_instance_summaries`](Self::set_instance_summaries).
        ///
        /// <p>Metadata that describes the list instances operation.</p>
        pub fn instance_summaries(mut self, input: crate::model::InstanceSummary) -> Self {
            let mut v = self.instance_summaries.unwrap_or_default();
                            v.push(input);
                            self.instance_summaries = Some(v);
                            self
        }
        /// <p>Metadata that describes the list instances operation.</p>
        pub fn set_instance_summaries(mut self, input: std::option::Option<std::vec::Vec<crate::model::InstanceSummary>>) -> Self {
            self.instance_summaries = input; self
        }
        /// <p>Token for the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>Token for the next set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Consumes the builder and constructs a [`ListInstancesOutput`](crate::output::ListInstancesOutput).
        pub fn build(self) -> crate::output::ListInstancesOutput {
            crate::output::ListInstancesOutput {
                instance_summaries: self.instance_summaries
                ,
                next_token: self.next_token
                ,
            }
        }
    }
    
    
}
impl ListInstancesOutput {
    /// Creates a new builder-style object to manufacture [`ListInstancesOutput`](crate::output::ListInstancesOutput).
    pub fn builder() -> crate::output::list_instances_output::Builder {
        crate::output::list_instances_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct ListIdentityProvidersOutput  {
    /// <p>Metadata that describes the list identity providers operation.</p>
    #[doc(hidden)]pub identity_provider_summaries: std::option::Option<std::vec::Vec<crate::model::IdentityProviderSummary>>,
    /// <p>Token for the next set of results.</p>
    #[doc(hidden)]pub next_token: std::option::Option<std::string::String>,
}
impl ListIdentityProvidersOutput {
    /// <p>Metadata that describes the list identity providers operation.</p>
    pub fn identity_provider_summaries(&self) -> std::option::Option<& [crate::model::IdentityProviderSummary]> {
        self.identity_provider_summaries.as_deref()
    }
    /// <p>Token for the next set of results.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
impl  std::fmt::Debug for ListIdentityProvidersOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListIdentityProvidersOutput");
        formatter.field("identity_provider_summaries", &self.identity_provider_summaries);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListIdentityProvidersOutput`](crate::output::ListIdentityProvidersOutput).
pub mod list_identity_providers_output {
    
    /// A builder for [`ListIdentityProvidersOutput`](crate::output::ListIdentityProvidersOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) identity_provider_summaries: std::option::Option<std::vec::Vec<crate::model::IdentityProviderSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `identity_provider_summaries`.
        ///
        /// To override the contents of this collection use [`set_identity_provider_summaries`](Self::set_identity_provider_summaries).
        ///
        /// <p>Metadata that describes the list identity providers operation.</p>
        pub fn identity_provider_summaries(mut self, input: crate::model::IdentityProviderSummary) -> Self {
            let mut v = self.identity_provider_summaries.unwrap_or_default();
                            v.push(input);
                            self.identity_provider_summaries = Some(v);
                            self
        }
        /// <p>Metadata that describes the list identity providers operation.</p>
        pub fn set_identity_provider_summaries(mut self, input: std::option::Option<std::vec::Vec<crate::model::IdentityProviderSummary>>) -> Self {
            self.identity_provider_summaries = input; self
        }
        /// <p>Token for the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>Token for the next set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Consumes the builder and constructs a [`ListIdentityProvidersOutput`](crate::output::ListIdentityProvidersOutput).
        pub fn build(self) -> crate::output::ListIdentityProvidersOutput {
            crate::output::ListIdentityProvidersOutput {
                identity_provider_summaries: self.identity_provider_summaries
                ,
                next_token: self.next_token
                ,
            }
        }
    }
    
    
}
impl ListIdentityProvidersOutput {
    /// Creates a new builder-style object to manufacture [`ListIdentityProvidersOutput`](crate::output::ListIdentityProvidersOutput).
    pub fn builder() -> crate::output::list_identity_providers_output::Builder {
        crate::output::list_identity_providers_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct DisassociateUserOutput  {
    /// <p>Metadata that describes the associate user operation.</p>
    #[doc(hidden)]pub instance_user_summary: std::option::Option<crate::model::InstanceUserSummary>,
}
impl DisassociateUserOutput {
    /// <p>Metadata that describes the associate user operation.</p>
    pub fn instance_user_summary(&self) -> std::option::Option<& crate::model::InstanceUserSummary> {
        self.instance_user_summary.as_ref()
    }
}
impl  std::fmt::Debug for DisassociateUserOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DisassociateUserOutput");
        formatter.field("instance_user_summary", &self.instance_user_summary);
        formatter.finish()
    }
}
/// See [`DisassociateUserOutput`](crate::output::DisassociateUserOutput).
pub mod disassociate_user_output {
    
    /// A builder for [`DisassociateUserOutput`](crate::output::DisassociateUserOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) instance_user_summary: std::option::Option<crate::model::InstanceUserSummary>,
    }
    impl Builder {
        /// <p>Metadata that describes the associate user operation.</p>
        pub fn instance_user_summary(mut self, input: crate::model::InstanceUserSummary) -> Self {
            self.instance_user_summary = Some(input);
            self
        }
        /// <p>Metadata that describes the associate user operation.</p>
        pub fn set_instance_user_summary(mut self, input: std::option::Option<crate::model::InstanceUserSummary>) -> Self {
            self.instance_user_summary = input; self
        }
        /// Consumes the builder and constructs a [`DisassociateUserOutput`](crate::output::DisassociateUserOutput).
        pub fn build(self) -> crate::output::DisassociateUserOutput {
            crate::output::DisassociateUserOutput {
                instance_user_summary: self.instance_user_summary
                ,
            }
        }
    }
    
    
}
impl DisassociateUserOutput {
    /// Creates a new builder-style object to manufacture [`DisassociateUserOutput`](crate::output::DisassociateUserOutput).
    pub fn builder() -> crate::output::disassociate_user_output::Builder {
        crate::output::disassociate_user_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct DeregisterIdentityProviderOutput  {
    /// <p>Metadata that describes the results of an identity provider operation.</p>
    #[doc(hidden)]pub identity_provider_summary: std::option::Option<crate::model::IdentityProviderSummary>,
}
impl DeregisterIdentityProviderOutput {
    /// <p>Metadata that describes the results of an identity provider operation.</p>
    pub fn identity_provider_summary(&self) -> std::option::Option<& crate::model::IdentityProviderSummary> {
        self.identity_provider_summary.as_ref()
    }
}
impl  std::fmt::Debug for DeregisterIdentityProviderOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeregisterIdentityProviderOutput");
        formatter.field("identity_provider_summary", &self.identity_provider_summary);
        formatter.finish()
    }
}
/// See [`DeregisterIdentityProviderOutput`](crate::output::DeregisterIdentityProviderOutput).
pub mod deregister_identity_provider_output {
    
    /// A builder for [`DeregisterIdentityProviderOutput`](crate::output::DeregisterIdentityProviderOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) identity_provider_summary: std::option::Option<crate::model::IdentityProviderSummary>,
    }
    impl Builder {
        /// <p>Metadata that describes the results of an identity provider operation.</p>
        pub fn identity_provider_summary(mut self, input: crate::model::IdentityProviderSummary) -> Self {
            self.identity_provider_summary = Some(input);
            self
        }
        /// <p>Metadata that describes the results of an identity provider operation.</p>
        pub fn set_identity_provider_summary(mut self, input: std::option::Option<crate::model::IdentityProviderSummary>) -> Self {
            self.identity_provider_summary = input; self
        }
        /// Consumes the builder and constructs a [`DeregisterIdentityProviderOutput`](crate::output::DeregisterIdentityProviderOutput).
        pub fn build(self) -> crate::output::DeregisterIdentityProviderOutput {
            crate::output::DeregisterIdentityProviderOutput {
                identity_provider_summary: self.identity_provider_summary
                ,
            }
        }
    }
    
    
}
impl DeregisterIdentityProviderOutput {
    /// Creates a new builder-style object to manufacture [`DeregisterIdentityProviderOutput`](crate::output::DeregisterIdentityProviderOutput).
    pub fn builder() -> crate::output::deregister_identity_provider_output::Builder {
        crate::output::deregister_identity_provider_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct AssociateUserOutput  {
    /// <p>Metadata that describes the associate user operation.</p>
    #[doc(hidden)]pub instance_user_summary: std::option::Option<crate::model::InstanceUserSummary>,
}
impl AssociateUserOutput {
    /// <p>Metadata that describes the associate user operation.</p>
    pub fn instance_user_summary(&self) -> std::option::Option<& crate::model::InstanceUserSummary> {
        self.instance_user_summary.as_ref()
    }
}
impl  std::fmt::Debug for AssociateUserOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AssociateUserOutput");
        formatter.field("instance_user_summary", &self.instance_user_summary);
        formatter.finish()
    }
}
/// See [`AssociateUserOutput`](crate::output::AssociateUserOutput).
pub mod associate_user_output {
    
    /// A builder for [`AssociateUserOutput`](crate::output::AssociateUserOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) instance_user_summary: std::option::Option<crate::model::InstanceUserSummary>,
    }
    impl Builder {
        /// <p>Metadata that describes the associate user operation.</p>
        pub fn instance_user_summary(mut self, input: crate::model::InstanceUserSummary) -> Self {
            self.instance_user_summary = Some(input);
            self
        }
        /// <p>Metadata that describes the associate user operation.</p>
        pub fn set_instance_user_summary(mut self, input: std::option::Option<crate::model::InstanceUserSummary>) -> Self {
            self.instance_user_summary = input; self
        }
        /// Consumes the builder and constructs a [`AssociateUserOutput`](crate::output::AssociateUserOutput).
        pub fn build(self) -> crate::output::AssociateUserOutput {
            crate::output::AssociateUserOutput {
                instance_user_summary: self.instance_user_summary
                ,
            }
        }
    }
    
    
}
impl AssociateUserOutput {
    /// Creates a new builder-style object to manufacture [`AssociateUserOutput`](crate::output::AssociateUserOutput).
    pub fn builder() -> crate::output::associate_user_output::Builder {
        crate::output::associate_user_output::Builder::default()
    }
}

