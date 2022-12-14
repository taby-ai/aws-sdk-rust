// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
            pub(crate) struct Handle {
                pub(crate) client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>,
                pub(crate) conf: crate::Config,
            }

            /// Client for Amazon Personalize Runtime
                    ///
                    /// Client for invoking operations on Amazon Personalize Runtime. Each operation on Amazon Personalize Runtime is a method on this
                    /// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
                        /// # Examples
                        /// **Constructing a client and invoking an operation**
                        /// ```rust,no_run
                        /// # async fn docs() {
                        ///     // create a shared configuration. This can be used & shared between multiple service clients.
                        ///     let shared_config = aws_config::load_from_env().await;
                        ///     let client = aws_sdk_personalizeruntime::Client::new(&shared_config);
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
                        /// let config = aws_sdk_personalizeruntime::config::Builder::from(&shared_config)
                        ///   .retry_config(RetryConfig::disabled())
                        ///   .build();
                        /// let client = aws_sdk_personalizeruntime::Client::from_conf(config);
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
    /// Constructs a fluent builder for the [`GetPersonalizedRanking`](crate::client::fluent_builders::GetPersonalizedRanking) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`campaign_arn(impl Into<String>)`](crate::client::fluent_builders::GetPersonalizedRanking::campaign_arn) / [`set_campaign_arn(Option<String>)`](crate::client::fluent_builders::GetPersonalizedRanking::set_campaign_arn): <p>The Amazon Resource Name (ARN) of the campaign to use for generating the personalized ranking.</p>
    ///   - [`input_list(Vec<String>)`](crate::client::fluent_builders::GetPersonalizedRanking::input_list) / [`set_input_list(Option<Vec<String>>)`](crate::client::fluent_builders::GetPersonalizedRanking::set_input_list): <p>A list of items (by <code>itemId</code>) to rank. If an item was not included in the training dataset, the item is appended to the end of the reranked list. The maximum is 500.</p>
    ///   - [`user_id(impl Into<String>)`](crate::client::fluent_builders::GetPersonalizedRanking::user_id) / [`set_user_id(Option<String>)`](crate::client::fluent_builders::GetPersonalizedRanking::set_user_id): <p>The user for which you want the campaign to provide a personalized ranking.</p>
    ///   - [`context(HashMap<String, String>)`](crate::client::fluent_builders::GetPersonalizedRanking::context) / [`set_context(Option<HashMap<String, String>>)`](crate::client::fluent_builders::GetPersonalizedRanking::set_context): <p>The contextual metadata to use when getting recommendations. Contextual metadata includes any interaction information that might be relevant when getting a user's recommendations, such as the user's current location or device type.</p>
    ///   - [`filter_arn(impl Into<String>)`](crate::client::fluent_builders::GetPersonalizedRanking::filter_arn) / [`set_filter_arn(Option<String>)`](crate::client::fluent_builders::GetPersonalizedRanking::set_filter_arn): <p>The Amazon Resource Name (ARN) of a filter you created to include items or exclude items from recommendations for a given user. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
    ///   - [`filter_values(HashMap<String, String>)`](crate::client::fluent_builders::GetPersonalizedRanking::filter_values) / [`set_filter_values(Option<HashMap<String, String>>)`](crate::client::fluent_builders::GetPersonalizedRanking::set_filter_values): <p>The values to use when filtering recommendations. For each placeholder parameter in your filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p>  <p>For filter expressions that use an <code>INCLUDE</code> element to include items, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you can omit the <code>filter-values</code>.In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
                        /// - On success, responds with [`GetPersonalizedRankingOutput`](crate::output::GetPersonalizedRankingOutput) with field(s):
                        ///   - [`personalized_ranking(Option<Vec<PredictedItem>>)`](crate::output::GetPersonalizedRankingOutput::personalized_ranking): <p>A list of items in order of most likely interest to the user. The maximum is 500.</p>
    ///   - [`recommendation_id(Option<String>)`](crate::output::GetPersonalizedRankingOutput::recommendation_id): <p>The ID of the recommendation.</p>
                        /// - On failure, responds with [`SdkError<GetPersonalizedRankingError>`](crate::error::GetPersonalizedRankingError)
    pub fn get_personalized_ranking(&self) -> fluent_builders::GetPersonalizedRanking {
                            fluent_builders::GetPersonalizedRanking::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`GetRecommendations`](crate::client::fluent_builders::GetRecommendations) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`campaign_arn(impl Into<String>)`](crate::client::fluent_builders::GetRecommendations::campaign_arn) / [`set_campaign_arn(Option<String>)`](crate::client::fluent_builders::GetRecommendations::set_campaign_arn): <p>The Amazon Resource Name (ARN) of the campaign to use for getting recommendations.</p>
    ///   - [`item_id(impl Into<String>)`](crate::client::fluent_builders::GetRecommendations::item_id) / [`set_item_id(Option<String>)`](crate::client::fluent_builders::GetRecommendations::set_item_id): <p>The item ID to provide recommendations for.</p>  <p>Required for <code>RELATED_ITEMS</code> recipe type.</p>
    ///   - [`user_id(impl Into<String>)`](crate::client::fluent_builders::GetRecommendations::user_id) / [`set_user_id(Option<String>)`](crate::client::fluent_builders::GetRecommendations::set_user_id): <p>The user ID to provide recommendations for.</p>  <p>Required for <code>USER_PERSONALIZATION</code> recipe type.</p>
    ///   - [`num_results(i32)`](crate::client::fluent_builders::GetRecommendations::num_results) / [`set_num_results(i32)`](crate::client::fluent_builders::GetRecommendations::set_num_results): <p>The number of results to return. The default is 25. The maximum is 500.</p>
    ///   - [`context(HashMap<String, String>)`](crate::client::fluent_builders::GetRecommendations::context) / [`set_context(Option<HashMap<String, String>>)`](crate::client::fluent_builders::GetRecommendations::set_context): <p>The contextual metadata to use when getting recommendations. Contextual metadata includes any interaction information that might be relevant when getting a user's recommendations, such as the user's current location or device type.</p>
    ///   - [`filter_arn(impl Into<String>)`](crate::client::fluent_builders::GetRecommendations::filter_arn) / [`set_filter_arn(Option<String>)`](crate::client::fluent_builders::GetRecommendations::set_filter_arn): <p>The ARN of the filter to apply to the returned recommendations. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>  <p>When using this parameter, be sure the filter resource is <code>ACTIVE</code>.</p>
    ///   - [`filter_values(HashMap<String, String>)`](crate::client::fluent_builders::GetRecommendations::filter_values) / [`set_filter_values(Option<HashMap<String, String>>)`](crate::client::fluent_builders::GetRecommendations::set_filter_values): <p>The values to use when filtering recommendations. For each placeholder parameter in your filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p>  <p>For filter expressions that use an <code>INCLUDE</code> element to include items, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you can omit the <code>filter-values</code>.In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering recommendations and user segments</a>.</p>
    ///   - [`recommender_arn(impl Into<String>)`](crate::client::fluent_builders::GetRecommendations::recommender_arn) / [`set_recommender_arn(Option<String>)`](crate::client::fluent_builders::GetRecommendations::set_recommender_arn): <p>The Amazon Resource Name (ARN) of the recommender to use to get recommendations. Provide a recommender ARN if you created a Domain dataset group with a recommender for a domain use case.</p>
    ///   - [`promotions(Vec<Promotion>)`](crate::client::fluent_builders::GetRecommendations::promotions) / [`set_promotions(Option<Vec<Promotion>>)`](crate::client::fluent_builders::GetRecommendations::set_promotions): <p>The promotions to apply to the recommendation request. A promotion defines additional business rules that apply to a configurable subset of recommended items.</p>
                        /// - On success, responds with [`GetRecommendationsOutput`](crate::output::GetRecommendationsOutput) with field(s):
                        ///   - [`item_list(Option<Vec<PredictedItem>>)`](crate::output::GetRecommendationsOutput::item_list): <p>A list of recommendations sorted in descending order by prediction score. There can be a maximum of 500 items in the list.</p>
    ///   - [`recommendation_id(Option<String>)`](crate::output::GetRecommendationsOutput::recommendation_id): <p>The ID of the recommendation.</p>
                        /// - On failure, responds with [`SdkError<GetRecommendationsError>`](crate::error::GetRecommendationsError)
    pub fn get_recommendations(&self) -> fluent_builders::GetRecommendations {
                            fluent_builders::GetRecommendations::new(self.handle.clone())
                        }
}
pub mod fluent_builders {
    
    //! Utilities to ergonomically construct a request to the service.
    //! 
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `GetPersonalizedRanking`.
                        ///
    /// <p>Re-ranks a list of recommended items for the given user. The first item in the list is deemed the most likely item to be of interest to the user.</p> <note> 
    /// <p>The solution backing the campaign must have been created using a recipe of type PERSONALIZED_RANKING.</p> 
    /// </note>
    #[derive(std::clone::Clone, std::fmt::Debug, )]
    pub struct GetPersonalizedRanking {
                            handle: std::sync::Arc<super::Handle>,
                            inner: crate::input::get_personalized_ranking_input::Builder
                        }
    impl GetPersonalizedRanking  {
        /// Creates a new `GetPersonalizedRanking`.
                                pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
                                    Self { handle, inner: Default::default() }
                                }
        
                                /// Consume this builder, creating a customizable operation that can be modified before being
                                /// sent. The operation's inner [http::Request] can be modified as well.
                                pub async fn customize(self) -> std::result::Result<
                                    crate::operation::customize::CustomizableOperation<crate::operation::GetPersonalizedRanking, aws_http::retry::AwsResponseRetryClassifier,>,
                                    aws_smithy_http::result::SdkError<crate::error::GetPersonalizedRankingError>
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
                                pub async fn send(self) -> std::result::Result<crate::output::GetPersonalizedRankingOutput, aws_smithy_http::result::SdkError<crate::error::GetPersonalizedRankingError>>
                                 {
                                    let op = self.inner.build().map_err(|err|aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                                        .make_operation(&self.handle.conf)
                                        .await
                                        .map_err(|err|aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
                                    self.handle.client.call(op).await
                                }
        /// <p>The Amazon Resource Name (ARN) of the campaign to use for generating the personalized ranking.</p>
        pub fn campaign_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.campaign_arn(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the campaign to use for generating the personalized ranking.</p>
        pub fn set_campaign_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_campaign_arn(input);
            self
        }
        /// Appends an item to `inputList`.
        ///
        /// To override the contents of this collection use [`set_input_list`](Self::set_input_list).
        ///
        /// <p>A list of items (by <code>itemId</code>) to rank. If an item was not included in the training dataset, the item is appended to the end of the reranked list. The maximum is 500.</p>
        pub fn input_list(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.input_list(input.into());
            self
        }
        /// <p>A list of items (by <code>itemId</code>) to rank. If an item was not included in the training dataset, the item is appended to the end of the reranked list. The maximum is 500.</p>
        pub fn set_input_list(mut self, input: std::option::Option<std::vec::Vec<std::string::String>>) -> Self {
            self.inner = self.inner.set_input_list(input);
            self
        }
        /// <p>The user for which you want the campaign to provide a personalized ranking.</p>
        pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.user_id(input.into());
            self
        }
        /// <p>The user for which you want the campaign to provide a personalized ranking.</p>
        pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_user_id(input);
            self
        }
        /// Adds a key-value pair to `context`.
        ///
        /// To override the contents of this collection use [`set_context`](Self::set_context).
        ///
        /// <p>The contextual metadata to use when getting recommendations. Contextual metadata includes any interaction information that might be relevant when getting a user's recommendations, such as the user's current location or device type.</p>
        pub fn context(mut self, k: impl Into<std::string::String>, v: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.context(k.into(), v.into());
            self
        }
        /// <p>The contextual metadata to use when getting recommendations. Contextual metadata includes any interaction information that might be relevant when getting a user's recommendations, such as the user's current location or device type.</p>
        pub fn set_context(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>) -> Self {
            self.inner = self.inner.set_context(input);
            self
        }
        /// <p>The Amazon Resource Name (ARN) of a filter you created to include items or exclude items from recommendations for a given user. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
        pub fn filter_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.filter_arn(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of a filter you created to include items or exclude items from recommendations for a given user. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
        pub fn set_filter_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_filter_arn(input);
            self
        }
        /// Adds a key-value pair to `filterValues`.
        ///
        /// To override the contents of this collection use [`set_filter_values`](Self::set_filter_values).
        ///
        /// <p>The values to use when filtering recommendations. For each placeholder parameter in your filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p> 
        /// <p>For filter expressions that use an <code>INCLUDE</code> element to include items, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you can omit the <code>filter-values</code>.In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p> 
        /// <p>For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
        pub fn filter_values(mut self, k: impl Into<std::string::String>, v: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.filter_values(k.into(), v.into());
            self
        }
        /// <p>The values to use when filtering recommendations. For each placeholder parameter in your filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p> 
        /// <p>For filter expressions that use an <code>INCLUDE</code> element to include items, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you can omit the <code>filter-values</code>.In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p> 
        /// <p>For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
        pub fn set_filter_values(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>) -> Self {
            self.inner = self.inner.set_filter_values(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetRecommendations`.
                        ///
    /// <p>Returns a list of recommended items. For campaigns, the campaign's Amazon Resource Name (ARN) is required and the required user and item input depends on the recipe type used to create the solution backing the campaign as follows:</p> 
    /// <ul> 
    /// <li> <p>USER_PERSONALIZATION - <code>userId</code> required, <code>itemId</code> not used</p> </li> 
    /// <li> <p>RELATED_ITEMS - <code>itemId</code> required, <code>userId</code> not used</p> </li> 
    /// </ul> <note> 
    /// <p>Campaigns that are backed by a solution created using a recipe of type PERSONALIZED_RANKING use the API.</p> 
    /// </note> 
    /// <p> For recommenders, the recommender's ARN is required and the required item and user input depends on the use case (domain-based recipe) backing the recommender. For information on use case requirements see <a href="https://docs.aws.amazon.com/personalize/latest/dg/domain-use-cases.html">Choosing recommender use cases</a>. </p>
    #[derive(std::clone::Clone, std::fmt::Debug, )]
    pub struct GetRecommendations {
                            handle: std::sync::Arc<super::Handle>,
                            inner: crate::input::get_recommendations_input::Builder
                        }
    impl GetRecommendations  {
        /// Creates a new `GetRecommendations`.
                                pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
                                    Self { handle, inner: Default::default() }
                                }
        
                                /// Consume this builder, creating a customizable operation that can be modified before being
                                /// sent. The operation's inner [http::Request] can be modified as well.
                                pub async fn customize(self) -> std::result::Result<
                                    crate::operation::customize::CustomizableOperation<crate::operation::GetRecommendations, aws_http::retry::AwsResponseRetryClassifier,>,
                                    aws_smithy_http::result::SdkError<crate::error::GetRecommendationsError>
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
                                pub async fn send(self) -> std::result::Result<crate::output::GetRecommendationsOutput, aws_smithy_http::result::SdkError<crate::error::GetRecommendationsError>>
                                 {
                                    let op = self.inner.build().map_err(|err|aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?
                                        .make_operation(&self.handle.conf)
                                        .await
                                        .map_err(|err|aws_smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
                                    self.handle.client.call(op).await
                                }
        /// <p>The Amazon Resource Name (ARN) of the campaign to use for getting recommendations.</p>
        pub fn campaign_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.campaign_arn(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the campaign to use for getting recommendations.</p>
        pub fn set_campaign_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_campaign_arn(input);
            self
        }
        /// <p>The item ID to provide recommendations for.</p> 
        /// <p>Required for <code>RELATED_ITEMS</code> recipe type.</p>
        pub fn item_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.item_id(input.into());
            self
        }
        /// <p>The item ID to provide recommendations for.</p> 
        /// <p>Required for <code>RELATED_ITEMS</code> recipe type.</p>
        pub fn set_item_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_item_id(input);
            self
        }
        /// <p>The user ID to provide recommendations for.</p> 
        /// <p>Required for <code>USER_PERSONALIZATION</code> recipe type.</p>
        pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.user_id(input.into());
            self
        }
        /// <p>The user ID to provide recommendations for.</p> 
        /// <p>Required for <code>USER_PERSONALIZATION</code> recipe type.</p>
        pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_user_id(input);
            self
        }
        /// <p>The number of results to return. The default is 25. The maximum is 500.</p>
        pub fn num_results(mut self, input: i32) -> Self {
            self.inner = self.inner.num_results(input);
            self
        }
        /// <p>The number of results to return. The default is 25. The maximum is 500.</p>
        pub fn set_num_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_num_results(input);
            self
        }
        /// Adds a key-value pair to `context`.
        ///
        /// To override the contents of this collection use [`set_context`](Self::set_context).
        ///
        /// <p>The contextual metadata to use when getting recommendations. Contextual metadata includes any interaction information that might be relevant when getting a user's recommendations, such as the user's current location or device type.</p>
        pub fn context(mut self, k: impl Into<std::string::String>, v: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.context(k.into(), v.into());
            self
        }
        /// <p>The contextual metadata to use when getting recommendations. Contextual metadata includes any interaction information that might be relevant when getting a user's recommendations, such as the user's current location or device type.</p>
        pub fn set_context(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>) -> Self {
            self.inner = self.inner.set_context(input);
            self
        }
        /// <p>The ARN of the filter to apply to the returned recommendations. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p> 
        /// <p>When using this parameter, be sure the filter resource is <code>ACTIVE</code>.</p>
        pub fn filter_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.filter_arn(input.into());
            self
        }
        /// <p>The ARN of the filter to apply to the returned recommendations. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p> 
        /// <p>When using this parameter, be sure the filter resource is <code>ACTIVE</code>.</p>
        pub fn set_filter_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_filter_arn(input);
            self
        }
        /// Adds a key-value pair to `filterValues`.
        ///
        /// To override the contents of this collection use [`set_filter_values`](Self::set_filter_values).
        ///
        /// <p>The values to use when filtering recommendations. For each placeholder parameter in your filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p> 
        /// <p>For filter expressions that use an <code>INCLUDE</code> element to include items, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you can omit the <code>filter-values</code>.In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p> 
        /// <p>For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering recommendations and user segments</a>.</p>
        pub fn filter_values(mut self, k: impl Into<std::string::String>, v: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.filter_values(k.into(), v.into());
            self
        }
        /// <p>The values to use when filtering recommendations. For each placeholder parameter in your filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p> 
        /// <p>For filter expressions that use an <code>INCLUDE</code> element to include items, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude items, you can omit the <code>filter-values</code>.In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p> 
        /// <p>For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering recommendations and user segments</a>.</p>
        pub fn set_filter_values(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>) -> Self {
            self.inner = self.inner.set_filter_values(input);
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the recommender to use to get recommendations. Provide a recommender ARN if you created a Domain dataset group with a recommender for a domain use case.</p>
        pub fn recommender_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.recommender_arn(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the recommender to use to get recommendations. Provide a recommender ARN if you created a Domain dataset group with a recommender for a domain use case.</p>
        pub fn set_recommender_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_recommender_arn(input);
            self
        }
        /// Appends an item to `promotions`.
        ///
        /// To override the contents of this collection use [`set_promotions`](Self::set_promotions).
        ///
        /// <p>The promotions to apply to the recommendation request. A promotion defines additional business rules that apply to a configurable subset of recommended items.</p>
        pub fn promotions(mut self, input: crate::model::Promotion) -> Self {
            self.inner = self.inner.promotions(input);
            self
        }
        /// <p>The promotions to apply to the recommendation request. A promotion defines additional business rules that apply to a configurable subset of recommended items.</p>
        pub fn set_promotions(mut self, input: std::option::Option<std::vec::Vec<crate::model::Promotion>>) -> Self {
            self.inner = self.inner.set_promotions(input);
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

