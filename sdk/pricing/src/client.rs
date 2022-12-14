// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
            pub(crate) struct Handle {
                pub(crate) client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>,
                pub(crate) conf: crate::Config,
            }

            /// Client for AWS Price List Service
                    ///
                    /// Client for invoking operations on AWS Price List Service. Each operation on AWS Price List Service is a method on this
                    /// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
                        /// # Examples
                        /// **Constructing a client and invoking an operation**
                        /// ```rust,no_run
                        /// # async fn docs() {
                        ///     // create a shared configuration. This can be used & shared between multiple service clients.
                        ///     let shared_config = aws_config::load_from_env().await;
                        ///     let client = aws_sdk_pricing::Client::new(&shared_config);
                        ///     // invoke an operation
                        ///     /* let rsp = client
                        ///         .<operation_name>().
                        ///         .<param>("some value")
                        ///         .send().await; */
                        /// # }
                        /// ```
                        /// **Constructing a client with custom configuration**
                        /// ```rust,no_run
                        /// use aws_config::retry::RetryConfig;
                        /// # async fn docs() {
                        /// let shared_config = aws_config::load_from_env().await;
                        /// let config = aws_sdk_pricing::config::Builder::from(&shared_config)
                        ///   .retry_config(RetryConfig::disabled())
                        ///   .build();
                        /// let client = aws_sdk_pricing::Client::from_conf(config);
                        /// # }
            #[derive(std::fmt::Debug)]
            pub struct Client {
                handle: std::sync::Arc<Handle>
            }

            impl std::clone::Clone for Client {
                fn clone(&self) -> Self {
                    Self { handle: self.handle.clone() }
                }
            }

            #[doc(inline)]
            pub use aws_smithy_client::Builder;

            impl From<aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>> for Client {
                fn from(client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>) -> Self {
                    Self::with_config(client, crate::Config::builder().build())
                }
            }

            impl Client {
                /// Creates a client with the given service configuration.
                pub fn with_config(client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>, conf: crate::Config) -> Self {
                    Self {
                        handle: std::sync::Arc::new(Handle {
                            client,
                            conf,
                        })
                    }
                }

                /// Returns the client's configuration.
                pub fn conf(&self) -> &crate::Config {
                    &self.handle.conf
                }
            }
impl Client  {
    /// Constructs a fluent builder for the [`DescribeServices`](crate::client::fluent_builders::DescribeServices) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::DescribeServices::into_paginator).
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`service_code(impl Into<String>)`](crate::client::fluent_builders::DescribeServices::service_code) / [`set_service_code(Option<String>)`](crate::client::fluent_builders::DescribeServices::set_service_code): <p>The code for the service whose information you want to retrieve, such as <code>AmazonEC2</code>. You can use the <code>ServiceCode</code> to filter the results in a <code>GetProducts</code> call. To retrieve a list of all services, leave this blank.</p>
    ///   - [`format_version(impl Into<String>)`](crate::client::fluent_builders::DescribeServices::format_version) / [`set_format_version(Option<String>)`](crate::client::fluent_builders::DescribeServices::set_format_version): <p>The format version that you want the response to be in.</p>  <p>Valid values are: <code>aws_v1</code> </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::DescribeServices::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::DescribeServices::set_next_token): <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::DescribeServices::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::DescribeServices::set_max_results): <p>The maximum number of results that you want returned in the response.</p>
                        /// - On success, responds with [`DescribeServicesOutput`](crate::output::DescribeServicesOutput) with field(s):
                        ///   - [`services(Option<Vec<Service>>)`](crate::output::DescribeServicesOutput::services): <p>The service metadata for the service or services in the response.</p>
    ///   - [`format_version(Option<String>)`](crate::output::DescribeServicesOutput::format_version): <p>The format version of the response. For example, <code>aws_v1</code>.</p>
    ///   - [`next_token(Option<String>)`](crate::output::DescribeServicesOutput::next_token): <p>The pagination token for the next set of retrievable results.</p>
                        /// - On failure, responds with [`SdkError<DescribeServicesError>`](crate::error::DescribeServicesError)
    pub fn describe_services(&self) -> fluent_builders::DescribeServices {
                            fluent_builders::DescribeServices::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`GetAttributeValues`](crate::client::fluent_builders::GetAttributeValues) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetAttributeValues::into_paginator).
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`service_code(impl Into<String>)`](crate::client::fluent_builders::GetAttributeValues::service_code) / [`set_service_code(Option<String>)`](crate::client::fluent_builders::GetAttributeValues::set_service_code): <p>The service code for the service whose attributes you want to retrieve. For example, if you want the retrieve an EC2 attribute, use <code>AmazonEC2</code>.</p>
    ///   - [`attribute_name(impl Into<String>)`](crate::client::fluent_builders::GetAttributeValues::attribute_name) / [`set_attribute_name(Option<String>)`](crate::client::fluent_builders::GetAttributeValues::set_attribute_name): <p>The name of the attribute that you want to retrieve the values for, such as <code>volumeType</code>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetAttributeValues::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetAttributeValues::set_next_token): <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetAttributeValues::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetAttributeValues::set_max_results): <p>The maximum number of results to return in response.</p>
                        /// - On success, responds with [`GetAttributeValuesOutput`](crate::output::GetAttributeValuesOutput) with field(s):
                        ///   - [`attribute_values(Option<Vec<AttributeValue>>)`](crate::output::GetAttributeValuesOutput::attribute_values): <p>The list of values for an attribute. For example, <code>Throughput Optimized HDD</code> and <code>Provisioned IOPS</code> are two available values for the <code>AmazonEC2</code> <code>volumeType</code>.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetAttributeValuesOutput::next_token): <p>The pagination token that indicates the next set of results to retrieve.</p>
                        /// - On failure, responds with [`SdkError<GetAttributeValuesError>`](crate::error::GetAttributeValuesError)
    pub fn get_attribute_values(&self) -> fluent_builders::GetAttributeValues {
                            fluent_builders::GetAttributeValues::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`GetProducts`](crate::client::fluent_builders::GetProducts) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::GetProducts::into_paginator).
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`service_code(impl Into<String>)`](crate::client::fluent_builders::GetProducts::service_code) / [`set_service_code(Option<String>)`](crate::client::fluent_builders::GetProducts::set_service_code): <p>The code for the service whose products you want to retrieve. </p>
    ///   - [`filters(Vec<Filter>)`](crate::client::fluent_builders::GetProducts::filters) / [`set_filters(Option<Vec<Filter>>)`](crate::client::fluent_builders::GetProducts::set_filters): <p>The list of filters that limit the returned products. only products that match all filters are returned.</p>
    ///   - [`format_version(impl Into<String>)`](crate::client::fluent_builders::GetProducts::format_version) / [`set_format_version(Option<String>)`](crate::client::fluent_builders::GetProducts::set_format_version): <p>The format version that you want the response to be in.</p>  <p>Valid values are: <code>aws_v1</code> </p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetProducts::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetProducts::set_next_token): <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetProducts::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetProducts::set_max_results): <p>The maximum number of results to return in the response.</p>
                        /// - On success, responds with [`GetProductsOutput`](crate::output::GetProductsOutput) with field(s):
                        ///   - [`format_version(Option<String>)`](crate::output::GetProductsOutput::format_version): <p>The format version of the response. For example, aws_v1.</p>
    ///   - [`price_list(Option<Vec<String>>)`](crate::output::GetProductsOutput::price_list): <p>The list of products that match your filters. The list contains both the product metadata and the price information.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetProductsOutput::next_token): <p>The pagination token that indicates the next set of results to retrieve.</p>
                        /// - On failure, responds with [`SdkError<GetProductsError>`](crate::error::GetProductsError)
    pub fn get_products(&self) -> fluent_builders::GetProducts {
                            fluent_builders::GetProducts::new(self.handle.clone())
                        }
}
pub mod fluent_builders {
    
    //! Utilities to ergonomically construct a request to the service.
    //! 
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `DescribeServices`.
                        ///
    /// <p>Returns the metadata for one service or a list of the metadata for all services. Use this without a service code to get the service codes for all services. Use it with a service code, such as <code>AmazonEC2</code>, to get information specific to that service, such as the attribute names available for that service. For example, some of the attribute names available for EC2 are <code>volumeType</code>, <code>maxIopsVolume</code>, <code>operation</code>, <code>locationType</code>, and <code>instanceCapacity10xlarge</code>.</p>
    #[derive(std::clone::Clone, std::fmt::Debug, )]
    pub struct DescribeServices {
                            handle: std::sync::Arc<super::Handle>,
                            inner: crate::input::describe_services_input::Builder
                        }
    impl DescribeServices  {
        /// Creates a new `DescribeServices`.
                                pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
                                    Self { handle, inner: Default::default() }
                                }
        
                                /// Consume this builder, creating a customizable operation that can be modified before being
                                /// sent. The operation's inner [http::Request] can be modified as well.
                                pub async fn customize(self) -> std::result::Result<
                                    crate::operation::customize::CustomizableOperation<crate::operation::DescribeServices, aws_http::retry::AwsResponseRetryClassifier,>,
                                    aws_smithy_http::result::SdkError<crate::error::DescribeServicesError>
                                >  {
                                    let handle = self.handle.clone();
                                    let operation = self.inner.build().map_err(|err|aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                                        .make_operation(&handle.conf)
                                        .await
                                        .map_err(|err|aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
                                    Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                                }
        
                                /// Sends the request and returns the response.
                                ///
                                /// If an error occurs, an `SdkError` will be returned with additional details that
                                /// can be matched against.
                                ///
                                /// By default, any retryable failures will be retried twice. Retry behavior
                                /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                                /// set when configuring the client.
                                pub async fn send(self) -> std::result::Result<crate::output::DescribeServicesOutput, aws_smithy_http::result::SdkError<crate::error::DescribeServicesError>>
                                 {
                                    let op = self.inner.build().map_err(|err|aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                                        .make_operation(&self.handle.conf)
                                        .await
                                        .map_err(|err|aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
                                    self.handle.client.call(op).await
                                }
        /// Create a paginator for this request
                                    ///
                                    /// Paginators are used by calling [`send().await`](crate::paginator::DescribeServicesPaginator::send) which returns a [`Stream`](tokio_stream::Stream).
                                    pub fn into_paginator(self) -> crate::paginator::DescribeServicesPaginator {
                                        crate::paginator::DescribeServicesPaginator::new(self.handle, self.inner)
                                    }
        /// <p>The code for the service whose information you want to retrieve, such as <code>AmazonEC2</code>. You can use the <code>ServiceCode</code> to filter the results in a <code>GetProducts</code> call. To retrieve a list of all services, leave this blank.</p>
        pub fn service_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.service_code(input.into());
            self
        }
        /// <p>The code for the service whose information you want to retrieve, such as <code>AmazonEC2</code>. You can use the <code>ServiceCode</code> to filter the results in a <code>GetProducts</code> call. To retrieve a list of all services, leave this blank.</p>
        pub fn set_service_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_service_code(input);
            self
        }
        /// <p>The format version that you want the response to be in.</p> 
        /// <p>Valid values are: <code>aws_v1</code> </p>
        pub fn format_version(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.format_version(input.into());
            self
        }
        /// <p>The format version that you want the response to be in.</p> 
        /// <p>Valid values are: <code>aws_v1</code> </p>
        pub fn set_format_version(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_format_version(input);
            self
        }
        /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum number of results that you want returned in the response.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>The maximum number of results that you want returned in the response.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetAttributeValues`.
                        ///
    /// <p>Returns a list of attribute values. Attributes are similar to the details in a Price List API offer file. For a list of available attributes, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/reading-an-offer.html#pps-defs">Offer File Definitions</a> in the <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/billing-what-is.html">Billing and Cost Management User Guide</a>.</p>
    #[derive(std::clone::Clone, std::fmt::Debug, )]
    pub struct GetAttributeValues {
                            handle: std::sync::Arc<super::Handle>,
                            inner: crate::input::get_attribute_values_input::Builder
                        }
    impl GetAttributeValues  {
        /// Creates a new `GetAttributeValues`.
                                pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
                                    Self { handle, inner: Default::default() }
                                }
        
                                /// Consume this builder, creating a customizable operation that can be modified before being
                                /// sent. The operation's inner [http::Request] can be modified as well.
                                pub async fn customize(self) -> std::result::Result<
                                    crate::operation::customize::CustomizableOperation<crate::operation::GetAttributeValues, aws_http::retry::AwsResponseRetryClassifier,>,
                                    aws_smithy_http::result::SdkError<crate::error::GetAttributeValuesError>
                                >  {
                                    let handle = self.handle.clone();
                                    let operation = self.inner.build().map_err(|err|aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                                        .make_operation(&handle.conf)
                                        .await
                                        .map_err(|err|aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
                                    Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                                }
        
                                /// Sends the request and returns the response.
                                ///
                                /// If an error occurs, an `SdkError` will be returned with additional details that
                                /// can be matched against.
                                ///
                                /// By default, any retryable failures will be retried twice. Retry behavior
                                /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                                /// set when configuring the client.
                                pub async fn send(self) -> std::result::Result<crate::output::GetAttributeValuesOutput, aws_smithy_http::result::SdkError<crate::error::GetAttributeValuesError>>
                                 {
                                    let op = self.inner.build().map_err(|err|aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                                        .make_operation(&self.handle.conf)
                                        .await
                                        .map_err(|err|aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
                                    self.handle.client.call(op).await
                                }
        /// Create a paginator for this request
                                    ///
                                    /// Paginators are used by calling [`send().await`](crate::paginator::GetAttributeValuesPaginator::send) which returns a [`Stream`](tokio_stream::Stream).
                                    pub fn into_paginator(self) -> crate::paginator::GetAttributeValuesPaginator {
                                        crate::paginator::GetAttributeValuesPaginator::new(self.handle, self.inner)
                                    }
        /// <p>The service code for the service whose attributes you want to retrieve. For example, if you want the retrieve an EC2 attribute, use <code>AmazonEC2</code>.</p>
        pub fn service_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.service_code(input.into());
            self
        }
        /// <p>The service code for the service whose attributes you want to retrieve. For example, if you want the retrieve an EC2 attribute, use <code>AmazonEC2</code>.</p>
        pub fn set_service_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_service_code(input);
            self
        }
        /// <p>The name of the attribute that you want to retrieve the values for, such as <code>volumeType</code>.</p>
        pub fn attribute_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.attribute_name(input.into());
            self
        }
        /// <p>The name of the attribute that you want to retrieve the values for, such as <code>volumeType</code>.</p>
        pub fn set_attribute_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_attribute_name(input);
            self
        }
        /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum number of results to return in response.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>The maximum number of results to return in response.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetProducts`.
                        ///
    /// <p>Returns a list of all products that match the filter criteria.</p>
    #[derive(std::clone::Clone, std::fmt::Debug, )]
    pub struct GetProducts {
                            handle: std::sync::Arc<super::Handle>,
                            inner: crate::input::get_products_input::Builder
                        }
    impl GetProducts  {
        /// Creates a new `GetProducts`.
                                pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
                                    Self { handle, inner: Default::default() }
                                }
        
                                /// Consume this builder, creating a customizable operation that can be modified before being
                                /// sent. The operation's inner [http::Request] can be modified as well.
                                pub async fn customize(self) -> std::result::Result<
                                    crate::operation::customize::CustomizableOperation<crate::operation::GetProducts, aws_http::retry::AwsResponseRetryClassifier,>,
                                    aws_smithy_http::result::SdkError<crate::error::GetProductsError>
                                >  {
                                    let handle = self.handle.clone();
                                    let operation = self.inner.build().map_err(|err|aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                                        .make_operation(&handle.conf)
                                        .await
                                        .map_err(|err|aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
                                    Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                                }
        
                                /// Sends the request and returns the response.
                                ///
                                /// If an error occurs, an `SdkError` will be returned with additional details that
                                /// can be matched against.
                                ///
                                /// By default, any retryable failures will be retried twice. Retry behavior
                                /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                                /// set when configuring the client.
                                pub async fn send(self) -> std::result::Result<crate::output::GetProductsOutput, aws_smithy_http::result::SdkError<crate::error::GetProductsError>>
                                 {
                                    let op = self.inner.build().map_err(|err|aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                                        .make_operation(&self.handle.conf)
                                        .await
                                        .map_err(|err|aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
                                    self.handle.client.call(op).await
                                }
        /// Create a paginator for this request
                                    ///
                                    /// Paginators are used by calling [`send().await`](crate::paginator::GetProductsPaginator::send) which returns a [`Stream`](tokio_stream::Stream).
                                    pub fn into_paginator(self) -> crate::paginator::GetProductsPaginator {
                                        crate::paginator::GetProductsPaginator::new(self.handle, self.inner)
                                    }
        /// <p>The code for the service whose products you want to retrieve. </p>
        pub fn service_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.service_code(input.into());
            self
        }
        /// <p>The code for the service whose products you want to retrieve. </p>
        pub fn set_service_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_service_code(input);
            self
        }
        /// Appends an item to `Filters`.
        ///
        /// To override the contents of this collection use [`set_filters`](Self::set_filters).
        ///
        /// <p>The list of filters that limit the returned products. only products that match all filters are returned.</p>
        pub fn filters(mut self, input: crate::model::Filter) -> Self {
            self.inner = self.inner.filters(input);
            self
        }
        /// <p>The list of filters that limit the returned products. only products that match all filters are returned.</p>
        pub fn set_filters(mut self, input: std::option::Option<std::vec::Vec<crate::model::Filter>>) -> Self {
            self.inner = self.inner.set_filters(input);
            self
        }
        /// <p>The format version that you want the response to be in.</p> 
        /// <p>Valid values are: <code>aws_v1</code> </p>
        pub fn format_version(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.format_version(input.into());
            self
        }
        /// <p>The format version that you want the response to be in.</p> 
        /// <p>Valid values are: <code>aws_v1</code> </p>
        pub fn set_format_version(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_format_version(input);
            self
        }
        /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum number of results to return in the response.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>The maximum number of results to return in the response.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    
    
}

impl Client {
    /// Creates a client with the given service config and connector override.
                    pub fn from_conf_conn<C, E>(conf: crate::Config, conn: C) -> Self
                    where
                        C: aws_smithy_client::bounds::SmithyConnector<Error = E> + Send + 'static,
                        E: Into<aws_smithy_http::result::ConnectorError>,
                    {
                        let retry_config = conf.retry_config().cloned().unwrap_or_else(aws_smithy_types::retry::RetryConfig::disabled);
                        let timeout_config = conf.timeout_config().cloned().unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
                        let mut builder = aws_smithy_client::Builder::new()
                            .connector(aws_smithy_client::erase::DynConnector::new(conn))
                            .middleware(aws_smithy_client::erase::DynMiddleware::new(crate::middleware::DefaultMiddleware::new()))
                            .retry_config(retry_config.into())
                            .operation_timeout_config(timeout_config.into());
                        builder.set_sleep_impl(conf.sleep_impl());
                        let client = builder.build();
                        Self { handle: std::sync::Arc::new(Handle { client, conf }) }
                    }
    
                    /// Creates a new client from a shared config.
                    #[cfg(any(feature = "rustls", feature = "native-tls"))]
                    pub fn new(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
                        Self::from_conf(sdk_config.into())
                    }
    
                    /// Creates a new client from the service [`Config`](crate::Config).
                    #[cfg(any(feature = "rustls", feature = "native-tls"))]
                    pub fn from_conf(conf: crate::Config) -> Self {
                        let retry_config = conf.retry_config().cloned().unwrap_or_else(aws_smithy_types::retry::RetryConfig::disabled);
                        let timeout_config = conf.timeout_config().cloned().unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
                        let sleep_impl = conf.sleep_impl();
                        if (retry_config.has_retry() || timeout_config.has_timeouts()) && sleep_impl.is_none() {
                            panic!("An async sleep implementation is required for retries or timeouts to work. \
                                    Set the `sleep_impl` on the Config passed into this function to fix this panic.");
                        }
                        let mut builder = aws_smithy_client::Builder::new()
                            .dyn_https_connector(aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(&timeout_config))
                            .middleware(aws_smithy_client::erase::DynMiddleware::new(crate::middleware::DefaultMiddleware::new()))
                            .retry_config(retry_config.into())
                            .operation_timeout_config(timeout_config.into());
                        builder.set_sleep_impl(sleep_impl);
                        let client = builder.build();
    
                        Self { handle: std::sync::Arc::new(Handle { client, conf }) }
                    }
}

