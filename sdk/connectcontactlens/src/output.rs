// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct ListRealtimeContactAnalysisSegmentsOutput  {
    /// <p>An analyzed transcript or category.</p>
    #[doc(hidden)]pub segments: std::option::Option<std::vec::Vec<crate::model::RealtimeContactAnalysisSegment>>,
    /// <p>If there are additional results, this is the token for the next set of results. If response includes <code>nextToken</code> there are two possible scenarios:</p> 
    /// <ul> 
    /// <li> <p>There are more segments so another call is required to get them.</p> </li> 
    /// <li> <p>There are no more segments at this time, but more may be available later (real-time analysis is in progress) so the client should call the operation again to get new segments.</p> </li> 
    /// </ul> 
    /// <p>If response does not include <code>nextToken</code>, the analysis is completed (successfully or failed) and there are no more segments to retrieve.</p>
    #[doc(hidden)]pub next_token: std::option::Option<std::string::String>,
}
impl ListRealtimeContactAnalysisSegmentsOutput {
    /// <p>An analyzed transcript or category.</p>
    pub fn segments(&self) -> std::option::Option<& [crate::model::RealtimeContactAnalysisSegment]> {
        self.segments.as_deref()
    }
    /// <p>If there are additional results, this is the token for the next set of results. If response includes <code>nextToken</code> there are two possible scenarios:</p> 
    /// <ul> 
    /// <li> <p>There are more segments so another call is required to get them.</p> </li> 
    /// <li> <p>There are no more segments at this time, but more may be available later (real-time analysis is in progress) so the client should call the operation again to get new segments.</p> </li> 
    /// </ul> 
    /// <p>If response does not include <code>nextToken</code>, the analysis is completed (successfully or failed) and there are no more segments to retrieve.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
impl  std::fmt::Debug for ListRealtimeContactAnalysisSegmentsOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ListRealtimeContactAnalysisSegmentsOutput");
        formatter.field("segments", &self.segments);
        formatter.field("next_token", &self.next_token);
        formatter.finish()
    }
}
/// See [`ListRealtimeContactAnalysisSegmentsOutput`](crate::output::ListRealtimeContactAnalysisSegmentsOutput).
pub mod list_realtime_contact_analysis_segments_output {
    
    /// A builder for [`ListRealtimeContactAnalysisSegmentsOutput`](crate::output::ListRealtimeContactAnalysisSegmentsOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) segments: std::option::Option<std::vec::Vec<crate::model::RealtimeContactAnalysisSegment>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `segments`.
        ///
        /// To override the contents of this collection use [`set_segments`](Self::set_segments).
        ///
        /// <p>An analyzed transcript or category.</p>
        pub fn segments(mut self, input: crate::model::RealtimeContactAnalysisSegment) -> Self {
            let mut v = self.segments.unwrap_or_default();
                            v.push(input);
                            self.segments = Some(v);
                            self
        }
        /// <p>An analyzed transcript or category.</p>
        pub fn set_segments(mut self, input: std::option::Option<std::vec::Vec<crate::model::RealtimeContactAnalysisSegment>>) -> Self {
            self.segments = input; self
        }
        /// <p>If there are additional results, this is the token for the next set of results. If response includes <code>nextToken</code> there are two possible scenarios:</p> 
        /// <ul> 
        /// <li> <p>There are more segments so another call is required to get them.</p> </li> 
        /// <li> <p>There are no more segments at this time, but more may be available later (real-time analysis is in progress) so the client should call the operation again to get new segments.</p> </li> 
        /// </ul> 
        /// <p>If response does not include <code>nextToken</code>, the analysis is completed (successfully or failed) and there are no more segments to retrieve.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>If there are additional results, this is the token for the next set of results. If response includes <code>nextToken</code> there are two possible scenarios:</p> 
        /// <ul> 
        /// <li> <p>There are more segments so another call is required to get them.</p> </li> 
        /// <li> <p>There are no more segments at this time, but more may be available later (real-time analysis is in progress) so the client should call the operation again to get new segments.</p> </li> 
        /// </ul> 
        /// <p>If response does not include <code>nextToken</code>, the analysis is completed (successfully or failed) and there are no more segments to retrieve.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Consumes the builder and constructs a [`ListRealtimeContactAnalysisSegmentsOutput`](crate::output::ListRealtimeContactAnalysisSegmentsOutput).
        pub fn build(self) -> crate::output::ListRealtimeContactAnalysisSegmentsOutput {
            crate::output::ListRealtimeContactAnalysisSegmentsOutput {
                segments: self.segments
                ,
                next_token: self.next_token
                ,
            }
        }
    }
    
    
}
impl ListRealtimeContactAnalysisSegmentsOutput {
    /// Creates a new builder-style object to manufacture [`ListRealtimeContactAnalysisSegmentsOutput`](crate::output::ListRealtimeContactAnalysisSegmentsOutput).
    pub fn builder() -> crate::output::list_realtime_contact_analysis_segments_output::Builder {
        crate::output::list_realtime_contact_analysis_segments_output::Builder::default()
    }
}

