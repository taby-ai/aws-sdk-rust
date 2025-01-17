// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListAnomalyGroupSummariesOutput {
    /// <p>A list of anomaly group summaries.</p>
    #[doc(hidden)]
    pub anomaly_group_summary_list:
        std::option::Option<std::vec::Vec<crate::types::AnomalyGroupSummary>>,
    /// <p>Aggregated details about the anomaly groups.</p>
    #[doc(hidden)]
    pub anomaly_group_statistics: std::option::Option<crate::types::AnomalyGroupStatistics>,
    /// <p>The pagination token that's included if more results are available.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListAnomalyGroupSummariesOutput {
    /// <p>A list of anomaly group summaries.</p>
    pub fn anomaly_group_summary_list(
        &self,
    ) -> std::option::Option<&[crate::types::AnomalyGroupSummary]> {
        self.anomaly_group_summary_list.as_deref()
    }
    /// <p>Aggregated details about the anomaly groups.</p>
    pub fn anomaly_group_statistics(
        &self,
    ) -> std::option::Option<&crate::types::AnomalyGroupStatistics> {
        self.anomaly_group_statistics.as_ref()
    }
    /// <p>The pagination token that's included if more results are available.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListAnomalyGroupSummariesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListAnomalyGroupSummariesOutput {
    /// Creates a new builder-style object to manufacture [`ListAnomalyGroupSummariesOutput`](crate::operation::list_anomaly_group_summaries::ListAnomalyGroupSummariesOutput).
    pub fn builder() -> crate::operation::list_anomaly_group_summaries::builders::ListAnomalyGroupSummariesOutputBuilder{
        crate::operation::list_anomaly_group_summaries::builders::ListAnomalyGroupSummariesOutputBuilder::default()
    }
}

/// A builder for [`ListAnomalyGroupSummariesOutput`](crate::operation::list_anomaly_group_summaries::ListAnomalyGroupSummariesOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct ListAnomalyGroupSummariesOutputBuilder {
    pub(crate) anomaly_group_summary_list:
        std::option::Option<std::vec::Vec<crate::types::AnomalyGroupSummary>>,
    pub(crate) anomaly_group_statistics: std::option::Option<crate::types::AnomalyGroupStatistics>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListAnomalyGroupSummariesOutputBuilder {
    /// Appends an item to `anomaly_group_summary_list`.
    ///
    /// To override the contents of this collection use [`set_anomaly_group_summary_list`](Self::set_anomaly_group_summary_list).
    ///
    /// <p>A list of anomaly group summaries.</p>
    pub fn anomaly_group_summary_list(mut self, input: crate::types::AnomalyGroupSummary) -> Self {
        let mut v = self.anomaly_group_summary_list.unwrap_or_default();
        v.push(input);
        self.anomaly_group_summary_list = Some(v);
        self
    }
    /// <p>A list of anomaly group summaries.</p>
    pub fn set_anomaly_group_summary_list(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::AnomalyGroupSummary>>,
    ) -> Self {
        self.anomaly_group_summary_list = input;
        self
    }
    /// <p>Aggregated details about the anomaly groups.</p>
    pub fn anomaly_group_statistics(mut self, input: crate::types::AnomalyGroupStatistics) -> Self {
        self.anomaly_group_statistics = Some(input);
        self
    }
    /// <p>Aggregated details about the anomaly groups.</p>
    pub fn set_anomaly_group_statistics(
        mut self,
        input: std::option::Option<crate::types::AnomalyGroupStatistics>,
    ) -> Self {
        self.anomaly_group_statistics = input;
        self
    }
    /// <p>The pagination token that's included if more results are available.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The pagination token that's included if more results are available.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListAnomalyGroupSummariesOutput`](crate::operation::list_anomaly_group_summaries::ListAnomalyGroupSummariesOutput).
    pub fn build(
        self,
    ) -> crate::operation::list_anomaly_group_summaries::ListAnomalyGroupSummariesOutput {
        crate::operation::list_anomaly_group_summaries::ListAnomalyGroupSummariesOutput {
            anomaly_group_summary_list: self.anomaly_group_summary_list,
            anomaly_group_statistics: self.anomaly_group_statistics,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
