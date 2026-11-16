use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderValue, Method, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{delete, get, patch, post},
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::models::{
    AutomationRule, CreateAutomationRequest, CreateDeviceRequest, Device, ErrorResponse,
    HealthResponse, OverviewResponse, UpdateDeviceRequest,
};
use crate::state::SharedState;

/// Builds the Axum router with all LinkHaus API endpoints, CORS, and tracing middleware.
pub fn build_router(state: SharedState) -> Router {
    let allowed_origins = [
        HeaderValue::from_static("http://localhost:4321"),
        HeaderValue::from_static("http://127.0.0.1:4321"),
    ];

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, header::ACCEPT, header::AUTHORIZATION]);

    Router::new()
        .route("/api/health", get(health_handler))
        .route("/api/overview", get(overview_handler))
        .route(
            "/api/devices",
            get(devices_handler).post(create_device_handler),
        )
        .route("/api/devices/{id}/toggle", post(toggle_device_handler))
        .route(
            "/api/devices/{id}",
            patch(patch_device_handler).delete(delete_device_handler),
        )
        .route("/api/scenes/{id}/activate", post(activate_scene_handler))
        .route(
            "/api/automations",
            get(automations_handler).post(create_automation_handler),
        )
        .route(
            "/api/automations/{id}/toggle",
            patch(toggle_automation_handler),
        )
        .route("/api/automations/{id}/run", post(run_automation_handler))
        .route("/api/automations/{id}", delete(delete_automation_handler))
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}

fn json_error(status: StatusCode, message: impl Into<String>) -> Response {
    (
        status,
        Json(ErrorResponse {
            error: message.into(),
        }),
    )
        .into_response()
}

/// `GET /api/health`
pub async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        service: "linkhaus-core".to_string(),
        company: "Helios Smart Systems".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// `GET /api/overview`
pub async fn overview_handler(State(state): State<SharedState>) -> Json<OverviewResponse> {
    let guard = state.read().await;
    Json(guard.overview())
}

/// `GET /api/devices`
pub async fn devices_handler(State(state): State<SharedState>) -> Json<Vec<Device>> {
    let guard = state.read().await;
    Json(guard.devices.clone())
}

/// `POST /api/devices`
pub async fn create_device_handler(
    State(state): State<SharedState>,
    Json(payload): Json<CreateDeviceRequest>,
) -> Response {
    let mut guard = state.write().await;
    match guard.create_device(payload) {
        Ok(device) => (StatusCode::CREATED, Json(device)).into_response(),
        Err(msg) => json_error(StatusCode::BAD_REQUEST, msg),
    }
}

/// `DELETE /api/devices/{id}`
pub async fn delete_device_handler(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Response {
    let mut guard = state.write().await;
    if guard.delete_device(&id) {
        StatusCode::NO_CONTENT.into_response()
    } else {
        json_error(StatusCode::NOT_FOUND, format!("Device '{id}' not found"))
    }
}

/// `POST /api/devices/{id}/toggle`
pub async fn toggle_device_handler(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Response {
    let mut guard = state.write().await;
    match guard.toggle_device(&id) {
        Some(device) => (StatusCode::OK, Json(device)).into_response(),
        None => json_error(StatusCode::NOT_FOUND, format!("Device '{id}' not found")),
    }
}

/// `PATCH /api/devices/{id}`
pub async fn patch_device_handler(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateDeviceRequest>,
) -> Response {
    let mut guard = state.write().await;
    match guard.update_device(&id, payload) {
        Ok(Some(device)) => (StatusCode::OK, Json(device)).into_response(),
        Ok(None) => json_error(StatusCode::NOT_FOUND, format!("Device '{id}' not found")),
        Err(msg) => json_error(StatusCode::BAD_REQUEST, msg),
    }
}

/// `POST /api/scenes/{id}/activate`
pub async fn activate_scene_handler(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Response {
    let mut guard = state.write().await;
    match guard.activate_scene(&id) {
        Some(overview) => (StatusCode::OK, Json(overview)).into_response(),
        None => json_error(StatusCode::NOT_FOUND, format!("Scene '{id}' not found")),
    }
}

/// `GET /api/automations`
pub async fn automations_handler(State(state): State<SharedState>) -> Json<Vec<AutomationRule>> {
    let guard = state.read().await;
    Json(guard.automations.clone())
}

/// `POST /api/automations`
pub async fn create_automation_handler(
    State(state): State<SharedState>,
    Json(payload): Json<CreateAutomationRequest>,
) -> Response {
    let mut guard = state.write().await;
    match guard.create_automation(payload) {
        Ok(rule) => (StatusCode::CREATED, Json(rule)).into_response(),
        Err(msg) => json_error(StatusCode::BAD_REQUEST, msg),
    }
}

/// `PATCH /api/automations/{id}/toggle`
pub async fn toggle_automation_handler(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Response {
    let mut guard = state.write().await;
    match guard.toggle_automation(&id) {
        Some(rule) => (StatusCode::OK, Json(rule)).into_response(),
        None => json_error(
            StatusCode::NOT_FOUND,
            format!("Automation rule '{id}' not found"),
        ),
    }
}

/// `POST /api/automations/{id}/run`
pub async fn run_automation_handler(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Response {
    let mut guard = state.write().await;
    match guard.run_automation(&id) {
        Some(overview) => (StatusCode::OK, Json(overview)).into_response(),
        None => json_error(
            StatusCode::NOT_FOUND,
            format!("Automation rule '{id}' not found"),
        ),
    }
}

/// `DELETE /api/automations/{id}`
pub async fn delete_automation_handler(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Response {
    let mut guard = state.write().await;
    if guard.delete_automation(&id) {
        StatusCode::NO_CONTENT.into_response()
    } else {
        json_error(
            StatusCode::NOT_FOUND,
            format!("Automation rule '{id}' not found"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::DeviceKind;
    use crate::state::AppState;
    use axum::body::Body;
    use axum::http::Request;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn read_json<T: serde::de::DeserializeOwned>(response: Response) -> T {
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test]
    async fn health_returns_expected_payload() {
        let app = build_router(AppState::new_shared());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body: HealthResponse = read_json(response).await;
        assert_eq!(body.status, "ok");
        assert_eq!(body.service, "linkhaus-core");
        assert_eq!(body.company, "Helios Smart Systems");
        assert_eq!(body.version, "0.1.0");
    }

    #[tokio::test]
    async fn overview_returns_seeded_rooms_devices_scenes_and_automations() {
        let app = build_router(AppState::new_shared());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/overview")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body: OverviewResponse = read_json(response).await;
        assert_eq!(body.company, "Helios Smart Systems");
        assert_eq!(body.product, "LinkHaus");
        assert_eq!(body.rooms.len(), 3);
        assert_eq!(body.scenes.len(), 3);
        assert_eq!(body.automations.len(), 1);
        assert_eq!(body.metrics.total_devices, body.devices.len());
        assert!(body.metrics.active_devices > 0);
        assert!(body.metrics.power_watts > 0);

        let buttons: Vec<_> = body
            .devices
            .iter()
            .filter(|d| d.kind == DeviceKind::Switch)
            .collect();
        assert_eq!(buttons.len(), 4);
    }

    #[tokio::test]
    async fn toggle_device_flips_state_and_updates_power() {
        let state = AppState::new_shared();
        let app = build_router(state);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/devices/dev-living-arc/toggle")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let off_device: Device = read_json(response).await;
        assert!(!off_device.is_on);
        assert_eq!(off_device.power_watts, 0);

        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/devices/dev-living-arc/toggle")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let on_device: Device = read_json(response).await;
        assert!(on_device.is_on);
        assert!(on_device.power_watts > 0);
    }

    #[tokio::test]
    async fn toggle_missing_device_returns_404() {
        let app = build_router(AppState::new_shared());
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/devices/nonexistent/toggle")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn create_and_delete_device_lifecycle() {
        let state = AppState::new_shared();
        let app = build_router(state);

        let create_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/devices")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{
                            "name": "Atelier Accent Strip",
                            "room_id": "room-studio",
                            "kind": "lighting",
                            "is_on": true,
                            "level": 75,
                            "color_temp_k": 4200
                        }"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(create_resp.status(), StatusCode::CREATED);
        let created: Device = read_json(create_resp).await;
        assert_eq!(created.name, "Atelier Accent Strip");
        assert_eq!(created.kind, DeviceKind::Lighting);
        assert_eq!(created.level, Some(75));
        assert_eq!(created.color_temp_k, Some(4200));
        assert!(created.is_on);

        let del_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::DELETE)
                    .uri(format!("/api/devices/{}", created.id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(del_resp.status(), StatusCode::NO_CONTENT);

        let del_again = app
            .oneshot(
                Request::builder()
                    .method(Method::DELETE)
                    .uri(format!("/api/devices/{}", created.id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(del_again.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn patch_device_updates_climate_lighting_and_rejects_invalid() {
        let state = AppState::new_shared();
        let app = build_router(state);

        let climate_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/api/devices/dev-living-climate")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{
                            "temperature_c": 23.5,
                            "climate_mode": "heat",
                            "fan_speed": "high"
                        }"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(climate_resp.status(), StatusCode::OK);
        let climate: Device = read_json(climate_resp).await;
        assert_eq!(climate.temperature_c, Some(23.5));
        assert_eq!(climate.climate_mode.as_deref(), Some("heat"));
        assert_eq!(climate.fan_speed.as_deref(), Some("high"));

        let light_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/api/devices/dev-studio-task")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{"level": 50, "is_on": true, "color_temp_k": 4000}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(light_resp.status(), StatusCode::OK);
        let light: Device = read_json(light_resp).await;
        assert_eq!(light.level, Some(50));
        assert_eq!(light.color_temp_k, Some(4000));
        assert_eq!(light.power_watts, 14);

        let bad_resp = app
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri("/api/devices/dev-living-climate")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"temperature_c": 45.0}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(bad_resp.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn button_1_cascade_automation_turns_on_buttons_2_3_4() {
        let state = AppState::new_shared();
        let app = build_router(state);

        let before_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/api/overview")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let before: OverviewResponse = read_json(before_resp).await;

        let toggle_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/devices/dev-button-1/toggle")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(toggle_resp.status(), StatusCode::OK);

        let after_resp = app
            .oneshot(
                Request::builder()
                    .uri("/api/overview")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let after: OverviewResponse = read_json(after_resp).await;

        for id in ["dev-button-1", "dev-button-2", "dev-button-3", "dev-button-4"] {
            let dev = after.devices.iter().find(|d| d.id == id).unwrap();
            assert!(dev.is_on, "{id} should be turned ON by cascade");
        }
        assert_eq!(
            after.metrics.active_devices,
            before.metrics.active_devices + 4
        );
        assert!(after.metrics.power_watts > before.metrics.power_watts);
    }

    #[tokio::test]
    async fn custom_automation_create_toggle_run_delete_and_cycle_guard() {
        let state = AppState::new_shared();
        let app = build_router(state);

        // Create two mutually-triggering rules to verify cycle guard doesn't infinite-loop
        let rule_a_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/automations")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{
                            "name": "Cycle A to B",
                            "trigger_device_id": "dev-living-wash",
                            "trigger_condition": "turns_on",
                            "actions": [
                                { "target_device_id": "dev-studio-task", "action": "toggle" }
                            ]
                        }"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(rule_a_resp.status(), StatusCode::CREATED);
        let rule_a: AutomationRule = read_json(rule_a_resp).await;

        let rule_b_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/automations")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(
                        r#"{
                            "name": "Cycle B to A",
                            "trigger_device_id": "dev-studio-task",
                            "trigger_condition": "turns_off",
                            "actions": [
                                { "target_device_id": "dev-living-wash", "action": "turn_off" }
                            ]
                        }"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(rule_b_resp.status(), StatusCode::CREATED);

        // Trigger rule A via manual run
        let run_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(format!("/api/automations/{}/run", rule_a.id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(run_resp.status(), StatusCode::OK);

        // Toggle rule A off
        let toggle_rule_resp = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::PATCH)
                    .uri(format!("/api/automations/{}/toggle", rule_a.id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(toggle_rule_resp.status(), StatusCode::OK);
        let toggled: AutomationRule = read_json(toggle_rule_resp).await;
        assert!(!toggled.enabled);

        // Delete rule A
        let del_resp = app
            .oneshot(
                Request::builder()
                    .method(Method::DELETE)
                    .uri(format!("/api/automations/{}", rule_a.id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(del_resp.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn activate_scene_updates_devices_and_marks_scene_active() {
        let app = build_router(AppState::new_shared());
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/api/scenes/scene-away-guard/activate")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let overview: OverviewResponse = read_json(response).await;

        let active_scene = overview
            .scenes
            .iter()
            .find(|s| s.id == "scene-away-guard")
            .unwrap();
        assert!(active_scene.is_active);
        assert_eq!(overview.scenes.iter().filter(|s| s.is_active).count(), 1);

        let living_arc = overview
            .devices
            .iter()
            .find(|d| d.id == "dev-living-arc")
            .unwrap();
        assert!(!living_arc.is_on);

        let lock = overview
            .devices
            .iter()
            .find(|d| d.id == "dev-entry-lock")
            .unwrap();
        assert!(lock.is_on);
    }
}
