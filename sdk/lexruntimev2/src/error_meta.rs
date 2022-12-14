// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum Error {
    /// <p></p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p></p>
    BadGatewayException(crate::error::BadGatewayException),
    /// <p></p>
    ConflictException(crate::error::ConflictException),
    /// <p></p>
    DependencyFailedException(crate::error::DependencyFailedException),
    /// <p></p>
    InternalServerException(crate::error::InternalServerException),
    /// <p></p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p></p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p></p>
    ValidationException(crate::error::ValidationException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>)
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::BadGatewayException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::DependencyFailedException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteSessionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteSessionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteSessionErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DeleteSessionErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DeleteSessionErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::DeleteSessionErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DeleteSessionErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::DeleteSessionErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::DeleteSessionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSessionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetSessionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetSessionErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetSessionErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetSessionErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetSessionErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetSessionErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetSessionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutSessionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutSessionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::PutSessionErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::PutSessionErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::PutSessionErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::PutSessionErrorKind::BadGatewayException(inner) => Error::BadGatewayException(inner),
                crate::error::PutSessionErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::PutSessionErrorKind::DependencyFailedException(inner) => Error::DependencyFailedException(inner),
                crate::error::PutSessionErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::PutSessionErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::PutSessionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RecognizeTextError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::RecognizeTextError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::RecognizeTextErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::RecognizeTextErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::RecognizeTextErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::RecognizeTextErrorKind::BadGatewayException(inner) => Error::BadGatewayException(inner),
                crate::error::RecognizeTextErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::RecognizeTextErrorKind::DependencyFailedException(inner) => Error::DependencyFailedException(inner),
                crate::error::RecognizeTextErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::RecognizeTextErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::RecognizeTextErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RecognizeUtteranceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::RecognizeUtteranceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::RecognizeUtteranceErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::RecognizeUtteranceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::RecognizeUtteranceErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::RecognizeUtteranceErrorKind::BadGatewayException(inner) => Error::BadGatewayException(inner),
                crate::error::RecognizeUtteranceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::RecognizeUtteranceErrorKind::DependencyFailedException(inner) => Error::DependencyFailedException(inner),
                crate::error::RecognizeUtteranceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::RecognizeUtteranceErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::RecognizeUtteranceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}

