#!/usr/bin/env node
import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { z } from "zod";
import { LinkHausApiError, LinkHausClient } from "./client.js";

const client = new LinkHausClient();

const server = new McpServer({
  name: "linkhaus-mcp",
  version: "1.0.0",
});

type ToolResult = {
  content: Array<{ type: "text"; text: string }>;
  isError?: boolean;
};

function ok(data: unknown): ToolResult {
  return {
    content: [
      {
        type: "text",
        text: JSON.stringify(data, null, 2),
      },
    ],
  };
}

function fail(error: unknown): ToolResult {
  const message =
    error instanceof LinkHausApiError
      ? `LinkHaus API error (${error.status || "network"}): ${error.message}`
      : error instanceof Error
        ? error.message
        : String(error);

  return {
    isError: true,
    content: [{ type: "text", text: message }],
  };
}

async function runTool(fn: () => Promise<unknown>): Promise<ToolResult> {
  try {
    const result = await fn();
    return ok(result);
  } catch (error) {
    return fail(error);
  }
}

server.tool(
  "linkhaus_health",
  "Check LinkHaus Rust core health, service status, and version.",
  {},
  async () => runTool(() => client.health()),
);

server.tool(
  "linkhaus_overview",
  "Get the full LinkHaus smart home snapshot: metrics, rooms, devices, scenes, and IF/THEN automations.",
  {},
  async () => runTool(() => client.overview()),
);

server.tool(
  "linkhaus_toggle_device",
  "Toggle a smart home device ON/OFF by device_id. Also triggers any matching IF/THEN automation rules.",
  {
    device_id: z.string().min(1).describe("The unique ID of the device to toggle"),
  },
  async ({ device_id }) => runTool(() => client.toggleDevice(device_id)),
);

server.tool(
  "linkhaus_update_device",
  "Update device settings such as power state, brightness level, climate temperature/mode/fan, lighting color temperature, or switch button mode.",
  {
    device_id: z.string().min(1).describe("The unique ID of the device to update"),
    name: z.string().min(1).optional().describe("Optional new display name for the device"),
    is_on: z.boolean().optional().describe("Turn the device ON (true) or OFF (false)"),
    level: z
      .number()
      .int()
      .min(0)
      .max(100)
      .optional()
      .describe("Intensity or brightness level (0-100)"),
    temperature_c: z
      .number()
      .min(16)
      .max(30)
      .optional()
      .describe("Target temperature in Celsius for climate devices (16-30)"),
    climate_mode: z
      .enum(["cool", "heat", "auto", "fan"])
      .optional()
      .describe("Climate operating mode"),
    fan_speed: z
      .enum(["low", "medium", "high", "auto"])
      .optional()
      .describe("Fan speed for climate devices"),
    color_temp_k: z
      .number()
      .int()
      .min(2700)
      .max(6500)
      .optional()
      .describe("Color temperature in Kelvin for lighting devices (2700-6500)"),
    button_mode: z
      .enum(["toggle", "momentary", "scene"])
      .optional()
      .describe("Operating mode for switch/button devices"),
  },
  async ({ device_id, ...payload }) =>
    runTool(() => client.updateDevice(device_id, payload)),
);

server.tool(
  "linkhaus_create_device",
  "Add a new smart home device to LinkHaus.",
  {
    name: z.string().min(1).describe("Display name of the new device"),
    room_id: z.string().min(1).describe("ID of the room where the device is located"),
    kind: z
      .enum(["lighting", "climate", "switch", "security", "energy"])
      .describe("Device category"),
    is_on: z.boolean().optional().describe("Initial power state"),
    level: z
      .number()
      .int()
      .min(0)
      .max(100)
      .optional()
      .describe("Initial brightness/intensity level (0-100)"),
    temperature_c: z
      .number()
      .min(16)
      .max(30)
      .optional()
      .describe("Initial target temperature in Celsius for climate devices"),
    climate_mode: z
      .enum(["cool", "heat", "auto", "fan"])
      .optional()
      .describe("Initial climate mode"),
    fan_speed: z
      .enum(["low", "medium", "high", "auto"])
      .optional()
      .describe("Initial fan speed"),
    color_temp_k: z
      .number()
      .int()
      .min(2700)
      .max(6500)
      .optional()
      .describe("Initial color temperature in Kelvin (2700-6500)"),
    button_mode: z
      .enum(["toggle", "momentary", "scene"])
      .optional()
      .describe("Initial button mode for switch devices"),
  },
  async (payload) => runTool(() => client.createDevice(payload)),
);

server.tool(
  "linkhaus_delete_device",
  "Remove a smart home device from LinkHaus by device_id.",
  {
    device_id: z.string().min(1).describe("The unique ID of the device to delete"),
  },
  async ({ device_id }) =>
    runTool(async () => {
      await client.deleteDevice(device_id);
      return { deleted: true, device_id };
    }),
);

server.tool(
  "linkhaus_activate_scene",
  "Activate a LinkHaus smart home scene preset by scene_id.",
  {
    scene_id: z.string().min(1).describe("The unique ID of the scene to activate"),
  },
  async ({ scene_id }) => runTool(() => client.activateScene(scene_id)),
);

server.tool(
  "linkhaus_list_automations",
  "List all IF/THEN automation rules configured in LinkHaus.",
  {},
  async () => runTool(() => client.listAutomations()),
);

server.tool(
  "linkhaus_create_automation",
  "Create a new IF/THEN automation rule that triggers actions on target devices when a trigger condition is met.",
  {
    name: z.string().min(1).describe("Human-readable name for the automation rule"),
    enabled: z
      .boolean()
      .optional()
      .describe("Whether the rule starts enabled (defaults to true on backend)"),
    trigger_device_id: z
      .string()
      .min(1)
      .describe("ID of the device whose state change triggers this rule"),
    trigger_condition: z
      .enum(["turns_on", "turns_off", "level_above", "temp_above"])
      .describe("Condition that triggers the automation"),
    trigger_value: z
      .number()
      .optional()
      .describe("Threshold value for level_above or temp_above conditions"),
    actions: z
      .array(
        z.object({
          target_device_id: z
            .string()
            .min(1)
            .describe("ID of the target device to control"),
          action: z
            .enum(["turn_on", "turn_off", "toggle", "set_level", "set_temp"])
            .describe("Action to execute on the target device"),
          value: z
            .number()
            .optional()
            .describe("Numeric value for set_level or set_temp actions"),
        }),
      )
      .min(1)
      .describe("One or more THEN actions to execute when triggered"),
  },
  async (payload) => runTool(() => client.createAutomation(payload)),
);

server.tool(
  "linkhaus_run_automation",
  "Manually execute an IF/THEN automation rule by rule_id and return the updated system overview.",
  {
    rule_id: z.string().min(1).describe("The unique ID of the automation rule to run"),
  },
  async ({ rule_id }) => runTool(() => client.runAutomation(rule_id)),
);

async function main(): Promise<void> {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error(`LinkHaus MCP server running on stdio (API: ${client.baseUrl})`);
}

main().catch((error) => {
  console.error("Fatal error starting LinkHaus MCP server:", error);
  process.exit(1);
});
