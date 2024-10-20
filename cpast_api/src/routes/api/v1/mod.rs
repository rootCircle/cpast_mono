use utoipa::OpenApi;

pub(crate) mod evaluate;
pub(crate) mod share;

#[derive(OpenApi)]
#[openapi(paths(
    crate::routes::api::v1::share::post::post_share_code,
    crate::routes::api::v1::share::get::get_share_code
))]
pub(crate) struct EvaluateApiv1;
