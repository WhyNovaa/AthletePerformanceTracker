use std::sync::Arc;
use std::time::Duration;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use serde_json::json;
use tokio::net::TcpListener;
use crate::db::postgres_pool::DBPool;
use crate::api::error::Error;
use crate::models::metrics::biathlon::Biathlon;
use crate::models::metrics::running::Running;
use crate::models::metrics::weight_lifting::WeightLifting;
use crate::models::performance_tracker::PerformanceTracker;
use crate::api::responses::Responses;
use crate::models::service_models::{BiathlonPerformance, RunningPerformance, WeightLiftingPerformance};
use crate::models::sportsman::Sportsman;
use crate::service::core::Url;
use crate::traits::traits::{Metric, Pool, SportPerformance};


pub async fn retry_to_bind(url: &Url) -> Result<TcpListener, ()> {
    for _ in 0..5 {
        log::info!("Trying to bind service at {}", url);
        match TcpListener::bind(url.to_string()).await {
            Ok(listener) => {
                log::info!("Tcp listener bound successfully");
                return Ok(listener);
            }
            Err(e) => {
                log::error!("Tcp listener bind error: {}\n Retrying...", e);
            }
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    Err(())
}

#[utoipa::path(
    method(get),
    path = "/{sport}/{name}",
    params(
        ("sport" = String, Path, description = "Вид спорта (running, biathlon, weight_lifting)"),
        ("name" = String, Path, description = "Имя спортсмена")
    ),
    responses(
        (status = 200, description = "Успешный ответ", body = serde_json::Value, examples(
            ("running_example" = (summary = "Running example", value = json!({
                "distance": 999.9,
                "speed": 123.2
            }))),
            ("biathlon_example" = (summary = "Biathlon example", value = json!({
                "accuracy": 18.9,
                "distance": 20.3,
                "speed": 888
            }))),
            ("weight_lifting_example" = (summary = "Weight lifting example", value = json!({
                "weight": 120,
                "lifted_weight": 460
            })))
        )),
        (status = 400, description = "Плохой запрос", body = serde_json::Value, example = json!({ "message": "Name too long" })),
        (status = 404, description = "Не найдено", body = serde_json::Value, example = json!({ "message": "Performance not found" }))
    )
)]
pub async fn get_performance_by_sport(
    Extension(tracker): Extension<Arc<PerformanceTracker>>,
    Path((sport, name)): Path<(String, String)>,
) -> impl IntoResponse {
    match sport.as_str() {
        "running" => get_performance::<Running>(Extension(tracker), Path(name))
            .await
            .into_response(),
        "biathlon" => get_performance::<Biathlon>(Extension(tracker), Path(name))
            .await
            .into_response(),
        "weight_lifting" => get_performance::<WeightLifting>(Extension(tracker), Path(name))
            .await
            .into_response(),
        _ => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Invalid sport type" })),
        )
            .into_response(),
    }
}

#[utoipa::path(
    method(post),
    path = "/{sport}/{name}",
    params(
        ("sport" = String, Path, description = "Вид спорта (running, biathlon, weight_lifting)"),
        ("name" = String, Path, description = "Имя спортсмена")
    ),
    request_body(
        content = serde_json::Value,
        examples(
        ("running_example" = (summary = "Running example", value = json!({
            "distance": 999.9,
            "speed": 123.2
        }))),
        ("biathlon_example" = (summary = "Biathlon example", value = json!({
            "accuracy": 18.9,
            "distance": 20.3,
            "speed": 888
        }))),
        ("weight_lifting_example" = (summary = "Weight lifting example", value = json!({
            "weight": 120,
            "lifted_weight": 460
        })))
        )
    ),
    responses(
        (status = 200, description = "Успешный ответ", body = serde_json::Value, example = json!({"message": "Performance added successfully"})),
        (status = 400, description = "Плохой запрос", body = serde_json::Value, example = json!({ "message": "Invalid sport type or malformed request" })),
        (status = 404, description = "Не найдено", body = serde_json::Value, example = json!({ "message": "Performance not found" })),
        (status = 500, description = "Ошибка сервера", body = serde_json::Value, example = json!({ "message": "Something went wrong" }))
    )
)]

pub async fn add_performance_by_sport(
    Extension((tracker, pool)): Extension<(Arc<PerformanceTracker>, Arc<DBPool>)>,
    Path((sport, name)): Path<(String, String)>,
    body: Json<serde_json::Value>,
) -> impl IntoResponse {
    match sport.as_str() {
        "running" => match serde_json::from_value::<RunningPerformance>(body.0) {
            Ok(performance) => add_performance::<Running, RunningPerformance>(
                Extension((tracker, pool)),
                Path(name),
                Json(performance),
            )
                .await
                .into_response(),
            Err(_) => Responses::InvalidPerformanceFormat("RunningPerformance").into_response(),
        },
        "biathlon" => match serde_json::from_value::<BiathlonPerformance>(body.0) {
            Ok(performance) => add_performance::<Biathlon, BiathlonPerformance>(
                Extension((tracker, pool)),
                Path(name),
                Json(performance),
            )
                .await
                .into_response(),
            Err(_) => Responses::InvalidPerformanceFormat("BiathlonPerformance").into_response(),
        },
        "weight_lifting" => match serde_json::from_value::<WeightLiftingPerformance>(body.0) {
            Ok(performance) => add_performance::<WeightLifting, WeightLiftingPerformance>(
                Extension((tracker, pool)),
                Path(name),
                Json(performance),
            )
                .await
                .into_response(),
            Err(_) => {
                Responses::InvalidPerformanceFormat("WeightLiftingPerformance").into_response()
            }
        },
        _ => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Invalid sport type" })),
        )
            .into_response(),
    }
}

#[utoipa::path(
    method(delete),
    path = "/{sport}/{name}",
    params(
        ("sport" = String, Path, description = "Вид спорта (running, biathlon, weight_lifting)"),
        ("name" = String, Path, description = "Имя спортсмена")
    ),
    responses(
        (status = 200, description = "Успешный ответ", body = serde_json::Value, example = json!({ "message": "Performance removed successfully" })),
        (status = 400, description = "Плохой запрос", body = serde_json::Value, example = json!({ "message": "Name too long" })),
        (status = 404, description = "Не найдено", body = serde_json::Value, example = json!({ "message": "Performance not found" })),
        (status = 500, description = "Ошибка сервера", body = serde_json::Value, example = json!({ "message": "Something went wrong" }))
    )
)]
pub async fn remove_performance_by_sport(
    Extension((tracker, pool)): Extension<(Arc<PerformanceTracker>, Arc<DBPool>)>,
    Path((sport, name)): Path<(String, String)>,
) -> impl IntoResponse {
    match sport.as_str() {
        "running" => remove_performance::<Running>(Extension((tracker, pool)), Path(name))
            .await
            .into_response(),
        "biathlon" => remove_performance::<Biathlon>(Extension((tracker, pool)), Path(name))
            .await
            .into_response(),
        "weight_lifting" => {
            remove_performance::<WeightLifting>(Extension((tracker, pool)), Path(name))
                .await
                .into_response()
        }
        _ => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "message": "Invalid sport type" })),
        )
            .into_response(),
    }
}

async fn get_performance<T: Metric + Clone>(
    Extension(tracker): Extension<Arc<PerformanceTracker>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let sportsman = match Sportsman::new(name) {
        Ok(s) => s,
        Err(e) => return e.into_response(),
    };

    match tracker.get_performance::<T>(&sportsman).await {
        Ok(performance) => {
            log::info!("Performance: {:?}", performance);
            performance.into_response()
        }
        Err(e) => {
            log::info!("{}", e);
            Responses::Errors(e).into_response()
        }
    }
}

async fn add_performance<T, P>(
    Extension((tracker, pool)): Extension<(Arc<PerformanceTracker>, Arc<DBPool>)>,
    Path(name): Path<String>,
    Json(performance): Json<P>,
) -> impl IntoResponse
where
    T: Metric,
    P: Into<T>,
{
    let sportsman = match Sportsman::new(name) {
        Ok(s) => s,
        Err(e) => return e.into_response(),
    };
    let metric: T = performance.into();
    let response_name = metric.response_name();

    if let Err(e) = pool.add_sportsman(&sportsman).await {
        log::error!("Error while saving sportsman: {e}");
        return Responses::Errors(Error::SaveError).into_response();
    }

    if let Err(e) = pool.add_performance(&sportsman, metric.clone_box()).await {
        log::error!("Error while saving performance{e}");
        return Responses::Errors(Error::SaveError).into_response();
    }

    tracker.add_performance(sportsman, metric.clone_box()).await;
    log::info!("Performance was added successfully");

    Responses::PerformanceAdded(response_name).into_response()
}

async fn remove_performance<T: Metric>(
    Extension((tracker, pool)): Extension<(Arc<PerformanceTracker>, Arc<DBPool>)>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let sportsman = match Sportsman::new(name) {
        Ok(s) => s,
        Err(e) => return e.into_response(),
    };

    match pool.remove_performance::<T>(&sportsman).await {
        Ok(removed) => {
            if !removed {
                return Responses::PerformanceNotFound.into_response();
            }
        }
        Err(e) => {
            return match e {
                sqlx::Error::RowNotFound => Responses::SportsmanNotFound.into_response(),
                _ => {
                    log::error!("Error while removing performance: {e}");
                    Responses::Errors(Error::RemoveError).into_response()
                }
            }
        }
    }

    match tracker.remove_performance::<T>(sportsman).await {
        Ok(_) => {
            log::info!("Performance was removed successfully");
            Responses::PerformanceRemoved.into_response()
        }
        Err(e) => {
            log::info!("{}", e);
            Responses::Errors(e).into_response()
        }
    }
}