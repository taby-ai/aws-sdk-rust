// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum Error {
    /// <p>The request is missing required parameters or has invalid parameters.</p>
    InvalidRequestException(crate::error::InvalidRequestException),
    /// <p>The resource was not found. Verify that the name or Amazon Resource Name (ARN) of the resource is correct.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>You have reached the maximum number of sampling rules.</p>
    RuleLimitExceededException(crate::error::RuleLimitExceededException),
    /// <p>The request exceeds the maximum number of requests per second.</p>
    ThrottledException(crate::error::ThrottledException),
    /// <p>You have exceeded the maximum number of tags you can apply to this resource.</p>
    TooManyTagsException(crate::error::TooManyTagsException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>)
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::RuleLimitExceededException(inner) => inner.fmt(f),
            Error::ThrottledException(inner) => inner.fmt(f),
            Error::TooManyTagsException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::BatchGetTracesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::BatchGetTracesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::BatchGetTracesErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::BatchGetTracesErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::BatchGetTracesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateGroupError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateGroupError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateGroupErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::CreateGroupErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::CreateGroupErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateSamplingRuleError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateSamplingRuleError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::CreateSamplingRuleErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::CreateSamplingRuleErrorKind::RuleLimitExceededException(inner) => Error::RuleLimitExceededException(inner),
                crate::error::CreateSamplingRuleErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::CreateSamplingRuleErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteGroupError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteGroupError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteGroupErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::DeleteGroupErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::DeleteGroupErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteSamplingRuleError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteSamplingRuleError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::DeleteSamplingRuleErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::DeleteSamplingRuleErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::DeleteSamplingRuleErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetEncryptionConfigError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetEncryptionConfigError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetEncryptionConfigErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::GetEncryptionConfigErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetEncryptionConfigErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetGroupError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetGroupError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetGroupErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::GetGroupErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetGroupErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetGroupsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetGroupsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetGroupsErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::GetGroupsErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetGroupsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetInsightError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetInsightError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetInsightErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::GetInsightErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetInsightErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetInsightEventsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetInsightEventsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetInsightEventsErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::GetInsightEventsErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetInsightEventsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetInsightImpactGraphError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetInsightImpactGraphError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetInsightImpactGraphErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::GetInsightImpactGraphErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetInsightImpactGraphErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetInsightSummariesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetInsightSummariesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetInsightSummariesErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::GetInsightSummariesErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetInsightSummariesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSamplingRulesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetSamplingRulesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetSamplingRulesErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::GetSamplingRulesErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetSamplingRulesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSamplingStatisticSummariesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetSamplingStatisticSummariesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetSamplingStatisticSummariesErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::GetSamplingStatisticSummariesErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetSamplingStatisticSummariesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSamplingTargetsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetSamplingTargetsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetSamplingTargetsErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::GetSamplingTargetsErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetSamplingTargetsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetServiceGraphError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetServiceGraphError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetServiceGraphErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::GetServiceGraphErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetServiceGraphErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetTimeSeriesServiceStatisticsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetTimeSeriesServiceStatisticsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetTimeSeriesServiceStatisticsErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::GetTimeSeriesServiceStatisticsErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetTimeSeriesServiceStatisticsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetTraceGraphError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetTraceGraphError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetTraceGraphErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::GetTraceGraphErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetTraceGraphErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetTraceSummariesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetTraceSummariesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetTraceSummariesErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::GetTraceSummariesErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::GetTraceSummariesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ListTagsForResourceErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::ListTagsForResourceErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutEncryptionConfigError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutEncryptionConfigError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::PutEncryptionConfigErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::PutEncryptionConfigErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::PutEncryptionConfigErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutTelemetryRecordsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutTelemetryRecordsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::PutTelemetryRecordsErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::PutTelemetryRecordsErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::PutTelemetryRecordsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutTraceSegmentsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutTraceSegmentsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::PutTraceSegmentsErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::PutTraceSegmentsErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::PutTraceSegmentsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::TagResourceErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::TagResourceErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::TagResourceErrorKind::TooManyTagsException(inner) => Error::TooManyTagsException(inner),
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
                crate::error::UntagResourceErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
                crate::error::UntagResourceErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateGroupError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateGroupError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UpdateGroupErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::UpdateGroupErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::UpdateGroupErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateSamplingRuleError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateSamplingRuleError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::UpdateSamplingRuleErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
                crate::error::UpdateSamplingRuleErrorKind::ThrottledException(inner) => Error::ThrottledException(inner),
                crate::error::UpdateSamplingRuleErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}

