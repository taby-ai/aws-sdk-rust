// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

/// See [`SendCommandInput`](crate::input::SendCommandInput).
pub mod send_command_input {
    
    /// A builder for [`SendCommandInput`](crate::input::SendCommandInput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) session_token: std::option::Option<std::string::String>,
        pub(crate) start_session: std::option::Option<crate::model::StartSessionRequest>,
        pub(crate) start_transaction: std::option::Option<crate::model::StartTransactionRequest>,
        pub(crate) end_session: std::option::Option<crate::model::EndSessionRequest>,
        pub(crate) commit_transaction: std::option::Option<crate::model::CommitTransactionRequest>,
        pub(crate) abort_transaction: std::option::Option<crate::model::AbortTransactionRequest>,
        pub(crate) execute_statement: std::option::Option<crate::model::ExecuteStatementRequest>,
        pub(crate) fetch_page: std::option::Option<crate::model::FetchPageRequest>,
    }
    impl Builder {
        /// <p>Specifies the session token for the current command. A session token is constant throughout the life of the session.</p> 
        /// <p>To obtain a session token, run the <code>StartSession</code> command. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
        pub fn session_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.session_token = Some(input.into());
            self
        }
        /// <p>Specifies the session token for the current command. A session token is constant throughout the life of the session.</p> 
        /// <p>To obtain a session token, run the <code>StartSession</code> command. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
        pub fn set_session_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.session_token = input; self
        }
        /// <p>Command to start a new session. A session token is obtained as part of the response.</p>
        pub fn start_session(mut self, input: crate::model::StartSessionRequest) -> Self {
            self.start_session = Some(input);
            self
        }
        /// <p>Command to start a new session. A session token is obtained as part of the response.</p>
        pub fn set_start_session(mut self, input: std::option::Option<crate::model::StartSessionRequest>) -> Self {
            self.start_session = input; self
        }
        /// <p>Command to start a new transaction.</p>
        pub fn start_transaction(mut self, input: crate::model::StartTransactionRequest) -> Self {
            self.start_transaction = Some(input);
            self
        }
        /// <p>Command to start a new transaction.</p>
        pub fn set_start_transaction(mut self, input: std::option::Option<crate::model::StartTransactionRequest>) -> Self {
            self.start_transaction = input; self
        }
        /// <p>Command to end the current session.</p>
        pub fn end_session(mut self, input: crate::model::EndSessionRequest) -> Self {
            self.end_session = Some(input);
            self
        }
        /// <p>Command to end the current session.</p>
        pub fn set_end_session(mut self, input: std::option::Option<crate::model::EndSessionRequest>) -> Self {
            self.end_session = input; self
        }
        /// <p>Command to commit the specified transaction.</p>
        pub fn commit_transaction(mut self, input: crate::model::CommitTransactionRequest) -> Self {
            self.commit_transaction = Some(input);
            self
        }
        /// <p>Command to commit the specified transaction.</p>
        pub fn set_commit_transaction(mut self, input: std::option::Option<crate::model::CommitTransactionRequest>) -> Self {
            self.commit_transaction = input; self
        }
        /// <p>Command to abort the current transaction.</p>
        pub fn abort_transaction(mut self, input: crate::model::AbortTransactionRequest) -> Self {
            self.abort_transaction = Some(input);
            self
        }
        /// <p>Command to abort the current transaction.</p>
        pub fn set_abort_transaction(mut self, input: std::option::Option<crate::model::AbortTransactionRequest>) -> Self {
            self.abort_transaction = input; self
        }
        /// <p>Command to execute a statement in the specified transaction.</p>
        pub fn execute_statement(mut self, input: crate::model::ExecuteStatementRequest) -> Self {
            self.execute_statement = Some(input);
            self
        }
        /// <p>Command to execute a statement in the specified transaction.</p>
        pub fn set_execute_statement(mut self, input: std::option::Option<crate::model::ExecuteStatementRequest>) -> Self {
            self.execute_statement = input; self
        }
        /// <p>Command to fetch a page.</p>
        pub fn fetch_page(mut self, input: crate::model::FetchPageRequest) -> Self {
            self.fetch_page = Some(input);
            self
        }
        /// <p>Command to fetch a page.</p>
        pub fn set_fetch_page(mut self, input: std::option::Option<crate::model::FetchPageRequest>) -> Self {
            self.fetch_page = input; self
        }
        /// Consumes the builder and constructs a [`SendCommandInput`](crate::input::SendCommandInput).
        pub fn build(self) -> Result<crate::input::SendCommandInput, aws_smithy_http::operation::BuildError> {
            Ok(
                crate::input::SendCommandInput {
                    session_token: self.session_token
                    ,
                    start_session: self.start_session
                    ,
                    start_transaction: self.start_transaction
                    ,
                    end_session: self.end_session
                    ,
                    commit_transaction: self.commit_transaction
                    ,
                    abort_transaction: self.abort_transaction
                    ,
                    execute_statement: self.execute_statement
                    ,
                    fetch_page: self.fetch_page
                    ,
                }
            )
        }
    }
    
    
}
impl SendCommandInput {
    /// Consumes the builder and constructs an Operation<[`SendCommand`](crate::operation::SendCommand)>
    #[allow(unused_mut)]#[allow(clippy::let_and_return)]#[allow(clippy::needless_borrow)]pub async fn make_operation(&self, _config: &crate::config::Config) -> std::result::Result<aws_smithy_http::operation::Operation<crate::operation::SendCommand, aws_http::retry::AwsResponseRetryClassifier>, aws_smithy_http::operation::BuildError> {
        let mut request = {
            fn uri_base(_input: &crate::input::SendCommandInput, output: &mut String) -> Result<(), aws_smithy_http::operation::BuildError> {
                write!(output, "/").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]fn update_http_builder(
                            input: &crate::input::SendCommandInput,
                            builder: http::request::Builder
                        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError> {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(builder, http::header::CONTENT_TYPE, "application/x-amz-json-1.0");
            builder = aws_smithy_http::header::set_request_header_if_absent(
                                builder,
                                http::header::HeaderName::from_static("x-amz-target"),
                                "QLDBSession.SendCommand"
                            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_send_command(&self)?
        );
        if let Some(content_length) = body.content_length() {
                                request = aws_smithy_http::header::set_request_header_if_absent(request, http::header::CONTENT_LENGTH, content_length);
                            }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
                            aws_types::os_shim_internal::Env::real(),
                            crate::API_METADATA.clone(),
                        );
                        if let Some(app_name) = _config.app_name() {
                            user_agent = user_agent.with_app_name(app_name.clone());
                        }
                        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
                            request.properties_mut().insert(aws_types::SigningService::from_static(_config.signing_service()));
                            if let Some(region) = &_config.region {
                                request.properties_mut().insert(aws_types::region::SigningRegion::from(region.clone()));
                            }
        let endpoint_params = aws_endpoint::Params::new(_config.region.clone());
                            request.properties_mut()
                                .insert::<aws_smithy_http::endpoint::Result>(_config
                                    .endpoint_resolver
                                    .resolve_endpoint(&endpoint_params));
        if let Some(region) = &_config.region {
                                request.properties_mut().insert(region.clone());
                            }
        aws_http::auth::set_provider(&mut request.properties_mut(), _config.credentials_provider.clone());
        let op = aws_smithy_http::operation::Operation::new(request, crate::operation::SendCommand::new())
                            .with_metadata(aws_smithy_http::operation::Metadata::new("SendCommand", "qldbsession"));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`SendCommandInput`](crate::input::SendCommandInput).
    pub fn builder() -> crate::input::send_command_input::Builder {
        crate::input::send_command_input::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct SendCommandInput  {
    /// <p>Specifies the session token for the current command. A session token is constant throughout the life of the session.</p> 
    /// <p>To obtain a session token, run the <code>StartSession</code> command. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
    #[doc(hidden)]pub session_token: std::option::Option<std::string::String>,
    /// <p>Command to start a new session. A session token is obtained as part of the response.</p>
    #[doc(hidden)]pub start_session: std::option::Option<crate::model::StartSessionRequest>,
    /// <p>Command to start a new transaction.</p>
    #[doc(hidden)]pub start_transaction: std::option::Option<crate::model::StartTransactionRequest>,
    /// <p>Command to end the current session.</p>
    #[doc(hidden)]pub end_session: std::option::Option<crate::model::EndSessionRequest>,
    /// <p>Command to commit the specified transaction.</p>
    #[doc(hidden)]pub commit_transaction: std::option::Option<crate::model::CommitTransactionRequest>,
    /// <p>Command to abort the current transaction.</p>
    #[doc(hidden)]pub abort_transaction: std::option::Option<crate::model::AbortTransactionRequest>,
    /// <p>Command to execute a statement in the specified transaction.</p>
    #[doc(hidden)]pub execute_statement: std::option::Option<crate::model::ExecuteStatementRequest>,
    /// <p>Command to fetch a page.</p>
    #[doc(hidden)]pub fetch_page: std::option::Option<crate::model::FetchPageRequest>,
}
impl SendCommandInput {
    /// <p>Specifies the session token for the current command. A session token is constant throughout the life of the session.</p> 
    /// <p>To obtain a session token, run the <code>StartSession</code> command. This <code>SessionToken</code> is required for every subsequent command that is issued during the current session.</p>
    pub fn session_token(&self) -> std::option::Option<& str> {
        self.session_token.as_deref()
    }
    /// <p>Command to start a new session. A session token is obtained as part of the response.</p>
    pub fn start_session(&self) -> std::option::Option<& crate::model::StartSessionRequest> {
        self.start_session.as_ref()
    }
    /// <p>Command to start a new transaction.</p>
    pub fn start_transaction(&self) -> std::option::Option<& crate::model::StartTransactionRequest> {
        self.start_transaction.as_ref()
    }
    /// <p>Command to end the current session.</p>
    pub fn end_session(&self) -> std::option::Option<& crate::model::EndSessionRequest> {
        self.end_session.as_ref()
    }
    /// <p>Command to commit the specified transaction.</p>
    pub fn commit_transaction(&self) -> std::option::Option<& crate::model::CommitTransactionRequest> {
        self.commit_transaction.as_ref()
    }
    /// <p>Command to abort the current transaction.</p>
    pub fn abort_transaction(&self) -> std::option::Option<& crate::model::AbortTransactionRequest> {
        self.abort_transaction.as_ref()
    }
    /// <p>Command to execute a statement in the specified transaction.</p>
    pub fn execute_statement(&self) -> std::option::Option<& crate::model::ExecuteStatementRequest> {
        self.execute_statement.as_ref()
    }
    /// <p>Command to fetch a page.</p>
    pub fn fetch_page(&self) -> std::option::Option<& crate::model::FetchPageRequest> {
        self.fetch_page.as_ref()
    }
}
impl  std::fmt::Debug for SendCommandInput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SendCommandInput");
        formatter.field("session_token", &self.session_token);
        formatter.field("start_session", &self.start_session);
        formatter.field("start_transaction", &self.start_transaction);
        formatter.field("end_session", &self.end_session);
        formatter.field("commit_transaction", &self.commit_transaction);
        formatter.field("abort_transaction", &self.abort_transaction);
        formatter.field("execute_statement", &self.execute_statement);
        formatter.field("fetch_page", &self.fetch_page);
        formatter.finish()
    }
}

