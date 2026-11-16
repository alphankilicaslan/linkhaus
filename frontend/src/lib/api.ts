export type DeviceKind = 'lighting' | 'climate' | 'switch' | 'security' | 'energy' | string;

export interface HealthResponse {
  status: string;
  service: string;
  company: string;
  version: string;
}

export interface SystemMetrics {
  active_devices: number;
  total_devices: number;
  power_watts: number;
  uptime_secs: number;
}

export interface Room {
  id: string;
  name: string;
  floor?: string;
  temperature_c?: number;
}

export interface Device {
  id: string;
  name: string;
  room_id: string;
  kind: DeviceKind;
  is_on: boolean;
  level: number | null;
  power_watts: number;
  unit?: string;
  temperature_c?: number | null;
  climate_mode?: string | null;
  fan_speed?: string | null;
  color_temp_k?: number | null;
  button_mode?: string | null;
}

export interface Scene {
  id: string;
  name: string;
  description: string;
  is_active?: boolean;
  active?: boolean;
}

export interface RuleAction {
  target_device_id: string;
  action: string;
  value?: number | null;
}

export interface AutomationRule {
  id: string;
  name: string;
  enabled: boolean;
  trigger_device_id: string;
  trigger_condition: string;
  trigger_value?: number | null;
  actions: RuleAction[];
}

export interface OverviewResponse {
  company: string;
  product: string;
  metrics: SystemMetrics;
  rooms: Room[];
  devices: Device[];
  scenes: Scene[];
  automations?: AutomationRule[];
}

export interface UpdateDeviceRequest {
  is_on?: boolean;
  level?: number;
  name?: string;
  temperature_c?: number;
  climate_mode?: string;
  fan_speed?: string;
  color_temp_k?: number;
  button_mode?: string;
}

export interface CreateDeviceRequest {
  name: string;
  room_id: string;
  kind: DeviceKind;
  level?: number;
  temperature_c?: number;
  climate_mode?: string;
  fan_speed?: string;
  color_temp_k?: number;
  button_mode?: string;
}

export interface CreateAutomationRequest {
  name: string;
  trigger_device_id: string;
  trigger_condition: string;
  trigger_value?: number;
  actions: RuleAction[];
}

const DEFAULT_API_BASE = 'http://127.0.0.1:8080';

function getApiBase(): string {
  if (typeof window !== 'undefined') {
    const custom = (window as unknown as { __LINKHAUS_API__?: string }).__LINKHAUS_API__;
    if (custom) return custom.replace(/\/$/, '');
  }
  return DEFAULT_API_BASE;
}

async function requestJson<T>(path: string, init?: RequestInit): Promise<T> {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), 4000);

  try {
    const res = await fetch(`${getApiBase()}${path}`, {
      ...init,
      headers: {
        'Content-Type': 'application/json',
        ...(init?.headers ?? {}),
      },
      signal: controller.signal,
    });

    if (!res.ok) {
      throw new Error(`API ${res.status}: ${res.statusText}`);
    }

    if (res.status === 204) {
      return undefined as T;
    }

    const text = await res.text();
    if (!text) {
      return undefined as T;
    }
    return JSON.parse(text) as T;
  } finally {
    clearTimeout(timer);
  }
}

export async function fetchHealth(): Promise<HealthResponse> {
  return requestJson<HealthResponse>('/api/health');
}

export async function fetchOverview(): Promise<OverviewResponse> {
  return requestJson<OverviewResponse>('/api/overview');
}

export async function toggleDevice(id: string): Promise<Device> {
  return requestJson<Device>(`/api/devices/${encodeURIComponent(id)}/toggle`, {
    method: 'POST',
  });
}

export async function updateDevice(id: string, payload: UpdateDeviceRequest): Promise<Device> {
  return requestJson<Device>(`/api/devices/${encodeURIComponent(id)}`, {
    method: 'PATCH',
    body: JSON.stringify(payload),
  });
}

export async function createDevice(payload: CreateDeviceRequest): Promise<Device> {
  return requestJson<Device>('/api/devices', {
    method: 'POST',
    body: JSON.stringify(payload),
  });
}

export async function deleteDevice(id: string): Promise<void> {
  return requestJson<void>(`/api/devices/${encodeURIComponent(id)}`, {
    method: 'DELETE',
  });
}

export async function activateScene(id: string): Promise<OverviewResponse> {
  return requestJson<OverviewResponse>(`/api/scenes/${encodeURIComponent(id)}/activate`, {
    method: 'POST',
  });
}

export async function fetchAutomations(): Promise<AutomationRule[]> {
  return requestJson<AutomationRule[]>('/api/automations');
}

export async function createAutomation(payload: CreateAutomationRequest): Promise<AutomationRule> {
  return requestJson<AutomationRule>('/api/automations', {
    method: 'POST',
    body: JSON.stringify(payload),
  });
}

export async function toggleAutomation(id: string): Promise<AutomationRule> {
  return requestJson<AutomationRule>(`/api/automations/${encodeURIComponent(id)}/toggle`, {
    method: 'PATCH',
  });
}

export async function runAutomation(id: string): Promise<OverviewResponse> {
  return requestJson<OverviewResponse>(`/api/automations/${encodeURIComponent(id)}/run`, {
    method: 'POST',
  });
}

export async function deleteAutomation(id: string): Promise<void> {
  return requestJson<void>(`/api/automations/${encodeURIComponent(id)}`, {
    method: 'DELETE',
  });
}

export const OFFLINE_FALLBACK_OVERVIEW: OverviewResponse = {
  company: 'Helios Smart Systems',
  product: 'LinkHaus',
  metrics: {
    active_devices: 5,
    total_devices: 10,
    power_watts: 218,
    uptime_secs: 0,
  },
  rooms: [
    { id: 'living', name: 'Living Room', floor: 'Level 01', temperature_c: 22.0 },
    { id: 'studio', name: 'Studio', floor: 'Level 02', temperature_c: 21.0 },
    { id: 'entryway', name: 'Entryway', floor: 'Level 01', temperature_c: 19.5 },
  ],
  devices: [
    {
      id: 'demo-climate-1',
      name: 'Living Room AC',
      room_id: 'living',
      kind: 'climate',
      is_on: true,
      level: 65,
      power_watts: 110,
      temperature_c: 22,
      climate_mode: 'cool',
      fan_speed: 'medium',
    },
    {
      id: 'demo-light-1',
      name: 'Linear Cove Array',
      room_id: 'living',
      kind: 'lighting',
      is_on: true,
      level: 72,
      power_watts: 42,
      unit: '%',
      color_temp_k: 3400,
    },
    {
      id: 'demo-light-2',
      name: 'Desk Task Beam',
      room_id: 'studio',
      kind: 'lighting',
      is_on: true,
      level: 88,
      power_watts: 28,
      unit: '%',
      color_temp_k: 4800,
    },
    {
      id: 'demo-btn-1',
      name: 'Button 1',
      room_id: 'living',
      kind: 'switch',
      is_on: false,
      level: null,
      power_watts: 4,
      button_mode: 'toggle',
    },
    {
      id: 'demo-btn-2',
      name: 'Button 2',
      room_id: 'living',
      kind: 'switch',
      is_on: false,
      level: null,
      power_watts: 4,
      button_mode: 'toggle',
    },
    {
      id: 'demo-btn-3',
      name: 'Button 3',
      room_id: 'studio',
      kind: 'switch',
      is_on: false,
      level: null,
      power_watts: 4,
      button_mode: 'toggle',
    },
    {
      id: 'demo-btn-4',
      name: 'Button 4',
      room_id: 'entryway',
      kind: 'switch',
      is_on: false,
      level: null,
      power_watts: 4,
      button_mode: 'toggle',
    },
    {
      id: 'demo-energy-1',
      name: 'Solar Storage Node',
      room_id: 'studio',
      kind: 'energy',
      is_on: true,
      level: 84,
      power_watts: 19,
      unit: '%',
    },
    {
      id: 'demo-lock-1',
      name: 'Perimeter Deadbolt',
      room_id: 'entryway',
      kind: 'security',
      is_on: true,
      level: null,
      power_watts: 4,
    },
    {
      id: 'demo-light-3',
      name: 'Threshold Strip',
      room_id: 'entryway',
      kind: 'lighting',
      is_on: false,
      level: 30,
      power_watts: 8,
      unit: '%',
      color_temp_k: 2700,
    },
  ],
  scenes: [
    {
      id: 'monochrome-evening',
      name: 'Monochrome Evening',
      description: 'Low-glare warm architectural lighting at 35% output.',
      is_active: false,
      active: false,
    },
    {
      id: 'focus-mode',
      name: 'Focus Mode',
      description: 'Crisp studio illumination with stabilized 21°C airflow.',
      is_active: true,
      active: true,
    },
    {
      id: 'away-guard',
      name: 'Away Guard',
      description: 'Engages perimeter security and trims non-essential loads.',
      is_active: false,
      active: false,
    },
  ],
  automations: [
    {
      id: 'demo-rule-cascade',
      name: 'Button 1 Cascade',
      enabled: true,
      trigger_device_id: 'demo-btn-1',
      trigger_condition: 'turns_on',
      actions: [
        { target_device_id: 'demo-btn-2', action: 'turn_on' },
        { target_device_id: 'demo-btn-3', action: 'turn_on' },
        { target_device_id: 'demo-btn-4', action: 'turn_on' },
      ],
    },
  ],
};
