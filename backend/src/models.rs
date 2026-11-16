use serde::{Deserialize, Serialize};

/// Health check response returned by `GET /api/health`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub company: String,
    pub version: String,
}

/// Aggregated telemetry and state metrics for the LinkHaus overview.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OverviewMetrics {
    pub active_devices: usize,
    pub total_devices: usize,
    pub power_watts: u32,
    pub uptime_secs: u64,
}

/// Full system snapshot returned by `GET /api/overview`, `POST /api/scenes/{id}/activate`,
/// and `POST /api/automations/{id}/run`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OverviewResponse {
    pub company: String,
    pub product: String,
    pub metrics: OverviewMetrics,
    pub rooms: Vec<Room>,
    pub devices: Vec<Device>,
    pub scenes: Vec<Scene>,
    pub automations: Vec<AutomationRule>,
}

/// Supported smart home device categories in LinkHaus.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeviceKind {
    Lighting,
    Climate,
    Switch,
    Security,
    Energy,
}

/// Represents an individual smart home node managed by LinkHaus.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub room_id: String,
    pub kind: DeviceKind,
    pub is_on: bool,
    pub level: Option<u8>,
    pub temperature_c: Option<f32>,
    pub climate_mode: Option<String>,
    pub fan_speed: Option<String>,
    pub color_temp_k: Option<u16>,
    pub button_mode: Option<String>,
    pub power_watts: u32,
    #[serde(skip_serializing, skip_deserializing)]
    pub rated_watts: u32,
    pub status_label: String,
}

/// Architectural zone within the residence.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub floor: String,
    pub temperature_c: f32,
}

/// Preconfigured multi-device automation preset.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Scene {
    pub id: String,
    pub name: String,
    pub description: String,
    pub is_active: bool,
}

/// Payload for `POST /api/devices`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateDeviceRequest {
    pub name: String,
    pub room_id: String,
    pub kind: DeviceKind,
    #[serde(default)]
    pub is_on: Option<bool>,
    #[serde(default)]
    pub level: Option<u8>,
    #[serde(default)]
    pub temperature_c: Option<f32>,
    #[serde(default)]
    pub climate_mode: Option<String>,
    #[serde(default)]
    pub fan_speed: Option<String>,
    #[serde(default)]
    pub color_temp_k: Option<u16>,
    #[serde(default)]
    pub button_mode: Option<String>,
}

/// Partial update payload for `PATCH /api/devices/{id}`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct UpdateDeviceRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub is_on: Option<bool>,
    #[serde(default)]
    pub level: Option<u8>,
    #[serde(default)]
    pub temperature_c: Option<f32>,
    #[serde(default)]
    pub climate_mode: Option<String>,
    #[serde(default)]
    pub fan_speed: Option<String>,
    #[serde(default)]
    pub color_temp_k: Option<u16>,
    #[serde(default)]
    pub button_mode: Option<String>,
}

/// A single action executed when an automation rule fires.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RuleAction {
    pub target_device_id: String,
    pub action: String,
    #[serde(default)]
    pub value: Option<f32>,
}

/// Autonomous IF/THEN automation rule.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AutomationRule {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub trigger_device_id: String,
    pub trigger_condition: String,
    #[serde(default)]
    pub trigger_value: Option<f32>,
    pub actions: Vec<RuleAction>,
}

/// Payload for `POST /api/automations`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAutomationRequest {
    pub name: String,
    #[serde(default)]
    pub enabled: Option<bool>,
    pub trigger_device_id: String,
    pub trigger_condition: String,
    #[serde(default)]
    pub trigger_value: Option<f32>,
    pub actions: Vec<RuleAction>,
}

/// Standard JSON error envelope.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorResponse {
    pub error: String,
}
