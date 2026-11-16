export type DeviceKind = "lighting" | "climate" | "switch" | "security" | "energy";

export type ClimateMode = "cool" | "heat" | "auto" | "fan";
export type FanSpeed = "low" | "medium" | "high" | "auto";
export type ButtonMode = "toggle" | "momentary" | "scene";

export type TriggerCondition = "turns_on" | "turns_off" | "level_above" | "temp_above";
export type RuleActionType = "turn_on" | "turn_off" | "toggle" | "set_level" | "set_temp";

export interface UpdateDevicePayload {
  name?: string;
  is_on?: boolean;
  level?: number;
  temperature_c?: number;
  climate_mode?: string;
  fan_speed?: string;
  color_temp_k?: number;
  button_mode?: string;
}

export interface CreateDevicePayload {
  name: string;
  room_id: string;
  kind: DeviceKind;
  is_on?: boolean;
  level?: number;
  temperature_c?: number;
  climate_mode?: string;
  fan_speed?: string;
  color_temp_k?: number;
  button_mode?: string;
}

export interface RuleActionInput {
  target_device_id: string;
  action: RuleActionType;
  value?: number;
}

export interface CreateAutomationPayload {
  name: string;
  enabled?: boolean;
  trigger_device_id: string;
  trigger_condition: TriggerCondition;
  trigger_value?: number;
  actions: RuleActionInput[];
}

export class LinkHausApiError extends Error {
  readonly status: number;

  constructor(status: number, message: string) {
    super(message);
    this.name = "LinkHausApiError";
    this.status = status;
  }
}

export class LinkHausClient {
  readonly baseUrl: string;
  private readonly timeoutMs: number;

  constructor(
    baseUrl: string = process.env.LINKHAUS_API_URL || "http://127.0.0.1:8080",
    timeoutMs: number = 10_000,
  ) {
    this.baseUrl = baseUrl.replace(/\/+$/, "");
    this.timeoutMs = timeoutMs;
  }

  private async request<T>(method: string, path: string, body?: unknown): Promise<T> {
    const url = `${this.baseUrl}${path}`;
    const headers: Record<string, string> = {
      Accept: "application/json",
    };

    const init: RequestInit = {
      method,
      headers,
      signal: AbortSignal.timeout(this.timeoutMs),
    };

    if (body !== undefined) {
      headers["Content-Type"] = "application/json";
      init.body = JSON.stringify(body);
    }

    let response: Response;
    try {
      response = await fetch(url, init);
    } catch (error) {
      const reason = error instanceof Error ? error.message : String(error);
      throw new LinkHausApiError(
        0,
        `Failed to reach LinkHaus backend at ${url}: ${reason}`,
      );
    }

    if (response.status === 204) {
      return undefined as T;
    }

    const text = await response.text();
    let parsed: unknown = null;
    if (text.length > 0) {
      try {
        parsed = JSON.parse(text);
      } catch {
        parsed = text;
      }
    }

    if (!response.ok) {
      const errorMessage =
        typeof parsed === "object" &&
        parsed !== null &&
        "error" in parsed &&
        typeof (parsed as { error: unknown }).error === "string"
          ? (parsed as { error: string }).error
          : typeof parsed === "string" && parsed.length > 0
            ? parsed
            : `HTTP ${response.status} ${response.statusText}`;

      throw new LinkHausApiError(response.status, errorMessage);
    }

    return parsed as T;
  }

  health(): Promise<unknown> {
    return this.request("GET", "/api/health");
  }

  overview(): Promise<unknown> {
    return this.request("GET", "/api/overview");
  }

  toggleDevice(deviceId: string): Promise<unknown> {
    return this.request("POST", `/api/devices/${encodeURIComponent(deviceId)}/toggle`);
  }

  updateDevice(deviceId: string, payload: UpdateDevicePayload): Promise<unknown> {
    return this.request("PATCH", `/api/devices/${encodeURIComponent(deviceId)}`, payload);
  }

  createDevice(payload: CreateDevicePayload): Promise<unknown> {
    return this.request("POST", "/api/devices", payload);
  }

  deleteDevice(deviceId: string): Promise<void> {
    return this.request("DELETE", `/api/devices/${encodeURIComponent(deviceId)}`);
  }

  activateScene(sceneId: string): Promise<unknown> {
    return this.request("POST", `/api/scenes/${encodeURIComponent(sceneId)}/activate`);
  }

  listAutomations(): Promise<unknown> {
    return this.request("GET", "/api/automations");
  }

  createAutomation(payload: CreateAutomationPayload): Promise<unknown> {
    return this.request("POST", "/api/automations", payload);
  }

  runAutomation(ruleId: string): Promise<unknown> {
    return this.request("POST", `/api/automations/${encodeURIComponent(ruleId)}/run`);
  }
}
