// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`DescribeEndpoints`](crate::operation::DescribeEndpoints)
            pub struct DescribeEndpointsPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::describe_endpoints_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl DescribeEndpointsPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::describe_endpoints_input::Builder) -> Self {
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
                    /// This paginator automatically flattens results using `endpoints`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::DescribeEndpointsPaginatorItems {
                        crate::paginator::DescribeEndpointsPaginatorItems(self)
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
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::DescribeEndpointsOutput, aws_smithy_http::result::SdkError<crate::error::DescribeEndpointsError>>> + Unpin
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
                                    let new_token = crate::lens::reflens_structure_crate_output_describe_endpoints_output_next_token(resp);
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

/// Paginator for [`ListJobs`](crate::operation::ListJobs)
            pub struct ListJobsPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::list_jobs_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl ListJobsPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::list_jobs_input::Builder) -> Self {
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
                    /// This paginator automatically flattens results using `jobs`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::ListJobsPaginatorItems {
                        crate::paginator::ListJobsPaginatorItems(self)
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
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::ListJobsOutput, aws_smithy_http::result::SdkError<crate::error::ListJobsError>>> + Unpin
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
                                    let new_token = crate::lens::reflens_structure_crate_output_list_jobs_output_next_token(resp);
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

/// Paginator for [`ListJobTemplates`](crate::operation::ListJobTemplates)
            pub struct ListJobTemplatesPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::list_job_templates_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl ListJobTemplatesPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::list_job_templates_input::Builder) -> Self {
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
                    /// This paginator automatically flattens results using `job_templates`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::ListJobTemplatesPaginatorItems {
                        crate::paginator::ListJobTemplatesPaginatorItems(self)
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
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::ListJobTemplatesOutput, aws_smithy_http::result::SdkError<crate::error::ListJobTemplatesError>>> + Unpin
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
                                    let new_token = crate::lens::reflens_structure_crate_output_list_job_templates_output_next_token(resp);
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

/// Paginator for [`ListPresets`](crate::operation::ListPresets)
            pub struct ListPresetsPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::list_presets_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl ListPresetsPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::list_presets_input::Builder) -> Self {
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
                    /// This paginator automatically flattens results using `presets`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::ListPresetsPaginatorItems {
                        crate::paginator::ListPresetsPaginatorItems(self)
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
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::ListPresetsOutput, aws_smithy_http::result::SdkError<crate::error::ListPresetsError>>> + Unpin
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
                                    let new_token = crate::lens::reflens_structure_crate_output_list_presets_output_next_token(resp);
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

/// Paginator for [`ListQueues`](crate::operation::ListQueues)
            pub struct ListQueuesPaginator {
                handle: std::sync::Arc<crate::client::Handle>,
                builder: crate::input::list_queues_input::Builder,
                stop_on_duplicate_token: bool,
            }

            impl ListQueuesPaginator  {
                /// Create a new paginator-wrapper
                pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>, builder: crate::input::list_queues_input::Builder) -> Self {
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
                    /// This paginator automatically flattens results using `queues`. Queries to the underlying service
                    /// are dispatched lazily.
                    pub fn items(self) -> crate::paginator::ListQueuesPaginatorItems {
                        crate::paginator::ListQueuesPaginatorItems(self)
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
                pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::output::ListQueuesOutput, aws_smithy_http::result::SdkError<crate::error::ListQueuesError>>> + Unpin
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
                                    let new_token = crate::lens::reflens_structure_crate_output_list_queues_output_next_token(resp);
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

/// Flattened paginator for `DescribeEndpointsPaginator`
                ///
                /// This is created with [`.items()`](DescribeEndpointsPaginator::items)
                pub struct DescribeEndpointsPaginatorItems(DescribeEndpointsPaginator);

                impl  DescribeEndpointsPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::Endpoint, aws_smithy_http::result::SdkError<crate::error::DescribeEndpointsError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_describe_endpoints_output_endpoints(page).unwrap_or_default().into_iter())
                    }
                }

/// Flattened paginator for `ListJobsPaginator`
                ///
                /// This is created with [`.items()`](ListJobsPaginator::items)
                pub struct ListJobsPaginatorItems(ListJobsPaginator);

                impl  ListJobsPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::Job, aws_smithy_http::result::SdkError<crate::error::ListJobsError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_list_jobs_output_jobs(page).unwrap_or_default().into_iter())
                    }
                }

/// Flattened paginator for `ListJobTemplatesPaginator`
                ///
                /// This is created with [`.items()`](ListJobTemplatesPaginator::items)
                pub struct ListJobTemplatesPaginatorItems(ListJobTemplatesPaginator);

                impl  ListJobTemplatesPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::JobTemplate, aws_smithy_http::result::SdkError<crate::error::ListJobTemplatesError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_list_job_templates_output_job_templates(page).unwrap_or_default().into_iter())
                    }
                }

/// Flattened paginator for `ListPresetsPaginator`
                ///
                /// This is created with [`.items()`](ListPresetsPaginator::items)
                pub struct ListPresetsPaginatorItems(ListPresetsPaginator);

                impl  ListPresetsPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::Preset, aws_smithy_http::result::SdkError<crate::error::ListPresetsError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_list_presets_output_presets(page).unwrap_or_default().into_iter())
                    }
                }

/// Flattened paginator for `ListQueuesPaginator`
                ///
                /// This is created with [`.items()`](ListQueuesPaginator::items)
                pub struct ListQueuesPaginatorItems(ListQueuesPaginator);

                impl  ListQueuesPaginatorItems  {
                    /// Create the pagination stream
                    ///
                    /// _Note: No requests will be dispatched until the stream is used (eg. with [`.next().await`](tokio_stream::StreamExt::next))._
                    ///
                    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](tokio_stream::StreamExt::collect).
                    pub fn send(self) -> impl tokio_stream::Stream<Item = std::result::Result<crate::model::Queue, aws_smithy_http::result::SdkError<crate::error::ListQueuesError>>> + Unpin
                     {
                        aws_smithy_async::future::fn_stream::TryFlatMap::new(self.0.send()).flat_map(|page| crate::lens::lens_structure_crate_output_list_queues_output_queues(page).unwrap_or_default().into_iter())
                    }
                }

