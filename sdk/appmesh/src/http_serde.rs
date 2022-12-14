// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_payload_create_gateway_route_create_gateway_route_output_gateway_route(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::GatewayRouteData>, crate::error::CreateGatewayRouteError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_gateway_route_data_payload(body).map_err(crate::error::CreateGatewayRouteError::unhandled)
    }).transpose()
}

pub fn deser_payload_create_mesh_create_mesh_output_mesh(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::MeshData>, crate::error::CreateMeshError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_mesh_data_payload(body).map_err(crate::error::CreateMeshError::unhandled)
    }).transpose()
}

pub fn deser_payload_create_route_create_route_output_route(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::RouteData>, crate::error::CreateRouteError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_route_data_payload(body).map_err(crate::error::CreateRouteError::unhandled)
    }).transpose()
}

pub fn deser_payload_create_virtual_gateway_create_virtual_gateway_output_virtual_gateway(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VirtualGatewayData>, crate::error::CreateVirtualGatewayError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_virtual_gateway_data_payload(body).map_err(crate::error::CreateVirtualGatewayError::unhandled)
    }).transpose()
}

pub fn deser_payload_create_virtual_node_create_virtual_node_output_virtual_node(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VirtualNodeData>, crate::error::CreateVirtualNodeError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_virtual_node_data_payload(body).map_err(crate::error::CreateVirtualNodeError::unhandled)
    }).transpose()
}

pub fn deser_payload_create_virtual_router_create_virtual_router_output_virtual_router(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VirtualRouterData>, crate::error::CreateVirtualRouterError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_virtual_router_data_payload(body).map_err(crate::error::CreateVirtualRouterError::unhandled)
    }).transpose()
}

pub fn deser_payload_create_virtual_service_create_virtual_service_output_virtual_service(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VirtualServiceData>, crate::error::CreateVirtualServiceError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_virtual_service_data_payload(body).map_err(crate::error::CreateVirtualServiceError::unhandled)
    }).transpose()
}

pub fn deser_payload_delete_gateway_route_delete_gateway_route_output_gateway_route(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::GatewayRouteData>, crate::error::DeleteGatewayRouteError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_gateway_route_data_payload(body).map_err(crate::error::DeleteGatewayRouteError::unhandled)
    }).transpose()
}

pub fn deser_payload_delete_mesh_delete_mesh_output_mesh(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::MeshData>, crate::error::DeleteMeshError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_mesh_data_payload(body).map_err(crate::error::DeleteMeshError::unhandled)
    }).transpose()
}

pub fn deser_payload_delete_route_delete_route_output_route(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::RouteData>, crate::error::DeleteRouteError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_route_data_payload(body).map_err(crate::error::DeleteRouteError::unhandled)
    }).transpose()
}

pub fn deser_payload_delete_virtual_gateway_delete_virtual_gateway_output_virtual_gateway(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VirtualGatewayData>, crate::error::DeleteVirtualGatewayError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_virtual_gateway_data_payload(body).map_err(crate::error::DeleteVirtualGatewayError::unhandled)
    }).transpose()
}

pub fn deser_payload_delete_virtual_node_delete_virtual_node_output_virtual_node(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VirtualNodeData>, crate::error::DeleteVirtualNodeError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_virtual_node_data_payload(body).map_err(crate::error::DeleteVirtualNodeError::unhandled)
    }).transpose()
}

pub fn deser_payload_delete_virtual_router_delete_virtual_router_output_virtual_router(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VirtualRouterData>, crate::error::DeleteVirtualRouterError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_virtual_router_data_payload(body).map_err(crate::error::DeleteVirtualRouterError::unhandled)
    }).transpose()
}

pub fn deser_payload_delete_virtual_service_delete_virtual_service_output_virtual_service(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VirtualServiceData>, crate::error::DeleteVirtualServiceError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_virtual_service_data_payload(body).map_err(crate::error::DeleteVirtualServiceError::unhandled)
    }).transpose()
}

pub fn deser_payload_describe_gateway_route_describe_gateway_route_output_gateway_route(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::GatewayRouteData>, crate::error::DescribeGatewayRouteError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_gateway_route_data_payload(body).map_err(crate::error::DescribeGatewayRouteError::unhandled)
    }).transpose()
}

pub fn deser_payload_describe_mesh_describe_mesh_output_mesh(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::MeshData>, crate::error::DescribeMeshError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_mesh_data_payload(body).map_err(crate::error::DescribeMeshError::unhandled)
    }).transpose()
}

pub fn deser_payload_describe_route_describe_route_output_route(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::RouteData>, crate::error::DescribeRouteError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_route_data_payload(body).map_err(crate::error::DescribeRouteError::unhandled)
    }).transpose()
}

pub fn deser_payload_describe_virtual_gateway_describe_virtual_gateway_output_virtual_gateway(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VirtualGatewayData>, crate::error::DescribeVirtualGatewayError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_virtual_gateway_data_payload(body).map_err(crate::error::DescribeVirtualGatewayError::unhandled)
    }).transpose()
}

pub fn deser_payload_describe_virtual_node_describe_virtual_node_output_virtual_node(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VirtualNodeData>, crate::error::DescribeVirtualNodeError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_virtual_node_data_payload(body).map_err(crate::error::DescribeVirtualNodeError::unhandled)
    }).transpose()
}

pub fn deser_payload_describe_virtual_router_describe_virtual_router_output_virtual_router(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VirtualRouterData>, crate::error::DescribeVirtualRouterError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_virtual_router_data_payload(body).map_err(crate::error::DescribeVirtualRouterError::unhandled)
    }).transpose()
}

pub fn deser_payload_describe_virtual_service_describe_virtual_service_output_virtual_service(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VirtualServiceData>, crate::error::DescribeVirtualServiceError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_virtual_service_data_payload(body).map_err(crate::error::DescribeVirtualServiceError::unhandled)
    }).transpose()
}

pub fn deser_payload_update_gateway_route_update_gateway_route_output_gateway_route(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::GatewayRouteData>, crate::error::UpdateGatewayRouteError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_gateway_route_data_payload(body).map_err(crate::error::UpdateGatewayRouteError::unhandled)
    }).transpose()
}

pub fn deser_payload_update_mesh_update_mesh_output_mesh(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::MeshData>, crate::error::UpdateMeshError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_mesh_data_payload(body).map_err(crate::error::UpdateMeshError::unhandled)
    }).transpose()
}

pub fn deser_payload_update_route_update_route_output_route(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::RouteData>, crate::error::UpdateRouteError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_route_data_payload(body).map_err(crate::error::UpdateRouteError::unhandled)
    }).transpose()
}

pub fn deser_payload_update_virtual_gateway_update_virtual_gateway_output_virtual_gateway(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VirtualGatewayData>, crate::error::UpdateVirtualGatewayError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_virtual_gateway_data_payload(body).map_err(crate::error::UpdateVirtualGatewayError::unhandled)
    }).transpose()
}

pub fn deser_payload_update_virtual_node_update_virtual_node_output_virtual_node(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VirtualNodeData>, crate::error::UpdateVirtualNodeError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_virtual_node_data_payload(body).map_err(crate::error::UpdateVirtualNodeError::unhandled)
    }).transpose()
}

pub fn deser_payload_update_virtual_router_update_virtual_router_output_virtual_router(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VirtualRouterData>, crate::error::UpdateVirtualRouterError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_virtual_router_data_payload(body).map_err(crate::error::UpdateVirtualRouterError::unhandled)
    }).transpose()
}

pub fn deser_payload_update_virtual_service_update_virtual_service_output_virtual_service(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VirtualServiceData>, crate::error::UpdateVirtualServiceError> {
    (!body.is_empty()).then(||{
        crate::json_deser::deser_structure_crate_model_virtual_service_data_payload(body).map_err(crate::error::UpdateVirtualServiceError::unhandled)
    }).transpose()
}

