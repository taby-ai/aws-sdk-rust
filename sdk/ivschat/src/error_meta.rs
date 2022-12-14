// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum Error {
    /// <p></p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p></p>
    ConflictException(crate::error::ConflictException),
    /// <p></p>
    InternalServerException(crate::error::InternalServerException),
    /// <p></p>
    PendingVerification(crate::error::PendingVerification),
    /// <p></p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p></p>
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
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
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::PendingVerification(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateChatTokenError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateChatTokenError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateChatTokenErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::CreateChatTokenErrorKind::PendingVerification(inner) => Error::PendingVerification(inner),
                crate::error::CreateChatTokenErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::CreateChatTokenErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::CreateChatTokenErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateRoomError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateRoomError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateRoomErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::CreateRoomErrorKind::ConflictException(inner) => Error::ConflictException(inner),
                crate::error::CreateRoomErrorKind::PendingVerification(inner) => Error::PendingVerification(inner),
                crate::error::CreateRoomErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::CreateRoomErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
                crate::error::CreateRoomErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::CreateRoomErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteMessageError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteMessageError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteMessageErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DeleteMessageErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DeleteMessageErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::DeleteMessageErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DeleteMessageErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteRoomError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteRoomError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteRoomErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DeleteRoomErrorKind::PendingVerification(inner) => Error::PendingVerification(inner),
                crate::error::DeleteRoomErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DeleteRoomErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DeleteRoomErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DisconnectUserError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DisconnectUserError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DisconnectUserErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::DisconnectUserErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::DisconnectUserErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::DisconnectUserErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::DisconnectUserErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetRoomError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetRoomError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetRoomErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetRoomErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::GetRoomErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetRoomErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListRoomsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListRoomsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListRoomsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::ListRoomsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::ListRoomsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::ListRoomsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListTagsForResourceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::ListTagsForResourceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SendEventError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::SendEventError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::SendEventErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::SendEventErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::SendEventErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::SendEventErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::SendEventErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::TagResourceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::TagResourceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::TagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UntagResourceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::UntagResourceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateRoomError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateRoomError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UpdateRoomErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::UpdateRoomErrorKind::PendingVerification(inner) => Error::PendingVerification(inner),
                crate::error::UpdateRoomErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::UpdateRoomErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::UpdateRoomErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}

