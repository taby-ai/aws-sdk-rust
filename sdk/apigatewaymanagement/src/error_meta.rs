// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum Error {
    /// <p>The caller is not authorized to invoke this operation.</p>
    ForbiddenException(crate::error::ForbiddenException),
    /// <p>The connection with the provided id no longer exists.</p>
    GoneException(crate::error::GoneException),
    /// <p>The client is sending more than the allowed number of requests per unit of time or the WebSocket client side buffer is full.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>The data has exceeded the maximum size allowed.</p>
    PayloadTooLargeException(crate::error::PayloadTooLargeException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>)
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ForbiddenException(inner) => inner.fmt(f),
            Error::GoneException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::PayloadTooLargeException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteConnectionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteConnectionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteConnectionErrorKind::ForbiddenException(inner) => Error::ForbiddenException(inner),
                crate::error::DeleteConnectionErrorKind::GoneException(inner) => Error::GoneException(inner),
                crate::error::DeleteConnectionErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::DeleteConnectionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetConnectionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetConnectionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetConnectionErrorKind::ForbiddenException(inner) => Error::ForbiddenException(inner),
                crate::error::GetConnectionErrorKind::GoneException(inner) => Error::GoneException(inner),
                crate::error::GetConnectionErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::GetConnectionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PostToConnectionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PostToConnectionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::PostToConnectionErrorKind::ForbiddenException(inner) => Error::ForbiddenException(inner),
                crate::error::PostToConnectionErrorKind::GoneException(inner) => Error::GoneException(inner),
                crate::error::PostToConnectionErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
                crate::error::PostToConnectionErrorKind::PayloadTooLargeException(inner) => Error::PayloadTooLargeException(inner),
                crate::error::PostToConnectionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}

