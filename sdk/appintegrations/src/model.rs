// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The event integration.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct EventIntegration  {
    /// <p>The Amazon Resource Name (ARN) of the event integration.</p>
    #[doc(hidden)]pub event_integration_arn: std::option::Option<std::string::String>,
    /// <p>The name of the event integration.</p>
    #[doc(hidden)]pub name: std::option::Option<std::string::String>,
    /// <p>The event integration description.</p>
    #[doc(hidden)]pub description: std::option::Option<std::string::String>,
    /// <p>The event integration filter.</p>
    #[doc(hidden)]pub event_filter: std::option::Option<crate::model::EventFilter>,
    /// <p>The Amazon EventBridge bus for the event integration.</p>
    #[doc(hidden)]pub event_bridge_bus: std::option::Option<std::string::String>,
    /// <p>The tags.</p>
    #[doc(hidden)]pub tags: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl EventIntegration {
    /// <p>The Amazon Resource Name (ARN) of the event integration.</p>
    pub fn event_integration_arn(&self) -> std::option::Option<& str> {
        self.event_integration_arn.as_deref()
    }
    /// <p>The name of the event integration.</p>
    pub fn name(&self) -> std::option::Option<& str> {
        self.name.as_deref()
    }
    /// <p>The event integration description.</p>
    pub fn description(&self) -> std::option::Option<& str> {
        self.description.as_deref()
    }
    /// <p>The event integration filter.</p>
    pub fn event_filter(&self) -> std::option::Option<& crate::model::EventFilter> {
        self.event_filter.as_ref()
    }
    /// <p>The Amazon EventBridge bus for the event integration.</p>
    pub fn event_bridge_bus(&self) -> std::option::Option<& str> {
        self.event_bridge_bus.as_deref()
    }
    /// <p>The tags.</p>
    pub fn tags(&self) -> std::option::Option<& std::collections::HashMap<std::string::String, std::string::String>> {
        self.tags.as_ref()
    }
}
impl  std::fmt::Debug for EventIntegration  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("EventIntegration");
        formatter.field("event_integration_arn", &self.event_integration_arn);
        formatter.field("name", &self.name);
        formatter.field("description", &self.description);
        formatter.field("event_filter", &self.event_filter);
        formatter.field("event_bridge_bus", &self.event_bridge_bus);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
/// See [`EventIntegration`](crate::model::EventIntegration).
pub mod event_integration {
    
    /// A builder for [`EventIntegration`](crate::model::EventIntegration).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) event_integration_arn: std::option::Option<std::string::String>,
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) description: std::option::Option<std::string::String>,
        pub(crate) event_filter: std::option::Option<crate::model::EventFilter>,
        pub(crate) event_bridge_bus: std::option::Option<std::string::String>,
        pub(crate) tags: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) of the event integration.</p>
        pub fn event_integration_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.event_integration_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the event integration.</p>
        pub fn set_event_integration_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.event_integration_arn = input; self
        }
        /// <p>The name of the event integration.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p>The name of the event integration.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input; self
        }
        /// <p>The event integration description.</p>
        pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
            self.description = Some(input.into());
            self
        }
        /// <p>The event integration description.</p>
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.description = input; self
        }
        /// <p>The event integration filter.</p>
        pub fn event_filter(mut self, input: crate::model::EventFilter) -> Self {
            self.event_filter = Some(input);
            self
        }
        /// <p>The event integration filter.</p>
        pub fn set_event_filter(mut self, input: std::option::Option<crate::model::EventFilter>) -> Self {
            self.event_filter = input; self
        }
        /// <p>The Amazon EventBridge bus for the event integration.</p>
        pub fn event_bridge_bus(mut self, input: impl Into<std::string::String>) -> Self {
            self.event_bridge_bus = Some(input.into());
            self
        }
        /// <p>The Amazon EventBridge bus for the event integration.</p>
        pub fn set_event_bridge_bus(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.event_bridge_bus = input; self
        }
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>The tags.</p>
        pub fn tags(mut self, k: impl Into<std::string::String>, v: impl Into<std::string::String>) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
                            hash_map.insert(k.into(), v.into());
                            self.tags = Some(hash_map);
                            self
        }
        /// <p>The tags.</p>
        pub fn set_tags(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>) -> Self {
            self.tags = input; self
        }
        /// Consumes the builder and constructs a [`EventIntegration`](crate::model::EventIntegration).
        pub fn build(self) -> crate::model::EventIntegration {
            crate::model::EventIntegration {
                event_integration_arn: self.event_integration_arn
                ,
                name: self.name
                ,
                description: self.description
                ,
                event_filter: self.event_filter
                ,
                event_bridge_bus: self.event_bridge_bus
                ,
                tags: self.tags
                ,
            }
        }
    }
    
    
}
impl EventIntegration {
    /// Creates a new builder-style object to manufacture [`EventIntegration`](crate::model::EventIntegration).
    pub fn builder() -> crate::model::event_integration::Builder {
        crate::model::event_integration::Builder::default()
    }
}

/// <p>The event filter.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct EventFilter  {
    /// <p>The source of the events.</p>
    #[doc(hidden)]pub source: std::option::Option<std::string::String>,
}
impl EventFilter {
    /// <p>The source of the events.</p>
    pub fn source(&self) -> std::option::Option<& str> {
        self.source.as_deref()
    }
}
impl  std::fmt::Debug for EventFilter  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("EventFilter");
        formatter.field("source", &self.source);
        formatter.finish()
    }
}
/// See [`EventFilter`](crate::model::EventFilter).
pub mod event_filter {
    
    /// A builder for [`EventFilter`](crate::model::EventFilter).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) source: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The source of the events.</p>
        pub fn source(mut self, input: impl Into<std::string::String>) -> Self {
            self.source = Some(input.into());
            self
        }
        /// <p>The source of the events.</p>
        pub fn set_source(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.source = input; self
        }
        /// Consumes the builder and constructs a [`EventFilter`](crate::model::EventFilter).
        pub fn build(self) -> crate::model::EventFilter {
            crate::model::EventFilter {
                source: self.source
                ,
            }
        }
    }
    
    
}
impl EventFilter {
    /// Creates a new builder-style object to manufacture [`EventFilter`](crate::model::EventFilter).
    pub fn builder() -> crate::model::event_filter::Builder {
        crate::model::event_filter::Builder::default()
    }
}

/// <p>The event integration association.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct EventIntegrationAssociation  {
    /// <p>The Amazon Resource Name (ARN) for the event integration association.</p>
    #[doc(hidden)]pub event_integration_association_arn: std::option::Option<std::string::String>,
    /// <p>The identifier for the event integration association.</p>
    #[doc(hidden)]pub event_integration_association_id: std::option::Option<std::string::String>,
    /// <p>The name of the event integration.</p>
    #[doc(hidden)]pub event_integration_name: std::option::Option<std::string::String>,
    /// <p>The identifier for the client that is associated with the event integration.</p>
    #[doc(hidden)]pub client_id: std::option::Option<std::string::String>,
    /// <p>The name of the EventBridge rule.</p>
    #[doc(hidden)]pub event_bridge_rule_name: std::option::Option<std::string::String>,
    /// <p>The metadata associated with the client.</p>
    #[doc(hidden)]pub client_association_metadata: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl EventIntegrationAssociation {
    /// <p>The Amazon Resource Name (ARN) for the event integration association.</p>
    pub fn event_integration_association_arn(&self) -> std::option::Option<& str> {
        self.event_integration_association_arn.as_deref()
    }
    /// <p>The identifier for the event integration association.</p>
    pub fn event_integration_association_id(&self) -> std::option::Option<& str> {
        self.event_integration_association_id.as_deref()
    }
    /// <p>The name of the event integration.</p>
    pub fn event_integration_name(&self) -> std::option::Option<& str> {
        self.event_integration_name.as_deref()
    }
    /// <p>The identifier for the client that is associated with the event integration.</p>
    pub fn client_id(&self) -> std::option::Option<& str> {
        self.client_id.as_deref()
    }
    /// <p>The name of the EventBridge rule.</p>
    pub fn event_bridge_rule_name(&self) -> std::option::Option<& str> {
        self.event_bridge_rule_name.as_deref()
    }
    /// <p>The metadata associated with the client.</p>
    pub fn client_association_metadata(&self) -> std::option::Option<& std::collections::HashMap<std::string::String, std::string::String>> {
        self.client_association_metadata.as_ref()
    }
}
impl  std::fmt::Debug for EventIntegrationAssociation  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("EventIntegrationAssociation");
        formatter.field("event_integration_association_arn", &self.event_integration_association_arn);
        formatter.field("event_integration_association_id", &self.event_integration_association_id);
        formatter.field("event_integration_name", &self.event_integration_name);
        formatter.field("client_id", &self.client_id);
        formatter.field("event_bridge_rule_name", &self.event_bridge_rule_name);
        formatter.field("client_association_metadata", &self.client_association_metadata);
        formatter.finish()
    }
}
/// See [`EventIntegrationAssociation`](crate::model::EventIntegrationAssociation).
pub mod event_integration_association {
    
    /// A builder for [`EventIntegrationAssociation`](crate::model::EventIntegrationAssociation).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) event_integration_association_arn: std::option::Option<std::string::String>,
        pub(crate) event_integration_association_id: std::option::Option<std::string::String>,
        pub(crate) event_integration_name: std::option::Option<std::string::String>,
        pub(crate) client_id: std::option::Option<std::string::String>,
        pub(crate) event_bridge_rule_name: std::option::Option<std::string::String>,
        pub(crate) client_association_metadata: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) for the event integration association.</p>
        pub fn event_integration_association_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.event_integration_association_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) for the event integration association.</p>
        pub fn set_event_integration_association_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.event_integration_association_arn = input; self
        }
        /// <p>The identifier for the event integration association.</p>
        pub fn event_integration_association_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.event_integration_association_id = Some(input.into());
            self
        }
        /// <p>The identifier for the event integration association.</p>
        pub fn set_event_integration_association_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.event_integration_association_id = input; self
        }
        /// <p>The name of the event integration.</p>
        pub fn event_integration_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.event_integration_name = Some(input.into());
            self
        }
        /// <p>The name of the event integration.</p>
        pub fn set_event_integration_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.event_integration_name = input; self
        }
        /// <p>The identifier for the client that is associated with the event integration.</p>
        pub fn client_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.client_id = Some(input.into());
            self
        }
        /// <p>The identifier for the client that is associated with the event integration.</p>
        pub fn set_client_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.client_id = input; self
        }
        /// <p>The name of the EventBridge rule.</p>
        pub fn event_bridge_rule_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.event_bridge_rule_name = Some(input.into());
            self
        }
        /// <p>The name of the EventBridge rule.</p>
        pub fn set_event_bridge_rule_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.event_bridge_rule_name = input; self
        }
        /// Adds a key-value pair to `client_association_metadata`.
        ///
        /// To override the contents of this collection use [`set_client_association_metadata`](Self::set_client_association_metadata).
        ///
        /// <p>The metadata associated with the client.</p>
        pub fn client_association_metadata(mut self, k: impl Into<std::string::String>, v: impl Into<std::string::String>) -> Self {
            let mut hash_map = self.client_association_metadata.unwrap_or_default();
                            hash_map.insert(k.into(), v.into());
                            self.client_association_metadata = Some(hash_map);
                            self
        }
        /// <p>The metadata associated with the client.</p>
        pub fn set_client_association_metadata(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>) -> Self {
            self.client_association_metadata = input; self
        }
        /// Consumes the builder and constructs a [`EventIntegrationAssociation`](crate::model::EventIntegrationAssociation).
        pub fn build(self) -> crate::model::EventIntegrationAssociation {
            crate::model::EventIntegrationAssociation {
                event_integration_association_arn: self.event_integration_association_arn
                ,
                event_integration_association_id: self.event_integration_association_id
                ,
                event_integration_name: self.event_integration_name
                ,
                client_id: self.client_id
                ,
                event_bridge_rule_name: self.event_bridge_rule_name
                ,
                client_association_metadata: self.client_association_metadata
                ,
            }
        }
    }
    
    
}
impl EventIntegrationAssociation {
    /// Creates a new builder-style object to manufacture [`EventIntegrationAssociation`](crate::model::EventIntegrationAssociation).
    pub fn builder() -> crate::model::event_integration_association::Builder {
        crate::model::event_integration_association::Builder::default()
    }
}

/// <p>Summary information about the DataIntegration.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct DataIntegrationSummary  {
    /// <p>The Amazon Resource Name (ARN) of the DataIntegration.</p>
    #[doc(hidden)]pub arn: std::option::Option<std::string::String>,
    /// <p>The name of the DataIntegration.</p>
    #[doc(hidden)]pub name: std::option::Option<std::string::String>,
    /// <p>The URI of the data source.</p>
    #[doc(hidden)]pub source_uri: std::option::Option<std::string::String>,
}
impl DataIntegrationSummary {
    /// <p>The Amazon Resource Name (ARN) of the DataIntegration.</p>
    pub fn arn(&self) -> std::option::Option<& str> {
        self.arn.as_deref()
    }
    /// <p>The name of the DataIntegration.</p>
    pub fn name(&self) -> std::option::Option<& str> {
        self.name.as_deref()
    }
    /// <p>The URI of the data source.</p>
    pub fn source_uri(&self) -> std::option::Option<& str> {
        self.source_uri.as_deref()
    }
}
impl  std::fmt::Debug for DataIntegrationSummary  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DataIntegrationSummary");
        formatter.field("arn", &self.arn);
        formatter.field("name", &self.name);
        formatter.field("source_uri", &self.source_uri);
        formatter.finish()
    }
}
/// See [`DataIntegrationSummary`](crate::model::DataIntegrationSummary).
pub mod data_integration_summary {
    
    /// A builder for [`DataIntegrationSummary`](crate::model::DataIntegrationSummary).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) arn: std::option::Option<std::string::String>,
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) source_uri: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) of the DataIntegration.</p>
        pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the DataIntegration.</p>
        pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.arn = input; self
        }
        /// <p>The name of the DataIntegration.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// <p>The name of the DataIntegration.</p>
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input; self
        }
        /// <p>The URI of the data source.</p>
        pub fn source_uri(mut self, input: impl Into<std::string::String>) -> Self {
            self.source_uri = Some(input.into());
            self
        }
        /// <p>The URI of the data source.</p>
        pub fn set_source_uri(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.source_uri = input; self
        }
        /// Consumes the builder and constructs a [`DataIntegrationSummary`](crate::model::DataIntegrationSummary).
        pub fn build(self) -> crate::model::DataIntegrationSummary {
            crate::model::DataIntegrationSummary {
                arn: self.arn
                ,
                name: self.name
                ,
                source_uri: self.source_uri
                ,
            }
        }
    }
    
    
}
impl DataIntegrationSummary {
    /// Creates a new builder-style object to manufacture [`DataIntegrationSummary`](crate::model::DataIntegrationSummary).
    pub fn builder() -> crate::model::data_integration_summary::Builder {
        crate::model::data_integration_summary::Builder::default()
    }
}

/// <p>Summary information about the DataIntegration association.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct DataIntegrationAssociationSummary  {
    /// <p>The Amazon Resource Name (ARN) of the DataIntegration association.</p>
    #[doc(hidden)]pub data_integration_association_arn: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN)of the DataIntegration.</p>
    #[doc(hidden)]pub data_integration_arn: std::option::Option<std::string::String>,
    /// <p>The identifier for teh client that is associated with the DataIntegration association.</p>
    #[doc(hidden)]pub client_id: std::option::Option<std::string::String>,
}
impl DataIntegrationAssociationSummary {
    /// <p>The Amazon Resource Name (ARN) of the DataIntegration association.</p>
    pub fn data_integration_association_arn(&self) -> std::option::Option<& str> {
        self.data_integration_association_arn.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN)of the DataIntegration.</p>
    pub fn data_integration_arn(&self) -> std::option::Option<& str> {
        self.data_integration_arn.as_deref()
    }
    /// <p>The identifier for teh client that is associated with the DataIntegration association.</p>
    pub fn client_id(&self) -> std::option::Option<& str> {
        self.client_id.as_deref()
    }
}
impl  std::fmt::Debug for DataIntegrationAssociationSummary  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DataIntegrationAssociationSummary");
        formatter.field("data_integration_association_arn", &self.data_integration_association_arn);
        formatter.field("data_integration_arn", &self.data_integration_arn);
        formatter.field("client_id", &self.client_id);
        formatter.finish()
    }
}
/// See [`DataIntegrationAssociationSummary`](crate::model::DataIntegrationAssociationSummary).
pub mod data_integration_association_summary {
    
    /// A builder for [`DataIntegrationAssociationSummary`](crate::model::DataIntegrationAssociationSummary).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) data_integration_association_arn: std::option::Option<std::string::String>,
        pub(crate) data_integration_arn: std::option::Option<std::string::String>,
        pub(crate) client_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) of the DataIntegration association.</p>
        pub fn data_integration_association_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.data_integration_association_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the DataIntegration association.</p>
        pub fn set_data_integration_association_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.data_integration_association_arn = input; self
        }
        /// <p>The Amazon Resource Name (ARN)of the DataIntegration.</p>
        pub fn data_integration_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.data_integration_arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN)of the DataIntegration.</p>
        pub fn set_data_integration_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.data_integration_arn = input; self
        }
        /// <p>The identifier for teh client that is associated with the DataIntegration association.</p>
        pub fn client_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.client_id = Some(input.into());
            self
        }
        /// <p>The identifier for teh client that is associated with the DataIntegration association.</p>
        pub fn set_client_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.client_id = input; self
        }
        /// Consumes the builder and constructs a [`DataIntegrationAssociationSummary`](crate::model::DataIntegrationAssociationSummary).
        pub fn build(self) -> crate::model::DataIntegrationAssociationSummary {
            crate::model::DataIntegrationAssociationSummary {
                data_integration_association_arn: self.data_integration_association_arn
                ,
                data_integration_arn: self.data_integration_arn
                ,
                client_id: self.client_id
                ,
            }
        }
    }
    
    
}
impl DataIntegrationAssociationSummary {
    /// Creates a new builder-style object to manufacture [`DataIntegrationAssociationSummary`](crate::model::DataIntegrationAssociationSummary).
    pub fn builder() -> crate::model::data_integration_association_summary::Builder {
        crate::model::data_integration_association_summary::Builder::default()
    }
}

/// <p>The name of the data and how often it should be pulled from the source.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct ScheduleConfiguration  {
    /// <p>The start date for objects to import in the first flow run.</p>
    #[doc(hidden)]pub first_execution_from: std::option::Option<std::string::String>,
    /// <p>The name of the object to pull from the data source.</p>
    #[doc(hidden)]pub object: std::option::Option<std::string::String>,
    /// <p>How often the data should be pulled from data source.</p>
    #[doc(hidden)]pub schedule_expression: std::option::Option<std::string::String>,
}
impl ScheduleConfiguration {
    /// <p>The start date for objects to import in the first flow run.</p>
    pub fn first_execution_from(&self) -> std::option::Option<& str> {
        self.first_execution_from.as_deref()
    }
    /// <p>The name of the object to pull from the data source.</p>
    pub fn object(&self) -> std::option::Option<& str> {
        self.object.as_deref()
    }
    /// <p>How often the data should be pulled from data source.</p>
    pub fn schedule_expression(&self) -> std::option::Option<& str> {
        self.schedule_expression.as_deref()
    }
}
impl  std::fmt::Debug for ScheduleConfiguration  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ScheduleConfiguration");
        formatter.field("first_execution_from", &self.first_execution_from);
        formatter.field("object", &self.object);
        formatter.field("schedule_expression", &self.schedule_expression);
        formatter.finish()
    }
}
/// See [`ScheduleConfiguration`](crate::model::ScheduleConfiguration).
pub mod schedule_configuration {
    
    /// A builder for [`ScheduleConfiguration`](crate::model::ScheduleConfiguration).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) first_execution_from: std::option::Option<std::string::String>,
        pub(crate) object: std::option::Option<std::string::String>,
        pub(crate) schedule_expression: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The start date for objects to import in the first flow run.</p>
        pub fn first_execution_from(mut self, input: impl Into<std::string::String>) -> Self {
            self.first_execution_from = Some(input.into());
            self
        }
        /// <p>The start date for objects to import in the first flow run.</p>
        pub fn set_first_execution_from(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.first_execution_from = input; self
        }
        /// <p>The name of the object to pull from the data source.</p>
        pub fn object(mut self, input: impl Into<std::string::String>) -> Self {
            self.object = Some(input.into());
            self
        }
        /// <p>The name of the object to pull from the data source.</p>
        pub fn set_object(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.object = input; self
        }
        /// <p>How often the data should be pulled from data source.</p>
        pub fn schedule_expression(mut self, input: impl Into<std::string::String>) -> Self {
            self.schedule_expression = Some(input.into());
            self
        }
        /// <p>How often the data should be pulled from data source.</p>
        pub fn set_schedule_expression(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.schedule_expression = input; self
        }
        /// Consumes the builder and constructs a [`ScheduleConfiguration`](crate::model::ScheduleConfiguration).
        pub fn build(self) -> crate::model::ScheduleConfiguration {
            crate::model::ScheduleConfiguration {
                first_execution_from: self.first_execution_from
                ,
                object: self.object
                ,
                schedule_expression: self.schedule_expression
                ,
            }
        }
    }
    
    
}
impl ScheduleConfiguration {
    /// Creates a new builder-style object to manufacture [`ScheduleConfiguration`](crate::model::ScheduleConfiguration).
    pub fn builder() -> crate::model::schedule_configuration::Builder {
        crate::model::schedule_configuration::Builder::default()
    }
}

