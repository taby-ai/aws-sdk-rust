// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the response to an <code>UploadDocuments</code> request.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct UploadDocumentsOutput  {
    /// <p>The status of an <code>UploadDocumentsRequest</code>.</p>
    #[doc(hidden)]pub status: std::option::Option<std::string::String>,
    /// <p>The number of documents that were added to the search domain.</p>
    #[doc(hidden)]pub adds: i64,
    /// <p>The number of documents that were deleted from the search domain.</p>
    #[doc(hidden)]pub deletes: i64,
    /// <p>Any warnings returned by the document service about the documents being uploaded.</p>
    #[doc(hidden)]pub warnings: std::option::Option<std::vec::Vec<crate::model::DocumentServiceWarning>>,
}
impl UploadDocumentsOutput {
    /// <p>The status of an <code>UploadDocumentsRequest</code>.</p>
    pub fn status(&self) -> std::option::Option<& str> {
        self.status.as_deref()
    }
    /// <p>The number of documents that were added to the search domain.</p>
    pub fn adds(&self) -> i64 {
        self.adds
    }
    /// <p>The number of documents that were deleted from the search domain.</p>
    pub fn deletes(&self) -> i64 {
        self.deletes
    }
    /// <p>Any warnings returned by the document service about the documents being uploaded.</p>
    pub fn warnings(&self) -> std::option::Option<& [crate::model::DocumentServiceWarning]> {
        self.warnings.as_deref()
    }
}
impl  std::fmt::Debug for UploadDocumentsOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UploadDocumentsOutput");
        formatter.field("status", &self.status);
        formatter.field("adds", &self.adds);
        formatter.field("deletes", &self.deletes);
        formatter.field("warnings", &self.warnings);
        formatter.finish()
    }
}
/// See [`UploadDocumentsOutput`](crate::output::UploadDocumentsOutput).
pub mod upload_documents_output {
    
    /// A builder for [`UploadDocumentsOutput`](crate::output::UploadDocumentsOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) status: std::option::Option<std::string::String>,
        pub(crate) adds: std::option::Option<i64>,
        pub(crate) deletes: std::option::Option<i64>,
        pub(crate) warnings: std::option::Option<std::vec::Vec<crate::model::DocumentServiceWarning>>,
    }
    impl Builder {
        /// <p>The status of an <code>UploadDocumentsRequest</code>.</p>
        pub fn status(mut self, input: impl Into<std::string::String>) -> Self {
            self.status = Some(input.into());
            self
        }
        /// <p>The status of an <code>UploadDocumentsRequest</code>.</p>
        pub fn set_status(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.status = input; self
        }
        /// <p>The number of documents that were added to the search domain.</p>
        pub fn adds(mut self, input: i64) -> Self {
            self.adds = Some(input);
            self
        }
        /// <p>The number of documents that were added to the search domain.</p>
        pub fn set_adds(mut self, input: std::option::Option<i64>) -> Self {
            self.adds = input; self
        }
        /// <p>The number of documents that were deleted from the search domain.</p>
        pub fn deletes(mut self, input: i64) -> Self {
            self.deletes = Some(input);
            self
        }
        /// <p>The number of documents that were deleted from the search domain.</p>
        pub fn set_deletes(mut self, input: std::option::Option<i64>) -> Self {
            self.deletes = input; self
        }
        /// Appends an item to `warnings`.
        ///
        /// To override the contents of this collection use [`set_warnings`](Self::set_warnings).
        ///
        /// <p>Any warnings returned by the document service about the documents being uploaded.</p>
        pub fn warnings(mut self, input: crate::model::DocumentServiceWarning) -> Self {
            let mut v = self.warnings.unwrap_or_default();
                            v.push(input);
                            self.warnings = Some(v);
                            self
        }
        /// <p>Any warnings returned by the document service about the documents being uploaded.</p>
        pub fn set_warnings(mut self, input: std::option::Option<std::vec::Vec<crate::model::DocumentServiceWarning>>) -> Self {
            self.warnings = input; self
        }
        /// Consumes the builder and constructs a [`UploadDocumentsOutput`](crate::output::UploadDocumentsOutput).
        pub fn build(self) -> crate::output::UploadDocumentsOutput {
            crate::output::UploadDocumentsOutput {
                status: self.status
                ,
                adds: self.adds
                    .unwrap_or_default()
                ,
                deletes: self.deletes
                    .unwrap_or_default()
                ,
                warnings: self.warnings
                ,
            }
        }
    }
    
    
}
impl UploadDocumentsOutput {
    /// Creates a new builder-style object to manufacture [`UploadDocumentsOutput`](crate::output::UploadDocumentsOutput).
    pub fn builder() -> crate::output::upload_documents_output::Builder {
        crate::output::upload_documents_output::Builder::default()
    }
}

/// <p>Contains the response to a <code>Suggest</code> request.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct SuggestOutput  {
    /// <p>The status of a <code>SuggestRequest</code>. Contains the resource ID (<code>rid</code>) and how long it took to process the request (<code>timems</code>).</p>
    #[doc(hidden)]pub status: std::option::Option<crate::model::SuggestStatus>,
    /// <p>Container for the matching search suggestion information.</p>
    #[doc(hidden)]pub suggest: std::option::Option<crate::model::SuggestModel>,
}
impl SuggestOutput {
    /// <p>The status of a <code>SuggestRequest</code>. Contains the resource ID (<code>rid</code>) and how long it took to process the request (<code>timems</code>).</p>
    pub fn status(&self) -> std::option::Option<& crate::model::SuggestStatus> {
        self.status.as_ref()
    }
    /// <p>Container for the matching search suggestion information.</p>
    pub fn suggest(&self) -> std::option::Option<& crate::model::SuggestModel> {
        self.suggest.as_ref()
    }
}
impl  std::fmt::Debug for SuggestOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SuggestOutput");
        formatter.field("status", &self.status);
        formatter.field("suggest", &self.suggest);
        formatter.finish()
    }
}
/// See [`SuggestOutput`](crate::output::SuggestOutput).
pub mod suggest_output {
    
    /// A builder for [`SuggestOutput`](crate::output::SuggestOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) status: std::option::Option<crate::model::SuggestStatus>,
        pub(crate) suggest: std::option::Option<crate::model::SuggestModel>,
    }
    impl Builder {
        /// <p>The status of a <code>SuggestRequest</code>. Contains the resource ID (<code>rid</code>) and how long it took to process the request (<code>timems</code>).</p>
        pub fn status(mut self, input: crate::model::SuggestStatus) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>The status of a <code>SuggestRequest</code>. Contains the resource ID (<code>rid</code>) and how long it took to process the request (<code>timems</code>).</p>
        pub fn set_status(mut self, input: std::option::Option<crate::model::SuggestStatus>) -> Self {
            self.status = input; self
        }
        /// <p>Container for the matching search suggestion information.</p>
        pub fn suggest(mut self, input: crate::model::SuggestModel) -> Self {
            self.suggest = Some(input);
            self
        }
        /// <p>Container for the matching search suggestion information.</p>
        pub fn set_suggest(mut self, input: std::option::Option<crate::model::SuggestModel>) -> Self {
            self.suggest = input; self
        }
        /// Consumes the builder and constructs a [`SuggestOutput`](crate::output::SuggestOutput).
        pub fn build(self) -> crate::output::SuggestOutput {
            crate::output::SuggestOutput {
                status: self.status
                ,
                suggest: self.suggest
                ,
            }
        }
    }
    
    
}
impl SuggestOutput {
    /// Creates a new builder-style object to manufacture [`SuggestOutput`](crate::output::SuggestOutput).
    pub fn builder() -> crate::output::suggest_output::Builder {
        crate::output::suggest_output::Builder::default()
    }
}

/// <p>The result of a <code>Search</code> request. Contains the documents that match the specified search criteria and any requested fields, highlights, and facet information.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct SearchOutput  {
    /// <p>The status information returned for the search request.</p>
    #[doc(hidden)]pub status: std::option::Option<crate::model::SearchStatus>,
    /// <p>The documents that match the search criteria.</p>
    #[doc(hidden)]pub hits: std::option::Option<crate::model::Hits>,
    /// <p>The requested facet information.</p>
    #[doc(hidden)]pub facets: std::option::Option<std::collections::HashMap<std::string::String, crate::model::BucketInfo>>,
    /// <p>The requested field statistics information.</p>
    #[doc(hidden)]pub stats: std::option::Option<std::collections::HashMap<std::string::String, crate::model::FieldStats>>,
}
impl SearchOutput {
    /// <p>The status information returned for the search request.</p>
    pub fn status(&self) -> std::option::Option<& crate::model::SearchStatus> {
        self.status.as_ref()
    }
    /// <p>The documents that match the search criteria.</p>
    pub fn hits(&self) -> std::option::Option<& crate::model::Hits> {
        self.hits.as_ref()
    }
    /// <p>The requested facet information.</p>
    pub fn facets(&self) -> std::option::Option<& std::collections::HashMap<std::string::String, crate::model::BucketInfo>> {
        self.facets.as_ref()
    }
    /// <p>The requested field statistics information.</p>
    pub fn stats(&self) -> std::option::Option<& std::collections::HashMap<std::string::String, crate::model::FieldStats>> {
        self.stats.as_ref()
    }
}
impl  std::fmt::Debug for SearchOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("SearchOutput");
        formatter.field("status", &self.status);
        formatter.field("hits", &self.hits);
        formatter.field("facets", &self.facets);
        formatter.field("stats", &self.stats);
        formatter.finish()
    }
}
/// See [`SearchOutput`](crate::output::SearchOutput).
pub mod search_output {
    
    /// A builder for [`SearchOutput`](crate::output::SearchOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) status: std::option::Option<crate::model::SearchStatus>,
        pub(crate) hits: std::option::Option<crate::model::Hits>,
        pub(crate) facets: std::option::Option<std::collections::HashMap<std::string::String, crate::model::BucketInfo>>,
        pub(crate) stats: std::option::Option<std::collections::HashMap<std::string::String, crate::model::FieldStats>>,
    }
    impl Builder {
        /// <p>The status information returned for the search request.</p>
        pub fn status(mut self, input: crate::model::SearchStatus) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>The status information returned for the search request.</p>
        pub fn set_status(mut self, input: std::option::Option<crate::model::SearchStatus>) -> Self {
            self.status = input; self
        }
        /// <p>The documents that match the search criteria.</p>
        pub fn hits(mut self, input: crate::model::Hits) -> Self {
            self.hits = Some(input);
            self
        }
        /// <p>The documents that match the search criteria.</p>
        pub fn set_hits(mut self, input: std::option::Option<crate::model::Hits>) -> Self {
            self.hits = input; self
        }
        /// Adds a key-value pair to `facets`.
        ///
        /// To override the contents of this collection use [`set_facets`](Self::set_facets).
        ///
        /// <p>The requested facet information.</p>
        pub fn facets(mut self, k: impl Into<std::string::String>, v: crate::model::BucketInfo) -> Self {
            let mut hash_map = self.facets.unwrap_or_default();
                            hash_map.insert(k.into(), v);
                            self.facets = Some(hash_map);
                            self
        }
        /// <p>The requested facet information.</p>
        pub fn set_facets(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, crate::model::BucketInfo>>) -> Self {
            self.facets = input; self
        }
        /// Adds a key-value pair to `stats`.
        ///
        /// To override the contents of this collection use [`set_stats`](Self::set_stats).
        ///
        /// <p>The requested field statistics information.</p>
        pub fn stats(mut self, k: impl Into<std::string::String>, v: crate::model::FieldStats) -> Self {
            let mut hash_map = self.stats.unwrap_or_default();
                            hash_map.insert(k.into(), v);
                            self.stats = Some(hash_map);
                            self
        }
        /// <p>The requested field statistics information.</p>
        pub fn set_stats(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, crate::model::FieldStats>>) -> Self {
            self.stats = input; self
        }
        /// Consumes the builder and constructs a [`SearchOutput`](crate::output::SearchOutput).
        pub fn build(self) -> crate::output::SearchOutput {
            crate::output::SearchOutput {
                status: self.status
                ,
                hits: self.hits
                ,
                facets: self.facets
                ,
                stats: self.stats
                ,
            }
        }
    }
    
    
}
impl SearchOutput {
    /// Creates a new builder-style object to manufacture [`SearchOutput`](crate::output::SearchOutput).
    pub fn builder() -> crate::output::search_output::Builder {
        crate::output::search_output::Builder::default()
    }
}

