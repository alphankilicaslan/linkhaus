use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

use crate::models::{
    AutomationRule, CreateAutomationRequest, CreateDeviceRequest, Device, DeviceKind,
    OverviewMetrics, OverviewResponse, Room, RuleAction, Scene, UpdateDeviceRequest,
};

/// Shared thread-safe application state handle.
pub type SharedState = Arc<RwLock<AppState>>;

const MAX_CASCADE_DEPTH: usize = 16;

/// Internal preset target for a single device when activating a scene.
#[derive(Debug, Clone)]
pub struct SceneDevicePreset {
    pub device_id: &'static str,
    pub is_on: bool,
    pub level: Option<u8>,
}

/// In-memory state container for LinkHaus rooms, devices, scenes, and automation rules.
#[derive(Debug)]
pub struct AppState {
    started_at: Instant,
    next_device_seq: u64,
    next_rule_seq: u64,
    pub rooms: Vec<Room>,
    pub devices: Vec<Device>,
    pub scenes: Vec<Scene>,
    pub automations: Vec<AutomationRule>,
    scene_presets: HashMap<String, Vec<SceneDevicePreset>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    /// Creates a new `AppState` seeded with realistic LinkHaus architectural smart home data.
    pub fn new() -> Self {
        let rooms = vec![
            Room {
                id: "room-living".to_string(),
                name: "Living Room".to_string(),
                floor: "Ground Floor".to_string(),
                temperature_c: 21.5,
            },
            Room {
                id: "room-studio".to_string(),
                name: "Studio".to_string(),
                floor: "Upper Level".to_string(),
                temperature_c: 20.8,
            },
            Room {
                id: "room-entryway".to_string(),
                name: "Entryway".to_string(),
                floor: "Ground Floor".to_string(),
                temperature_c: 19.6,
            },
        ];

        let mut devices = vec![
            Device {
                id: "dev-living-arc".to_string(),
                name: "Linear Arc Ceiling".to_string(),
                room_id: "room-living".to_string(),
                kind: DeviceKind::Lighting,
                is_on: true,
                level: Some(68),
                temperature_c: None,
                climate_mode: None,
                fan_speed: None,
                color_temp_k: Some(3200),
                button_mode: None,
                power_watts: 0,
                rated_watts: 48,
                status_label: String::new(),
            },
            Device {
                id: "dev-living-wash".to_string(),
                name: "Monolith Floor Wash".to_string(),
                room_id: "room-living".to_string(),
                kind: DeviceKind::Lighting,
                is_on: false,
                level: Some(30),
                temperature_c: None,
                climate_mode: None,
                fan_speed: None,
                color_temp_k: Some(2700),
                button_mode: None,
                power_watts: 0,
                rated_watts: 32,
                status_label: String::new(),
            },
            Device {
                id: "dev-living-climate".to_string(),
                name: "Helios Air Core".to_string(),
                room_id: "room-living".to_string(),
                kind: DeviceKind::Climate,
                is_on: true,
                level: Some(45),
                temperature_c: Some(22.0),
                climate_mode: Some("cool".to_string()),
                fan_speed: Some("auto".to_string()),
                color_temp_k: None,
                button_mode: None,
                power_watts: 0,
                rated_watts: 320,
                status_label: String::new(),
            },
            Device {
                id: "dev-studio-task".to_string(),
                name: "Studio Task Bar".to_string(),
                room_id: "room-studio".to_string(),
                kind: DeviceKind::Lighting,
                is_on: true,
                level: Some(85),
                temperature_c: None,
                climate_mode: None,
                fan_speed: None,
                color_temp_k: Some(5000),
                button_mode: None,
                power_watts: 0,
                rated_watts: 28,
                status_label: String::new(),
            },
            Device {
                id: "dev-studio-climate".to_string(),
                name: "Studio Thermal Loop".to_string(),
                room_id: "room-studio".to_string(),
                kind: DeviceKind::Climate,
                is_on: true,
                level: Some(50),
                temperature_c: Some(21.0),
                climate_mode: Some("auto".to_string()),
                fan_speed: Some("low".to_string()),
                color_temp_k: None,
                button_mode: None,
                power_watts: 0,
                rated_watts: 180,
                status_label: String::new(),
            },
            Device {
                id: "dev-studio-node".to_string(),
                name: "Helios Energy Node".to_string(),
                room_id: "room-studio".to_string(),
                kind: DeviceKind::Energy,
                is_on: true,
                level: Some(74),
                temperature_c: None,
                climate_mode: None,
                fan_speed: None,
                color_temp_k: None,
                button_mode: None,
                power_watts: 0,
                rated_watts: 120,
                status_label: String::new(),
            },
            Device {
                id: "dev-entry-sconce".to_string(),
                name: "Threshold Sconce".to_string(),
                room_id: "room-entryway".to_string(),
                kind: DeviceKind::Lighting,
                is_on: true,
                level: Some(40),
                temperature_c: None,
                climate_mode: None,
                fan_speed: None,
                color_temp_k: Some(3000),
                button_mode: None,
                power_watts: 0,
                rated_watts: 18,
                status_label: String::new(),
            },
            Device {
                id: "dev-entry-lock".to_string(),
                name: "Architectural Deadbolt".to_string(),
                room_id: "room-entryway".to_string(),
                kind: DeviceKind::Security,
                is_on: true,
                level: None,
                temperature_c: None,
                climate_mode: None,
                fan_speed: None,
                color_temp_k: None,
                button_mode: None,
                power_watts: 0,
                rated_watts: 6,
                status_label: String::new(),
            },
            Device {
                id: "dev-entry-perimeter".to_string(),
                name: "Perimeter LiDAR Guard".to_string(),
                room_id: "room-entryway".to_string(),
                kind: DeviceKind::Security,
                is_on: true,
                level: None,
                temperature_c: None,
                climate_mode: None,
                fan_speed: None,
                color_temp_k: None,
                button_mode: None,
                power_watts: 0,
                rated_watts: 14,
                status_label: String::new(),
            },
            Device {
                id: "dev-button-1".to_string(),
                name: "Button 1".to_string(),
                room_id: "room-living".to_string(),
                kind: DeviceKind::Switch,
                is_on: false,
                level: None,
                temperature_c: None,
                climate_mode: None,
                fan_speed: None,
                color_temp_k: None,
                button_mode: Some("toggle".to_string()),
                power_watts: 0,
                rated_watts: 2,
                status_label: String::new(),
            },
            Device {
                id: "dev-button-2".to_string(),
                name: "Button 2".to_string(),
                room_id: "room-living".to_string(),
                kind: DeviceKind::Switch,
                is_on: false,
                level: None,
                temperature_c: None,
                climate_mode: None,
                fan_speed: None,
                color_temp_k: None,
                button_mode: Some("toggle".to_string()),
                power_watts: 0,
                rated_watts: 2,
                status_label: String::new(),
            },
            Device {
                id: "dev-button-3".to_string(),
                name: "Button 3".to_string(),
                room_id: "room-studio".to_string(),
                kind: DeviceKind::Switch,
                is_on: false,
                level: None,
                temperature_c: None,
                climate_mode: None,
                fan_speed: None,
                color_temp_k: None,
                button_mode: Some("toggle".to_string()),
                power_watts: 0,
                rated_watts: 2,
                status_label: String::new(),
            },
            Device {
                id: "dev-button-4".to_string(),
                name: "Button 4".to_string(),
                room_id: "room-entryway".to_string(),
                kind: DeviceKind::Switch,
                is_on: false,
                level: None,
                temperature_c: None,
                climate_mode: None,
                fan_speed: None,
                color_temp_k: None,
                button_mode: Some("toggle".to_string()),
                power_watts: 0,
                rated_watts: 2,
                status_label: String::new(),
            },
        ];

        for device in &mut devices {
            Self::recompute_device(device);
        }

        let scenes = vec![
            Scene {
                id: "scene-monochrome-evening".to_string(),
                name: "Monochrome Evening".to_string(),
                description: "Low-glare architectural lighting with quiet thermal balance."
                    .to_string(),
                is_active: false,
            },
            Scene {
                id: "scene-focus-mode".to_string(),
                name: "Focus Mode".to_string(),
                description: "High-clarity studio illumination and active energy optimization."
                    .to_string(),
                is_active: false,
            },
            Scene {
                id: "scene-away-guard".to_string(),
                name: "Away Guard".to_string(),
                description: "All lighting off, climate standby, perimeter security armed."
                    .to_string(),
                is_active: false,
            },
        ];

        let automations = vec![AutomationRule {
            id: "rule-button1-cascade".to_string(),
            name: "Button 1 Cascade".to_string(),
            enabled: true,
            trigger_device_id: "dev-button-1".to_string(),
            trigger_condition: "turns_on".to_string(),
            trigger_value: None,
            actions: vec![
                RuleAction {
                    target_device_id: "dev-button-2".to_string(),
                    action: "turn_on".to_string(),
                    value: None,
                },
                RuleAction {
                    target_device_id: "dev-button-3".to_string(),
                    action: "turn_on".to_string(),
                    value: None,
                },
                RuleAction {
                    target_device_id: "dev-button-4".to_string(),
                    action: "turn_on".to_string(),
                    value: None,
                },
            ],
        }];

        let mut scene_presets = HashMap::new();
        scene_presets.insert(
            "scene-monochrome-evening".to_string(),
            vec![
                SceneDevicePreset {
                    device_id: "dev-living-arc",
                    is_on: true,
                    level: Some(32),
                },
                SceneDevicePreset {
                    device_id: "dev-living-wash",
                    is_on: true,
                    level: Some(25),
                },
                SceneDevicePreset {
                    device_id: "dev-living-climate",
                    is_on: true,
                    level: Some(35),
                },
                SceneDevicePreset {
                    device_id: "dev-studio-task",
                    is_on: false,
                    level: Some(20),
                },
                SceneDevicePreset {
                    device_id: "dev-entry-sconce",
                    is_on: true,
                    level: Some(25),
                },
                SceneDevicePreset {
                    device_id: "dev-entry-lock",
                    is_on: true,
                    level: None,
                },
            ],
        );
        scene_presets.insert(
            "scene-focus-mode".to_string(),
            vec![
                SceneDevicePreset {
                    device_id: "dev-living-arc",
                    is_on: false,
                    level: Some(40),
                },
                SceneDevicePreset {
                    device_id: "dev-living-wash",
                    is_on: false,
                    level: Some(20),
                },
                SceneDevicePreset {
                    device_id: "dev-studio-task",
                    is_on: true,
                    level: Some(95),
                },
                SceneDevicePreset {
                    device_id: "dev-studio-climate",
                    is_on: true,
                    level: Some(60),
                },
                SceneDevicePreset {
                    device_id: "dev-studio-node",
                    is_on: true,
                    level: Some(88),
                },
                SceneDevicePreset {
                    device_id: "dev-entry-sconce",
                    is_on: false,
                    level: Some(30),
                },
            ],
        );
        scene_presets.insert(
            "scene-away-guard".to_string(),
            vec![
                SceneDevicePreset {
                    device_id: "dev-living-arc",
                    is_on: false,
                    level: Some(0),
                },
                SceneDevicePreset {
                    device_id: "dev-living-wash",
                    is_on: false,
                    level: Some(0),
                },
                SceneDevicePreset {
                    device_id: "dev-living-climate",
                    is_on: false,
                    level: Some(15),
                },
                SceneDevicePreset {
                    device_id: "dev-studio-task",
                    is_on: false,
                    level: Some(0),
                },
                SceneDevicePreset {
                    device_id: "dev-studio-climate",
                    is_on: false,
                    level: Some(15),
                },
                SceneDevicePreset {
                    device_id: "dev-entry-sconce",
                    is_on: false,
                    level: Some(0),
                },
                SceneDevicePreset {
                    device_id: "dev-entry-lock",
                    is_on: true,
                    level: None,
                },
                SceneDevicePreset {
                    device_id: "dev-entry-perimeter",
                    is_on: true,
                    level: None,
                },
            ],
        );

        Self {
            started_at: Instant::now(),
            next_device_seq: 1,
            next_rule_seq: 1,
            rooms,
            devices,
            scenes,
            automations,
            scene_presets,
        }
    }

    /// Creates a thread-safe shared state instance.
    pub fn new_shared() -> SharedState {
        Arc::new(RwLock::new(Self::new()))
    }

    /// Recomputes `power_watts` and `status_label` for a device based on its current state.
    pub fn recompute_device(device: &mut Device) {
        if !device.is_on {
            device.power_watts = 0;
            device.status_label = match device.kind {
                DeviceKind::Security => "Disarmed".to_string(),
                DeviceKind::Energy => "Idle".to_string(),
                DeviceKind::Switch => "Off".to_string(),
                DeviceKind::Lighting | DeviceKind::Climate => "Standby".to_string(),
            };
            return;
        }

        let level = device.level.unwrap_or(100);
        device.power_watts = if device.level.is_some() {
            let scaled = (device.rated_watts * u32::from(level) + 50) / 100;
            scaled.max(if level > 0 { 1 } else { 0 })
        } else {
            device.rated_watts
        };

        device.status_label = match device.kind {
            DeviceKind::Lighting => {
                let kelvin = device.color_temp_k.unwrap_or(3000);
                format!("{level}% · {kelvin}K")
            }
            DeviceKind::Climate => {
                let temp = device.temperature_c.unwrap_or(22.0);
                let mode = device.climate_mode.as_deref().unwrap_or("auto");
                format!("{temp:.1}°C · {mode}")
            }
            DeviceKind::Switch => {
                let mode = device.button_mode.as_deref().unwrap_or("toggle");
                format!("Active · {mode}")
            }
            DeviceKind::Security => "Armed · Active".to_string(),
            DeviceKind::Energy => format!("{level}% · Grid Sync"),
        };
    }

    /// Builds the full `OverviewResponse` snapshot from current state.
    pub fn overview(&self) -> OverviewResponse {
        let active_devices = self.devices.iter().filter(|d| d.is_on).count();
        let total_devices = self.devices.len();
        let power_watts = self.devices.iter().map(|d| d.power_watts).sum();
        let uptime_secs = self.started_at.elapsed().as_secs();

        OverviewResponse {
            company: "Helios Smart Systems".to_string(),
            product: "LinkHaus".to_string(),
            metrics: OverviewMetrics {
                active_devices,
                total_devices,
                power_watts,
                uptime_secs,
            },
            rooms: self.rooms.clone(),
            devices: self.devices.clone(),
            scenes: self.scenes.clone(),
            automations: self.automations.clone(),
        }
    }

    fn validate_device_settings(
        level: Option<u8>,
        temperature_c: Option<f32>,
        climate_mode: Option<&str>,
        fan_speed: Option<&str>,
        color_temp_k: Option<u16>,
        button_mode: Option<&str>,
    ) -> Result<(), String> {
        if let Some(lvl) = level
            && lvl > 100
        {
            return Err("level must be between 0 and 100".to_string());
        }
        if let Some(temp) = temperature_c
            && (temp.is_nan() || !(16.0..=30.0).contains(&temp))
        {
            return Err("temperature_c must be between 16.0 and 30.0".to_string());
        }
        if let Some(mode) = climate_mode
            && !matches!(mode, "cool" | "heat" | "auto" | "fan")
        {
            return Err("climate_mode must be one of: cool, heat, auto, fan".to_string());
        }
        if let Some(speed) = fan_speed
            && !matches!(speed, "low" | "medium" | "high" | "auto")
        {
            return Err("fan_speed must be one of: low, medium, high, auto".to_string());
        }
        if let Some(kelvin) = color_temp_k
            && !(2700..=6500).contains(&kelvin)
        {
            return Err("color_temp_k must be between 2700 and 6500".to_string());
        }
        if let Some(mode) = button_mode
            && !matches!(mode, "toggle" | "momentary" | "scene")
        {
            return Err("button_mode must be one of: toggle, momentary, scene".to_string());
        }
        Ok(())
    }

    /// Creates and stores a new device with a unique ID.
    pub fn create_device(&mut self, req: CreateDeviceRequest) -> Result<Device, String> {
        let name = req.name.trim().to_string();
        if name.is_empty() {
            return Err("device name must not be empty".to_string());
        }
        if !self.rooms.iter().any(|r| r.id == req.room_id) {
            return Err(format!("room_id '{}' not found", req.room_id));
        }

        Self::validate_device_settings(
            req.level,
            req.temperature_c,
            req.climate_mode.as_deref(),
            req.fan_speed.as_deref(),
            req.color_temp_k,
            req.button_mode.as_deref(),
        )?;

        let id = format!("dev-custom-{}", self.next_device_seq);
        self.next_device_seq += 1;

        let rated_watts = match req.kind {
            DeviceKind::Lighting => 35,
            DeviceKind::Climate => 240,
            DeviceKind::Switch => 2,
            DeviceKind::Security => 10,
            DeviceKind::Energy => 100,
        };

        let level = req.level.or(match req.kind {
            DeviceKind::Lighting | DeviceKind::Climate | DeviceKind::Energy => Some(50),
            DeviceKind::Switch | DeviceKind::Security => None,
        });

        let mut device = Device {
            id,
            name,
            room_id: req.room_id,
            kind: req.kind,
            is_on: req.is_on.unwrap_or(false),
            level,
            temperature_c: req.temperature_c.or(match req.kind {
                DeviceKind::Climate => Some(22.0),
                _ => None,
            }),
            climate_mode: req.climate_mode.or_else(|| match req.kind {
                DeviceKind::Climate => Some("auto".to_string()),
                _ => None,
            }),
            fan_speed: req.fan_speed.or_else(|| match req.kind {
                DeviceKind::Climate => Some("auto".to_string()),
                _ => None,
            }),
            color_temp_k: req.color_temp_k.or(match req.kind {
                DeviceKind::Lighting => Some(3000),
                _ => None,
            }),
            button_mode: req.button_mode.or_else(|| match req.kind {
                DeviceKind::Switch => Some("toggle".to_string()),
                _ => None,
            }),
            power_watts: 0,
            rated_watts,
            status_label: String::new(),
        };

        Self::recompute_device(&mut device);
        self.devices.push(device.clone());
        Ok(device)
    }

    /// Removes a device by ID, returning `true` if it existed.
    pub fn delete_device(&mut self, id: &str) -> bool {
        let before = self.devices.len();
        self.devices.retain(|d| d.id != id);
        self.devices.len() != before
    }

    /// Toggles `is_on` for a device by ID, updates its power draw, and triggers matching automations.
    pub fn toggle_device(&mut self, id: &str) -> Option<Device> {
        let idx = self.devices.iter().position(|d| d.id == id)?;
        let prev = self.devices[idx].clone();
        self.devices[idx].is_on = !self.devices[idx].is_on;
        Self::recompute_device(&mut self.devices[idx]);
        let curr = self.devices[idx].clone();

        self.run_automation_cascade(prev, curr);
        self.devices.iter().find(|d| d.id == id).cloned()
    }

    /// Partially updates a device's settings and triggers matching automations.
    pub fn update_device(
        &mut self,
        id: &str,
        req: UpdateDeviceRequest,
    ) -> Result<Option<Device>, String> {
        Self::validate_device_settings(
            req.level,
            req.temperature_c,
            req.climate_mode.as_deref(),
            req.fan_speed.as_deref(),
            req.color_temp_k,
            req.button_mode.as_deref(),
        )?;

        if let Some(ref name) = req.name
            && name.trim().is_empty()
        {
            return Err("device name must not be empty".to_string());
        }

        let Some(idx) = self.devices.iter().position(|d| d.id == id) else {
            return Ok(None);
        };

        let prev = self.devices[idx].clone();
        {
            let device = &mut self.devices[idx];
            if let Some(name) = req.name {
                device.name = name.trim().to_string();
            }
            if let Some(is_on) = req.is_on {
                device.is_on = is_on;
            }
            if let Some(level) = req.level {
                device.level = Some(level);
            }
            if let Some(temp) = req.temperature_c {
                device.temperature_c = Some(temp);
            }
            if let Some(mode) = req.climate_mode {
                device.climate_mode = Some(mode);
            }
            if let Some(speed) = req.fan_speed {
                device.fan_speed = Some(speed);
            }
            if let Some(kelvin) = req.color_temp_k {
                device.color_temp_k = Some(kelvin);
            }
            if let Some(mode) = req.button_mode {
                device.button_mode = Some(mode);
            }
            Self::recompute_device(device);
        }
        let curr = self.devices[idx].clone();

        self.run_automation_cascade(prev, curr);
        Ok(self.devices.iter().find(|d| d.id == id).cloned())
    }

    /// Applies a scene preset to devices and returns the updated `OverviewResponse`.
    pub fn activate_scene(&mut self, id: &str) -> Option<OverviewResponse> {
        let presets = self.scene_presets.get(id)?.clone();

        for scene in &mut self.scenes {
            scene.is_active = scene.id == id;
        }

        for preset in presets {
            if let Some(device) = self
                .devices
                .iter_mut()
                .find(|d| d.id == preset.device_id)
            {
                device.is_on = preset.is_on;
                if let Some(lvl) = preset.level {
                    device.level = Some(lvl);
                }
                Self::recompute_device(device);
            }
        }

        Some(self.overview())
    }

    /// Creates a new IF/THEN automation rule.
    pub fn create_automation(
        &mut self,
        req: CreateAutomationRequest,
    ) -> Result<AutomationRule, String> {
        let name = req.name.trim().to_string();
        if name.is_empty() {
            return Err("automation name must not be empty".to_string());
        }
        if !self.devices.iter().any(|d| d.id == req.trigger_device_id) {
            return Err(format!(
                "trigger_device_id '{}' not found",
                req.trigger_device_id
            ));
        }
        if !matches!(
            req.trigger_condition.as_str(),
            "turns_on" | "turns_off" | "level_above" | "temp_above"
        ) {
            return Err(
                "trigger_condition must be one of: turns_on, turns_off, level_above, temp_above"
                    .to_string(),
            );
        }
        if matches!(req.trigger_condition.as_str(), "level_above" | "temp_above") {
            match req.trigger_value {
                Some(v) if !v.is_nan() => {}
                _ => {
                    return Err(
                        "trigger_value is required for level_above and temp_above conditions"
                            .to_string(),
                    );
                }
            }
        }
        if req.actions.is_empty() {
            return Err("automation rule must include at least one action".to_string());
        }

        for act in &req.actions {
            if !self.devices.iter().any(|d| d.id == act.target_device_id) {
                return Err(format!(
                    "target_device_id '{}' not found",
                    act.target_device_id
                ));
            }
            if !matches!(
                act.action.as_str(),
                "turn_on" | "turn_off" | "toggle" | "set_level" | "set_temp"
            ) {
                return Err(
                    "action must be one of: turn_on, turn_off, toggle, set_level, set_temp"
                        .to_string(),
                );
            }
            if act.action == "set_level" {
                match act.value {
                    Some(v) if !v.is_nan() && (0.0..=100.0).contains(&v) => {}
                    _ => {
                        return Err("set_level action requires value between 0 and 100".to_string());
                    }
                }
            }
            if act.action == "set_temp" {
                match act.value {
                    Some(v) if !v.is_nan() && (16.0..=30.0).contains(&v) => {}
                    _ => {
                        return Err(
                            "set_temp action requires value between 16.0 and 30.0".to_string(),
                        );
                    }
                }
            }
        }

        let id = format!("rule-custom-{}", self.next_rule_seq);
        self.next_rule_seq += 1;

        let rule = AutomationRule {
            id,
            name,
            enabled: req.enabled.unwrap_or(true),
            trigger_device_id: req.trigger_device_id,
            trigger_condition: req.trigger_condition,
            trigger_value: req.trigger_value,
            actions: req.actions,
        };

        self.automations.push(rule.clone());
        Ok(rule)
    }

    /// Flips `enabled` on an automation rule by ID.
    pub fn toggle_automation(&mut self, id: &str) -> Option<AutomationRule> {
        let rule = self.automations.iter_mut().find(|r| r.id == id)?;
        rule.enabled = !rule.enabled;
        Some(rule.clone())
    }

    /// Deletes an automation rule by ID, returning `true` if it existed.
    pub fn delete_automation(&mut self, id: &str) -> bool {
        let before = self.automations.len();
        self.automations.retain(|r| r.id != id);
        self.automations.len() != before
    }

    /// Manually executes an automation rule's actions (and any downstream enabled cascades).
    pub fn run_automation(&mut self, id: &str) -> Option<OverviewResponse> {
        let actions = self
            .automations
            .iter()
            .find(|r| r.id == id)?
            .actions
            .clone();

        let mut visited_rules = HashSet::new();
        visited_rules.insert(id.to_string());

        let mut queue = VecDeque::new();
        for action in actions {
            if let Some((prev, curr)) = self.apply_rule_action(&action) {
                queue.push_back((prev, curr, 1));
            }
        }

        self.process_automation_queue(&mut queue, &mut visited_rules);
        Some(self.overview())
    }

    fn run_automation_cascade(&mut self, prev: Device, curr: Device) {
        let mut visited_rules = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((prev, curr, 0));
        self.process_automation_queue(&mut queue, &mut visited_rules);
    }

    fn process_automation_queue(
        &mut self,
        queue: &mut VecDeque<(Device, Device, usize)>,
        visited_rules: &mut HashSet<String>,
    ) {
        while let Some((prev_dev, curr_dev, depth)) = queue.pop_front() {
            if depth >= MAX_CASCADE_DEPTH {
                continue;
            }

            let matching_rules: Vec<(String, Vec<RuleAction>)> = self
                .automations
                .iter()
                .filter(|rule| {
                    rule.enabled
                        && !visited_rules.contains(&rule.id)
                        && rule.trigger_device_id == curr_dev.id
                        && Self::rule_matches(rule, &prev_dev, &curr_dev)
                })
                .map(|rule| (rule.id.clone(), rule.actions.clone()))
                .collect();

            for (rule_id, actions) in matching_rules {
                visited_rules.insert(rule_id);
                for action in actions {
                    if let Some((target_prev, target_curr)) = self.apply_rule_action(&action) {
                        queue.push_back((target_prev, target_curr, depth + 1));
                    }
                }
            }
        }
    }

    fn rule_matches(rule: &AutomationRule, prev: &Device, curr: &Device) -> bool {
        match rule.trigger_condition.as_str() {
            "turns_on" => !prev.is_on && curr.is_on,
            "turns_off" => prev.is_on && !curr.is_on,
            "level_above" => {
                let Some(threshold) = rule.trigger_value else {
                    return false;
                };
                let Some(curr_lvl) = curr.level else {
                    return false;
                };
                let was_above = prev
                    .level
                    .map(|p| f32::from(p) > threshold)
                    .unwrap_or(false);
                f32::from(curr_lvl) > threshold && !was_above
            }
            "temp_above" => {
                let Some(threshold) = rule.trigger_value else {
                    return false;
                };
                let Some(curr_temp) = curr.temperature_c else {
                    return false;
                };
                let was_above = prev.temperature_c.map(|p| p > threshold).unwrap_or(false);
                curr_temp > threshold && !was_above
            }
            _ => false,
        }
    }

    fn apply_rule_action(&mut self, action: &RuleAction) -> Option<(Device, Device)> {
        let target = self
            .devices
            .iter_mut()
            .find(|d| d.id == action.target_device_id)?;
        let prev = target.clone();

        match action.action.as_str() {
            "turn_on" => target.is_on = true,
            "turn_off" => target.is_on = false,
            "toggle" => target.is_on = !target.is_on,
            "set_level" => {
                if let Some(val) = action.value {
                    target.level = Some(val.clamp(0.0, 100.0).round() as u8);
                    target.is_on = true;
                }
            }
            "set_temp" => {
                if let Some(val) = action.value {
                    target.temperature_c = Some(val.clamp(16.0, 30.0));
                }
            }
            _ => return None,
        }

        Self::recompute_device(target);
        let curr = target.clone();
        if prev != curr {
            Some((prev, curr))
        } else {
            None
        }
    }
}
