// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum Error {
    /// <p>The target of the operation is currently being modified by a different request. Try again later.</p>
    ConcurrentModificationException(crate::error::ConcurrentModificationException),
    /// <p>The request was denied because performing this operation violates a constraint. </p> 
    /// <p>Some of the reasons in the following list might not apply to this specific operation.</p> 
    /// <ul> 
    /// <li> <p>You must meet the prerequisites for using tag policies. For information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html">Prerequisites and Permissions for Using Tag Policies</a> in the <i>Organizations User Guide.</i> </p> </li> 
    /// <li> <p>You must enable the tag policies service principal (<code>tagpolicies.tag.amazonaws.com</code>) to integrate with Organizations For information, see <a href="https://docs.aws.amazon.com/organizations/latest/APIReference/API_EnableAWSServiceAccess.html">EnableAWSServiceAccess</a>.</p> </li> 
    /// <li> <p>You must have a tag policy attached to the organization root, an OU, or an account.</p> </li> 
    /// </ul>
    ConstraintViolationException(crate::error::ConstraintViolationException),
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalServiceException(crate::error::InternalServiceException),
    /// <p>This error indicates one of the following:</p> 
    /// <ul> 
    /// <li> <p>A parameter is missing.</p> </li> 
    /// <li> <p>A malformed string was supplied for the request parameter.</p> </li> 
    /// <li> <p>An out-of-range value was supplied for the request parameter.</p> </li> 
    /// <li> <p>The target ID is invalid, unsupported, or doesn't exist.</p> </li> 
    /// <li> <p>You can't access the Amazon S3 bucket for report storage. For more information, see <a href="https://docs.aws.amazon.com/organizations/latest/userguide/orgs_manage_policies_tag-policies-prereqs.html#bucket-policies-org-report">Additional Requirements for Organization-wide Tag Compliance Reports</a> in the <i>Organizations User Guide.</i> </p> </li> 
    /// </ul>
    InvalidParameterException(crate::error::InvalidParameterException),
    /// <p>A <code>PaginationToken</code> is valid for a maximum of 15 minutes. Your request was denied because the specified <code>PaginationToken</code> has expired.</p>
    PaginationTokenExpiredException(crate::error::PaginationTokenExpiredException),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    ThrottledException(crate::error::ThrottledException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>)
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConcurrentModificationException(inner) => inner.fmt(f),
            Error::ConstraintViolationException(inner) => inner.fmt(f),
            Error::InternalServiceException(inner) => inner.fmt(f),
            Error::InvalidParameterException(inner) => inner.fmt(f),
            Error::PaginationTokenExpiredException(inner) => inner.fmt(f),
            Error::ThrottledException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeReportCreationError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeReportCreationError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DescribeReportCreationErrorKind::ConstraintViolationException(inner) => Error::ConstraintViolationException(inner),
                crate::error::DescribeReportCreationErrorKind::InternalServiceException(inner) => Error::InternalServiceException(inner),
                crate::error::DescribeReportCreationErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::DescribeReportCreationErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::DescribeReportCreationErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetComplianceSummaryError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetComplianceSummaryError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetComplianceSummaryErrorKind::ConstraintViolationException(inner) => Error::ConstraintViolationException(inner),
                crate::error::GetComplianceSummaryErrorKind::InternalServiceException(inner) => Error::InternalServiceException(inner),
                crate::error::GetComplianceSummaryErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::GetComplianceSummaryErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetComplianceSummaryErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetResourcesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetResourcesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetResourcesErrorKind::InternalServiceException(inner) => Error::InternalServiceException(inner),
                crate::error::GetResourcesErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::GetResourcesErrorKind::PaginationTokenExpiredException(inner) => Error::PaginationTokenExpiredException(inner),
                crate::error::GetResourcesErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetResourcesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetTagKeysError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetTagKeysError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetTagKeysErrorKind::InternalServiceException(inner) => Error::InternalServiceException(inner),
                crate::error::GetTagKeysErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::GetTagKeysErrorKind::PaginationTokenExpiredException(inner) => Error::PaginationTokenExpiredException(inner),
                crate::error::GetTagKeysErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetTagKeysErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetTagValuesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetTagValuesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetTagValuesErrorKind::InternalServiceException(inner) => Error::InternalServiceException(inner),
                crate::error::GetTagValuesErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::GetTagValuesErrorKind::PaginationTokenExpiredException(inner) => Error::PaginationTokenExpiredException(inner),
                crate::error::GetTagValuesErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetTagValuesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartReportCreationError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StartReportCreationError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::StartReportCreationErrorKind::ConcurrentModificationException(inner) => Error::ConcurrentModificationException(inner),
                crate::error::StartReportCreationErrorKind::ConstraintViolationException(inner) => Error::ConstraintViolationException(inner),
                crate::error::StartReportCreationErrorKind::InternalServiceException(inner) => Error::InternalServiceException(inner),
                crate::error::StartReportCreationErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::StartReportCreationErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::StartReportCreationErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagResourcesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagResourcesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::TagResourcesErrorKind::InternalServiceException(inner) => Error::InternalServiceException(inner),
                crate::error::TagResourcesErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::TagResourcesErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::TagResourcesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagResourcesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UntagResourcesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UntagResourcesErrorKind::InternalServiceException(inner) => Error::InternalServiceException(inner),
                crate::error::UntagResourcesErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
                crate::error::UntagResourcesErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::UntagResourcesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}

