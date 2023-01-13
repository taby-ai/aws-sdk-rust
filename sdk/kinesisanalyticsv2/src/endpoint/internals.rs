// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::collapsible_if)]
#[allow(clippy::bool_comparison)]
#[allow(clippy::nonminimal_bool)]
#[allow(clippy::comparison_to_empty)]
#[allow(clippy::redundant_pattern_matching)]
pub(super) fn resolve_endpoint(
    _params: &crate::endpoint::Params,
    _diagnostic_collector: &mut crate::endpoint_lib::diagnostic::DiagnosticCollector,
    partition_resolver: &crate::endpoint_lib::partition::PartitionResolver,
) -> aws_smithy_http::endpoint::Result {
    #[allow(unused)]
    let region = &_params.region;
    #[allow(unused)]
    let use_dual_stack = &_params.use_dual_stack;
    #[allow(unused)]
    let use_fips = &_params.use_fips;
    #[allow(unused)]
    let endpoint = &_params.endpoint;
    #[allow(unused)]
    if let Some(partition_result) =
        partition_resolver.resolve_partition(region, _diagnostic_collector)
    {
        #[allow(unused)]
        if let Some(endpoint) = endpoint {
            #[allow(unused)]
            if let Some(url) =
                crate::endpoint_lib::parse_url::parse_url(endpoint, _diagnostic_collector)
            {
                if (*use_fips) == (true) {
                    return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                        "Invalid Configuration: FIPS and custom endpoint are not supported"
                            .to_string(),
                    ));
                }
                if (*use_dual_stack) == (true) {
                    return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                        "Invalid Configuration: Dualstack and custom endpoint are not supported"
                            .to_string(),
                    ));
                }
                return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                    .url(endpoint.to_owned())
                    .build());
            }
        }
        if (*use_fips) == (true) {
            if (*use_dual_stack) == (true) {
                if (true) == (partition_result.supports_fips()) {
                    if (true) == (partition_result.supports_dual_stack()) {
                        return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                            .url({
                                let mut out = String::new();
                                out.push_str("https://kinesisanalytics-fips.");
                                #[allow(clippy::needless_borrow)]
                                out.push_str(&region);
                                out.push('.');
                                #[allow(clippy::needless_borrow)]
                                out.push_str(&partition_result.dual_stack_dns_suffix());
                                out
                            })
                            .build());
                    }
                }
                return Err(aws_smithy_http::endpoint::ResolveEndpointError::message("FIPS and DualStack are enabled, but this partition does not support one or both"
.to_string()));
            }
        }
        if (*use_fips) == (true) {
            if (true) == (partition_result.supports_fips()) {
                return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                    .url({
                        let mut out = String::new();
                        out.push_str("https://kinesisanalytics-fips.");
                        #[allow(clippy::needless_borrow)]
                        out.push_str(&region);
                        out.push('.');
                        #[allow(clippy::needless_borrow)]
                        out.push_str(&partition_result.dns_suffix());
                        out
                    })
                    .build());
            }
            return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                "FIPS is enabled but this partition does not support FIPS".to_string(),
            ));
        }
        if (*use_dual_stack) == (true) {
            if (true) == (partition_result.supports_dual_stack()) {
                return Ok(aws_smithy_types::endpoint::Endpoint::builder()
                    .url({
                        let mut out = String::new();
                        out.push_str("https://kinesisanalytics.");
                        #[allow(clippy::needless_borrow)]
                        out.push_str(&region);
                        out.push('.');
                        #[allow(clippy::needless_borrow)]
                        out.push_str(&partition_result.dual_stack_dns_suffix());
                        out
                    })
                    .build());
            }
            return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
                "DualStack is enabled but this partition does not support DualStack".to_string(),
            ));
        }
        return Ok(aws_smithy_types::endpoint::Endpoint::builder()
            .url({
                let mut out = String::new();
                out.push_str("https://kinesisanalytics.");
                #[allow(clippy::needless_borrow)]
                out.push_str(&region);
                out.push('.');
                #[allow(clippy::needless_borrow)]
                out.push_str(&partition_result.dns_suffix());
                out
            })
            .build());
    }
    #[allow(unreachable_code)]
    return Err(aws_smithy_http::endpoint::ResolveEndpointError::message(
        format!(
            "No rules matched these parameters. This is a bug. {:?}",
            _params
        ),
    ));
}
