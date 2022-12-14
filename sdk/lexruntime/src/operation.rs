// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DeleteSession`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_session`](crate::client::Client::delete_session).
            ///
            /// See [`crate::client::fluent_builders::DeleteSession`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct DeleteSession {
    _private: ()
}
impl DeleteSession {
    /// Creates a new builder-style object to manufacture [`DeleteSessionInput`](crate::input::DeleteSessionInput).
    pub fn builder() -> crate::input::delete_session_input::Builder {
        crate::input::delete_session_input::Builder::default()
    }
    /// Creates a new `DeleteSession` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteSession {
                type Output = std::result::Result<crate::output::DeleteSessionOutput, crate::error::DeleteSessionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_session_error(response)
                     } else {
                        crate::operation_deser::parse_delete_session_response(response)
                     }
                }
            }

/// Operation shape for `GetSession`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_session`](crate::client::Client::get_session).
            ///
            /// See [`crate::client::fluent_builders::GetSession`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct GetSession {
    _private: ()
}
impl GetSession {
    /// Creates a new builder-style object to manufacture [`GetSessionInput`](crate::input::GetSessionInput).
    pub fn builder() -> crate::input::get_session_input::Builder {
        crate::input::get_session_input::Builder::default()
    }
    /// Creates a new `GetSession` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetSession {
                type Output = std::result::Result<crate::output::GetSessionOutput, crate::error::GetSessionError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_session_error(response)
                     } else {
                        crate::operation_deser::parse_get_session_response(response)
                     }
                }
            }

/// Operation shape for `PostContent`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`post_content`](crate::client::Client::post_content).
            ///
            /// See [`crate::client::fluent_builders::PostContent`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct PostContent {
    _private: ()
}
impl PostContent {
    /// Creates a new builder-style object to manufacture [`PostContentInput`](crate::input::PostContentInput).
    pub fn builder() -> crate::input::post_content_input::Builder {
        crate::input::post_content_input::Builder::default()
    }
    /// Creates a new `PostContent` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseHttpResponse for PostContent {
                type Output = std::result::Result<crate::output::PostContentOutput, crate::error::PostContentError>;
                fn parse_unloaded(&self, response: &mut aws_smithy_http::operation::Response) -> Option<Self::Output> {
                    // This is an error, defer to the non-streaming parser
                    if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
                        return None;
                    }
                    Some(crate::operation_deser::parse_post_content(response))
                }
                fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                    // if streaming, we only hit this case if its an error
                    crate::operation_deser::parse_post_content_error(response)
                }
            }

/// Operation shape for `PostText`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`post_text`](crate::client::Client::post_text).
            ///
            /// See [`crate::client::fluent_builders::PostText`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct PostText {
    _private: ()
}
impl PostText {
    /// Creates a new builder-style object to manufacture [`PostTextInput`](crate::input::PostTextInput).
    pub fn builder() -> crate::input::post_text_input::Builder {
        crate::input::post_text_input::Builder::default()
    }
    /// Creates a new `PostText` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PostText {
                type Output = std::result::Result<crate::output::PostTextOutput, crate::error::PostTextError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_post_text_error(response)
                     } else {
                        crate::operation_deser::parse_post_text_response(response)
                     }
                }
            }

/// Operation shape for `PutSession`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_session`](crate::client::Client::put_session).
            ///
            /// See [`crate::client::fluent_builders::PutSession`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct PutSession {
    _private: ()
}
impl PutSession {
    /// Creates a new builder-style object to manufacture [`PutSessionInput`](crate::input::PutSessionInput).
    pub fn builder() -> crate::input::put_session_input::Builder {
        crate::input::put_session_input::Builder::default()
    }
    /// Creates a new `PutSession` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseHttpResponse for PutSession {
                type Output = std::result::Result<crate::output::PutSessionOutput, crate::error::PutSessionError>;
                fn parse_unloaded(&self, response: &mut aws_smithy_http::operation::Response) -> Option<Self::Output> {
                    // This is an error, defer to the non-streaming parser
                    if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
                        return None;
                    }
                    Some(crate::operation_deser::parse_put_session(response))
                }
                fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                    // if streaming, we only hit this case if its an error
                    crate::operation_deser::parse_put_session_error(response)
                }
            }

/// Operation customization and supporting types
pub mod customize;

