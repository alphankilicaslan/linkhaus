# LinkHaus — Helios Smart Systems

**LinkHaus** is a local-first smart home platform developed by **Helios Smart Systems**, combining a high-performance **Rust** core backend, an Apple-inspired monochrome **Astro** web interface, and a **Model Context Protocol (MCP)** server for AI assistant control.

## Architecture

- [`backend/`](./backend) — Rust (`axum`, `tokio`, `serde`) REST API & autonomous IF/THEN rule engine (`127.0.0.1:8080`)
- [`frontend/`](./frontend) — Astro web UI with Apple-inspired frosted monochrome Dark/Light modes, custom SVG device controls, and live telemetry (`localhost:4321`)
- [`mcp/`](./mcp) — TypeScript MCP server (`@modelcontextprotocol/sdk`) exposing 10 smart-home tools for AI clients

## Features

- **Device Management:** Add, remove, toggle, and configure devices across rooms (`POST /api/devices`, `PATCH /api/devices/{id}`, `DELETE /api/devices/{id}`).
- **Specialized Controls:**
  - **Climate:** Target temperature (`16°C–30°C`), mode (`cool`, `heat`, `auto`, `fan`), and fan speed (`low`, `medium`, `high`).
  - **Lighting:** Brightness (`0–100%`) and color temperature (`2700K–6500K`).
  - **Switches / Buttons:** Configurable button modes (`toggle`, `momentary`, `scene`).
- **Autonomous IF/THEN Engine:** Create multi-target automation rules (e.g., *IF Button 1 turns ON → THEN turn ON Button 2, 3, and 4*) with loop protection.
- **AI / MCP Integration:** Connect Claude, Antigravity, or Cursor directly to LinkHaus via stdio MCP.

## Quick Start

### 1. Start the Rust Backend
```powershell
cd backend
cargo run
```

### 2. Start the Astro Frontend
```powershell
cd frontend
npm install
npm run dev
```

### 3. Build the MCP Server
```powershell
cd mcp
npm install
npm run build
```
See [`mcp/README.md`](./mcp/README.md) for `mcp_config.json` setup.
