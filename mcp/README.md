# LinkHaus MCP Server

Model Context Protocol (MCP) server for **LinkHaus** by **Helios Smart Systems**.  
Allows AI agents (Antigravity, Claude Desktop, Cursor, etc.) to inspect and control rooms, devices, scenes, and IF/THEN automations via the LinkHaus Rust backend.

## Prerequisites

- **Node.js** 22+
- **LinkHaus Rust Backend** running (default: `http://127.0.0.1:8080`)

## Build & Run

```bash
npm install
npm run build
npm start
```

### Environment Variables

| Variable | Default | Description |
| --- | --- | --- |
| `LINKHAUS_API_URL` | `http://127.0.0.1:8080` | Base URL of the LinkHaus Rust backend |

## Configuration (`mcp_config.json`)

Add the following entry to your Antigravity (`~/.gemini/antigravity/mcp_config.json`) or Claude Desktop (`claude_desktop_config.json`) configuration:

```json
{
  "mcpServers": {
    "linkhaus": {
      "command": "node",
      "args": [
        "C:/Users/Alphan.MonsterAlphan/.gemini/antigravity/scratch/linkhaus/mcp/dist/index.js"
      ],
      "env": {
        "LINKHAUS_API_URL": "http://127.0.0.1:8080"
      }
    }
  }
}
```

## Available MCP Tools

1. `linkhaus_health` — Check Rust core health & status.
2. `linkhaus_overview` — Get full rooms, devices, scenes, metrics, and automations.
3. `linkhaus_toggle_device` — Toggle a device ON/OFF by `device_id` (also triggers matching IF/THEN automations).
4. `linkhaus_update_device` — Update device settings (`is_on`, `level`, `temperature_c`, `climate_mode`, `fan_speed`, `color_temp_k`, `button_mode`, `name`).
5. `linkhaus_create_device` — Add a new device (`name`, `room_id`, `kind`, optional settings).
6. `linkhaus_delete_device` — Remove a device by `device_id`.
7. `linkhaus_activate_scene` — Activate a scene by `scene_id`.
8. `linkhaus_list_automations` — List all IF/THEN automation rules.
9. `linkhaus_create_automation` — Create a new IF/THEN automation rule (`name`, `trigger_device_id`, `trigger_condition`, `actions`).
10. `linkhaus_run_automation` — Manually run an automation rule by `rule_id`.
