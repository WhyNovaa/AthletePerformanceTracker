use utoipa::OpenApi;


#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::handlers::get_performance_by_sport,
        crate::api::handlers::add_performance_by_sport,
        crate::api::handlers::remove_performance_by_sport,
    ),
    components()
)]
pub struct ApiDoc;