// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`ListCidrBlocks`](crate::operation::ListCidrBlocks)
            pub struct ListCidrBlocksPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::list_cidr_blocks_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl ListCidrBlocksPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::list_cidr_blocks_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `max_results`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.max_results = Some(limit);
                    self
                }

                /// Create a flattened paginator
                    ///
                    /// This paginator automatically flattens results using `cidr_blocks`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::ListCidrBlocksPaginatorItems {
                        crate::paginator::ListCidrBlocksPaginatorItems(self)
                    }

                /// Stop paginating when the service returns the same pagination token twice in a row.
                ///
                /// Defaults to true.
                ///
                /// For certain operations, it may be useful to continue on duplicate token. For example,
                /// if an operation is for tailing a log file in real-time, then continuing may be desired.
                /// This option can be set to `false` to accommodate these use cases.
                pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
                    self.stop_on_duplicate_token = stop_on_duplicate_token;
                    self
                }

                /// Create the pagination stream
                ///
                /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::ListCidrBlocksOutput, aws_smithy_http::result::SdkError<crate::error::ListCidrBlocksError>>> + Unpin
                 {
                    // Move individual fields out of self for the borrow checker
                    let builder = self.builder;
                    let handle = self.handle;
                    aws_smithy_async::future::fn_stream::FnStream::new(move |tx| Box::pin(async move {
                        // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                        let mut input = match builder.build().map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into())) {
                            Ok(input) => input,
                            Err(e) => { let _ = tx.send(Err(e)).await; return; }
                        };
                        loop {
                            let op = match input.make_operation(&handle.conf)
                                .await
                                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into())) {
                                Ok(op) => op,
                                Err(e) => {
                                    let _ = tx.send(Err(e)).await;
                                    return;
                                }
                            };
                            let resp = handle.client.call(op).await;
                            // If the input member is None or it was an error
                            let done = match resp {
                                Ok(ref resp) => {
                                    let new_token = crate::lens::reflens_structure_crate_output_list_cidr_blocks_output_next_token(resp);
                                    let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                    if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
                                        true
                                    } else {
                                        input.next_token = new_token.cloned();
                                        is_empty
                                    }
                                },
                                Err(_) => true,
                            };
                            if tx.send(resp).await.is_err() {
                                // receiving end was dropped
                                return
                            }
                            if done {
                                return
                            }
                        }
                    }))
                }
            }

/// Paginator for [`ListCidrCollections`](crate::operation::ListCidrCollections)
            pub struct ListCidrCollectionsPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::list_cidr_collections_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl ListCidrCollectionsPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::list_cidr_collections_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `max_results`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.max_results = Some(limit);
                    self
                }

                /// Create a flattened paginator
                    ///
                    /// This paginator automatically flattens results using `cidr_collections`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::ListCidrCollectionsPaginatorItems {
                        crate::paginator::ListCidrCollectionsPaginatorItems(self)
                    }

                /// Stop paginating when the service returns the same pagination token twice in a row.
                ///
                /// Defaults to true.
                ///
                /// For certain operations, it may be useful to continue on duplicate token. For example,
                /// if an operation is for tailing a log file in real-time, then continuing may be desired.
                /// This option can be set to `false` to accommodate these use cases.
                pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
                    self.stop_on_duplicate_token = stop_on_duplicate_token;
                    self
                }

                /// Create the pagination stream
                ///
                /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::ListCidrCollectionsOutput, aws_smithy_http::result::SdkError<crate::error::ListCidrCollectionsError>>> + Unpin
                 {
                    // Move individual fields out of self for the borrow checker
                    let builder = self.builder;
                    let handle = self.handle;
                    aws_smithy_async::future::fn_stream::FnStream::new(move |tx| Box::pin(async move {
                        // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                        let mut input = match builder.build().map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into())) {
                            Ok(input) => input,
                            Err(e) => { let _ = tx.send(Err(e)).await; return; }
                        };
                        loop {
                            let op = match input.make_operation(&handle.conf)
                                .await
                                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into())) {
                                Ok(op) => op,
                                Err(e) => {
                                    let _ = tx.send(Err(e)).await;
                                    return;
                                }
                            };
                            let resp = handle.client.call(op).await;
                            // If the input member is None or it was an error
                            let done = match resp {
                                Ok(ref resp) => {
                                    let new_token = crate::lens::reflens_structure_crate_output_list_cidr_collections_output_next_token(resp);
                                    let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                    if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
                                        true
                                    } else {
                                        input.next_token = new_token.cloned();
                                        is_empty
                                    }
                                },
                                Err(_) => true,
                            };
                            if tx.send(resp).await.is_err() {
                                // receiving end was dropped
                                return
                            }
                            if done {
                                return
                            }
                        }
                    }))
                }
            }

/// Paginator for [`ListCidrLocations`](crate::operation::ListCidrLocations)
            pub struct ListCidrLocationsPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::list_cidr_locations_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl ListCidrLocationsPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::list_cidr_locations_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `max_results`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.max_results = Some(limit);
                    self
                }

                /// Create a flattened paginator
                    ///
                    /// This paginator automatically flattens results using `cidr_locations`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::ListCidrLocationsPaginatorItems {
                        crate::paginator::ListCidrLocationsPaginatorItems(self)
                    }

                /// Stop paginating when the service returns the same pagination token twice in a row.
                ///
                /// Defaults to true.
                ///
                /// For certain operations, it may be useful to continue on duplicate token. For example,
                /// if an operation is for tailing a log file in real-time, then continuing may be desired.
                /// This option can be set to `false` to accommodate these use cases.
                pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
                    self.stop_on_duplicate_token = stop_on_duplicate_token;
                    self
                }

                /// Create the pagination stream
                ///
                /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::ListCidrLocationsOutput, aws_smithy_http::result::SdkError<crate::error::ListCidrLocationsError>>> + Unpin
                 {
                    // Move individual fields out of self for the borrow checker
                    let builder = self.builder;
                    let handle = self.handle;
                    aws_smithy_async::future::fn_stream::FnStream::new(move |tx| Box::pin(async move {
                        // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                        let mut input = match builder.build().map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into())) {
                            Ok(input) => input,
                            Err(e) => { let _ = tx.send(Err(e)).await; return; }
                        };
                        loop {
                            let op = match input.make_operation(&handle.conf)
                                .await
                                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into())) {
                                Ok(op) => op,
                                Err(e) => {
                                    let _ = tx.send(Err(e)).await;
                                    return;
                                }
                            };
                            let resp = handle.client.call(op).await;
                            // If the input member is None or it was an error
                            let done = match resp {
                                Ok(ref resp) => {
                                    let new_token = crate::lens::reflens_structure_crate_output_list_cidr_locations_output_next_token(resp);
                                    let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                    if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
                                        true
                                    } else {
                                        input.next_token = new_token.cloned();
                                        is_empty
                                    }
                                },
                                Err(_) => true,
                            };
                            if tx.send(resp).await.is_err() {
                                // receiving end was dropped
                                return
                            }
                            if done {
                                return
                            }
                        }
                    }))
                }
            }

/// Paginator for [`ListHealthChecks`](crate::operation::ListHealthChecks)
            pub struct ListHealthChecksPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::list_health_checks_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl ListHealthChecksPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::list_health_checks_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `max_items`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.max_items = Some(limit);
                    self
                }

                /// Create a flattened paginator
                    ///
                    /// This paginator automatically flattens results using `health_checks`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::ListHealthChecksPaginatorItems {
                        crate::paginator::ListHealthChecksPaginatorItems(self)
                    }

                /// Stop paginating when the service returns the same pagination token twice in a row.
                ///
                /// Defaults to true.
                ///
                /// For certain operations, it may be useful to continue on duplicate token. For example,
                /// if an operation is for tailing a log file in real-time, then continuing may be desired.
                /// This option can be set to `false` to accommodate these use cases.
                pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
                    self.stop_on_duplicate_token = stop_on_duplicate_token;
                    self
                }

                /// Create the pagination stream
                ///
                /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::ListHealthChecksOutput, aws_smithy_http::result::SdkError<crate::error::ListHealthChecksError>>> + Unpin
                 {
                    // Move individual fields out of self for the borrow checker
                    let builder = self.builder;
                    let handle = self.handle;
                    aws_smithy_async::future::fn_stream::FnStream::new(move |tx| Box::pin(async move {
                        // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                        let mut input = match builder.build().map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into())) {
                            Ok(input) => input,
                            Err(e) => { let _ = tx.send(Err(e)).await; return; }
                        };
                        loop {
                            let op = match input.make_operation(&handle.conf)
                                .await
                                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into())) {
                                Ok(op) => op,
                                Err(e) => {
                                    let _ = tx.send(Err(e)).await;
                                    return;
                                }
                            };
                            let resp = handle.client.call(op).await;
                            // If the input member is None or it was an error
                            let done = match resp {
                                Ok(ref resp) => {
                                    let new_token = crate::lens::reflens_structure_crate_output_list_health_checks_output_next_marker(resp);
                                    let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                    if !is_empty && new_token == input.marker.as_ref() && self.stop_on_duplicate_token {
                                        true
                                    } else {
                                        input.marker = new_token.cloned();
                                        is_empty
                                    }
                                },
                                Err(_) => true,
                            };
                            if tx.send(resp).await.is_err() {
                                // receiving end was dropped
                                return
                            }
                            if done {
                                return
                            }
                        }
                    }))
                }
            }

/// Paginator for [`ListHostedZones`](crate::operation::ListHostedZones)
            pub struct ListHostedZonesPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::list_hosted_zones_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl ListHostedZonesPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::list_hosted_zones_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `max_items`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.max_items = Some(limit);
                    self
                }

                /// Create a flattened paginator
                    ///
                    /// This paginator automatically flattens results using `hosted_zones`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::ListHostedZonesPaginatorItems {
                        crate::paginator::ListHostedZonesPaginatorItems(self)
                    }

                /// Stop paginating when the service returns the same pagination token twice in a row.
                ///
                /// Defaults to true.
                ///
                /// For certain operations, it may be useful to continue on duplicate token. For example,
                /// if an operation is for tailing a log file in real-time, then continuing may be desired.
                /// This option can be set to `false` to accommodate these use cases.
                pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
                    self.stop_on_duplicate_token = stop_on_duplicate_token;
                    self
                }

                /// Create the pagination stream
                ///
                /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::ListHostedZonesOutput, aws_smithy_http::result::SdkError<crate::error::ListHostedZonesError>>> + Unpin
                 {
                    // Move individual fields out of self for the borrow checker
                    let builder = self.builder;
                    let handle = self.handle;
                    aws_smithy_async::future::fn_stream::FnStream::new(move |tx| Box::pin(async move {
                        // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                        let mut input = match builder.build().map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into())) {
                            Ok(input) => input,
                            Err(e) => { let _ = tx.send(Err(e)).await; return; }
                        };
                        loop {
                            let op = match input.make_operation(&handle.conf)
                                .await
                                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into())) {
                                Ok(op) => op,
                                Err(e) => {
                                    let _ = tx.send(Err(e)).await;
                                    return;
                                }
                            };
                            let resp = handle.client.call(op).await;
                            // If the input member is None or it was an error
                            let done = match resp {
                                Ok(ref resp) => {
                                    let new_token = crate::lens::reflens_structure_crate_output_list_hosted_zones_output_next_marker(resp);
                                    let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                    if !is_empty && new_token == input.marker.as_ref() && self.stop_on_duplicate_token {
                                        true
                                    } else {
                                        input.marker = new_token.cloned();
                                        is_empty
                                    }
                                },
                                Err(_) => true,
                            };
                            if tx.send(resp).await.is_err() {
                                // receiving end was dropped
                                return
                            }
                            if done {
                                return
                            }
                        }
                    }))
                }
            }

/// Paginator for [`ListQueryLoggingConfigs`](crate::operation::ListQueryLoggingConfigs)
            pub struct ListQueryLoggingConfigsPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::list_query_logging_configs_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl ListQueryLoggingConfigsPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::list_query_logging_configs_input::Builder) -> Self {
                    Self {
                        handle,
                        builder,
                        stop_on_duplicate_token: true,
                    }
                }

                /// Set the page size
                ///
                /// _Note: this method will override any previously set value for `max_results`_
                pub fn page_size(mut self, limit: i32) -> Self {
                    self.builder.max_results = Some(limit);
                    self
                }

                /// Create a flattened paginator
                    ///
                    /// This paginator automatically flattens results using `query_logging_configs`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::ListQueryLoggingConfigsPaginatorItems {
                        crate::paginator::ListQueryLoggingConfigsPaginatorItems(self)
                    }

                /// Stop paginating when the service returns the same pagination token twice in a row.
                ///
                /// Defaults to true.
                ///
                /// For certain operations, it may be useful to continue on duplicate token. For example,
                /// if an operation is for tailing a log file in real-time, then continuing may be desired.
                /// This option can be set to `false` to accommodate these use cases.
                pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
                    self.stop_on_duplicate_token = stop_on_duplicate_token;
                    self
                }

                /// Create the pagination stream
                ///
                /// _Note:_ No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next)).
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::ListQueryLoggingConfigsOutput, aws_smithy_http::result::SdkError<crate::error::ListQueryLoggingConfigsError>>> + Unpin
                 {
                    // Move individual fields out of self for the borrow checker
                    let builder = self.builder;
                    let handle = self.handle;
                    aws_smithy_async::future::fn_stream::FnStream::new(move |tx| Box::pin(async move {
                        // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                        let mut input = match builder.build().map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into())) {
                            Ok(input) => input,
                            Err(e) => { let _ = tx.send(Err(e)).await; return; }
                        };
                        loop {
                            let op = match input.make_operation(&handle.conf)
                                .await
                                .map_err(|err| aws_smithy_http::result::SdkError::ConstructionFailure(err.into())) {
                                Ok(op) => op,
                                Err(e) => {
                                    let _ = tx.send(Err(e)).await;
                                    return;
                                }
                            };
                            let resp = handle.client.call(op).await;
                            // If the input member is None or it was an error
                            let done = match resp {
                                Ok(ref resp) => {
                                    let new_token = crate::lens::reflens_structure_crate_output_list_query_logging_configs_output_next_token(resp);
                                    let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                    if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
                                        true
                                    } else {
                                        input.next_token = new_token.cloned();
                                        is_empty
                                    }
                                },
                                Err(_) => true,
                            };
                            if tx.send(resp).await.is_err() {
                                // receiving end was dropped
                                return
                            }
                            if done {
                                return
                            }
                        }
                    }))
                }
            }

/// Flattened paginator for `ListCidrBlocksPaginator`
                ///
                /// This is created with [`.items()`](ListCidrBlocksPaginator::items)
                pub struct ListCidrBlocksPaginatorItems(ListCidrBlocksPaginator);

                impl  ListCidrBlocksPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::CidrBlockSummary, aws_smithy_http::result::SdkError<crate::error::ListCidrBlocksError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_list_cidr_blocks_output_cidr_blocks(page).unwrap_or_default().into_iter())
                    }
                }

/// Flattened paginator for `ListCidrCollectionsPaginator`
                ///
                /// This is created with [`.items()`](ListCidrCollectionsPaginator::items)
                pub struct ListCidrCollectionsPaginatorItems(ListCidrCollectionsPaginator);

                impl  ListCidrCollectionsPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::CollectionSummary, aws_smithy_http::result::SdkError<crate::error::ListCidrCollectionsError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_list_cidr_collections_output_cidr_collections(page).unwrap_or_default().into_iter())
                    }
                }

/// Flattened paginator for `ListCidrLocationsPaginator`
                ///
                /// This is created with [`.items()`](ListCidrLocationsPaginator::items)
                pub struct ListCidrLocationsPaginatorItems(ListCidrLocationsPaginator);

                impl  ListCidrLocationsPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::LocationSummary, aws_smithy_http::result::SdkError<crate::error::ListCidrLocationsError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_list_cidr_locations_output_cidr_locations(page).unwrap_or_default().into_iter())
                    }
                }

/// Flattened paginator for `ListHealthChecksPaginator`
                ///
                /// This is created with [`.items()`](ListHealthChecksPaginator::items)
                pub struct ListHealthChecksPaginatorItems(ListHealthChecksPaginator);

                impl  ListHealthChecksPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::HealthCheck, aws_smithy_http::result::SdkError<crate::error::ListHealthChecksError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_list_health_checks_output_health_checks(page).unwrap_or_default().into_iter())
                    }
                }

/// Flattened paginator for `ListHostedZonesPaginator`
                ///
                /// This is created with [`.items()`](ListHostedZonesPaginator::items)
                pub struct ListHostedZonesPaginatorItems(ListHostedZonesPaginator);

                impl  ListHostedZonesPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::HostedZone, aws_smithy_http::result::SdkError<crate::error::ListHostedZonesError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_list_hosted_zones_output_hosted_zones(page).unwrap_or_default().into_iter())
                    }
                }

/// Flattened paginator for `ListQueryLoggingConfigsPaginator`
                ///
                /// This is created with [`.items()`](ListQueryLoggingConfigsPaginator::items)
                pub struct ListQueryLoggingConfigsPaginatorItems(ListQueryLoggingConfigsPaginator);

                impl  ListQueryLoggingConfigsPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::QueryLoggingConfig, aws_smithy_http::result::SdkError<crate::error::ListQueryLoggingConfigsError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_list_query_logging_configs_output_query_logging_configs(page).unwrap_or_default().into_iter())
                    }
                }

