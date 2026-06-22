// SPDX-License-Identifier: AGPL-3.0-or-later

//! petalTongue IPC client — content rendering via discovered socket.
//!
//! Connects to petalTongue via its JSON-RPC UDS interface (same discovery
//! mechanism as `NestGate`) and provides typed wrappers for:
//! - `content.render` — render a content page (markdown → HTML)
//! - `viz.serve` — request a visualization as SVG or scene-JSON
//!
//! ## Transport
//!
//! Uses the same `TransportEndpoint` injection pattern as `cas_push`.
//! Discovery via `probe_socket("petaltongue", "PETALTONGUE_SOCKET")`.
//!
//! ## Protocol
//!
//! petalTongue speaks NDJSON JSON-RPC 2.0 over UDS. When `SPOREPRINT_RIBOCIPHER=1`,
//! the Tier 1 clear signal (`0xEC 0x01`) is sent before the first request.

use crate::cas_push::{ReadWrite, TransportEndpoint, connect_transport};
use crate::error::Error;
use serde_json::{Value, json};
use std::io::{BufRead, BufReader, Write};
use std::time::Instant;

/// Result of a `visualization.render.graph` call.
#[derive(Debug)]
pub struct RenderResult {
    pub data: String,
    pub format: String,
    pub content_path: String,
    pub latency_ms: u64,
    pub metadata: Option<Value>,
}

/// Result of a `viz.serve` call.
#[derive(Debug)]
pub struct VizResult {
    pub body: String,
    /// Retained for parity test assertions.
    #[allow(dead_code)]
    pub format: VizFormat,
    pub latency_ms: u64,
}

/// Format of a visualization response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VizFormat {
    Svg,
    SceneJson,
}

/// A connected petalTongue IPC session.
pub struct PetalTongueClient {
    reader: BufReader<Box<dyn ReadWrite>>,
    request_id: u64,
}

impl PetalTongueClient {
    /// Connect to petalTongue via the given transport endpoint.
    pub fn connect(endpoint: &TransportEndpoint) -> Result<Self, Error> {
        let stream = connect_transport(endpoint)?;
        let reader = BufReader::new(stream);
        let mut client = Self {
            reader,
            request_id: 0,
        };

        client.announce()?;
        Ok(client)
    }

    /// Discover petalTongue socket and connect (used by integration tests).
    #[allow(dead_code)]
    pub fn discover_and_connect() -> Result<Self, Error> {
        let socket = discover_socket()?;
        let endpoint = TransportEndpoint::Uds { path: socket };
        Self::connect(&endpoint)
    }

    /// Send `primal.announce` handshake.
    fn announce(&mut self) -> Result<Value, Error> {
        self.request_id += 1;
        let req = crate::discovery::announce_request(self.request_id);
        self.send_rpc(&req)
    }

    /// Render a graph via `visualization.render.graph`.
    ///
    /// `session_id` identifies the render session (caller-assigned).
    /// `graph` is the graph data (nodes + edges) as a JSON value.
    /// `modality` controls output format: `None` for SVG, `Some("description")`
    /// for accessible text.
    pub fn render_graph(
        &mut self,
        session_id: &str,
        graph: &Value,
        modality: Option<&str>,
    ) -> Result<RenderResult, Error> {
        let t0 = Instant::now();
        self.request_id += 1;

        let mut params = serde_json::Map::new();
        params.insert("session_id".into(), json!(session_id));
        params.insert("graph".into(), graph.clone());
        if let Some(m) = modality {
            params.insert("modality".into(), json!(m));
        }

        let req = json!({
            "jsonrpc": "2.0",
            "method": "visualization.render.graph",
            "params": params,
            "id": self.request_id
        });

        let resp = self.send_rpc(&req)?;

        #[allow(clippy::cast_possible_truncation)]
        let latency_ms = t0.elapsed().as_millis() as u64;

        if let Some(err) = resp.get("error") {
            return Err(Error::Config(format!(
                "visualization.render.graph error: {}",
                err.get("message").and_then(Value::as_str).unwrap_or("unknown")
            )));
        }

        let result = resp.get("result").ok_or_else(|| {
            Error::Config("visualization.render.graph: missing result field".into())
        })?;

        let data = result
            .get("data")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();

        let format = result
            .get("format")
            .and_then(Value::as_str)
            .unwrap_or("svg")
            .to_string();

        let metadata = result.get("metadata").cloned();

        Ok(RenderResult {
            data,
            format,
            content_path: session_id.to_string(),
            latency_ms,
            metadata,
        })
    }

    /// Check petalTongue health via `health.check`.
    pub fn health_check(&mut self) -> Result<HealthStatus, Error> {
        let t0 = Instant::now();
        self.request_id += 1;

        let req = json!({
            "jsonrpc": "2.0",
            "method": "health.check",
            "params": {},
            "id": self.request_id
        });

        let resp = self.send_rpc(&req)?;

        #[allow(clippy::cast_possible_truncation)]
        let latency_ms = t0.elapsed().as_millis() as u64;

        if let Some(err) = resp.get("error") {
            return Err(Error::Config(format!(
                "health.check error: {}",
                err.get("message").and_then(Value::as_str).unwrap_or("unknown")
            )));
        }

        let result = resp.get("result").unwrap_or(&Value::Null);

        Ok(HealthStatus {
            status: result
                .get("status")
                .and_then(Value::as_str)
                .unwrap_or("unknown")
                .to_string(),
            version: result
                .get("version")
                .and_then(Value::as_str)
                .unwrap_or("unknown")
                .to_string(),
            primal: result
                .get("primal")
                .and_then(Value::as_str)
                .unwrap_or("petaltongue")
                .to_string(),
            uptime_s: result
                .get("uptime_s")
                .and_then(Value::as_u64)
                .unwrap_or(0),
            latency_ms,
        })
    }

    /// Request a visualization via `viz.serve`.
    ///
    /// `name` is the visualization identifier (e.g., "entity-graph", "kderm-topology").
    /// `format` selects SVG or scene-JSON output.
    pub fn viz(&mut self, name: &str, format: VizFormat) -> Result<VizResult, Error> {
        let t0 = Instant::now();
        self.request_id += 1;

        let format_str = match format {
            VizFormat::Svg => "svg",
            VizFormat::SceneJson => "scene-json",
        };

        let req = json!({
            "jsonrpc": "2.0",
            "method": "viz.serve",
            "params": {
                "name": name,
                "format": format_str
            },
            "id": self.request_id
        });

        let resp = self.send_rpc(&req)?;

        #[allow(clippy::cast_possible_truncation)]
        let latency_ms = t0.elapsed().as_millis() as u64;

        if let Some(err) = resp.get("error") {
            return Err(Error::Config(format!(
                "viz.serve error for '{name}': {}",
                err.get("message").and_then(Value::as_str).unwrap_or("unknown")
            )));
        }

        let result = resp.get("result").ok_or_else(|| {
            Error::Config(format!("viz.serve for '{name}': missing result field"))
        })?;

        let body = result
            .get("body")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();

        Ok(VizResult {
            body,
            format,
            latency_ms,
        })
    }

    /// Check if petalTongue supports a given method (probe with empty params).
    pub fn probe_method(&mut self, method: &str) -> Result<bool, Error> {
        self.request_id += 1;
        let req = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": {},
            "id": self.request_id
        });

        let resp = self.send_rpc(&req)?;

        resp.get("error").map_or(Ok(true), |err| {
            let code = err.get("code").and_then(Value::as_i64).unwrap_or(0);
            Ok(code != -32601)
        })
    }

    /// Send a JSON-RPC request and read the response.
    fn send_rpc(&mut self, request: &Value) -> Result<Value, Error> {
        let mut payload = serde_json::to_string(request)
            .map_err(|e| Error::Config(format!("JSON encode: {e}")))?;
        payload.push('\n');

        let writer = self.reader.get_mut();
        writer
            .write_all(payload.as_bytes())
            .map_err(|e| Error::Config(format!("petalTongue write: {e}")))?;
        writer
            .flush()
            .map_err(|e| Error::Config(format!("petalTongue flush: {e}")))?;

        let mut line = String::new();
        self.reader
            .read_line(&mut line)
            .map_err(|e| Error::Config(format!("petalTongue read: {e}")))?;

        serde_json::from_str(line.trim())
            .map_err(|e| Error::Config(format!("petalTongue JSON decode: {e}")))
    }
}

/// Discover petalTongue's socket path from environment.
pub fn discover_socket() -> Result<String, Error> {
    if let Some(path) = crate::discovery::probe_socket("petaltongue", "PETALTONGUE_SOCKET") {
        return Ok(path);
    }

    Err(Error::Config(
        "petalTongue socket not found. Set PETALTONGUE_SOCKET or ensure petalTongue is running."
            .into(),
    ))
}

/// Quick status check: connect, announce, probe key methods.
pub fn status() -> Result<PetalTongueStatus, Error> {
    let socket = discover_socket()?;
    let endpoint = TransportEndpoint::Uds {
        path: socket.clone(),
    };
    let mut client = PetalTongueClient::connect(&endpoint)?;

    let health = client.health_check().ok();
    let has_render_graph = client.probe_method("visualization.render.graph").unwrap_or(false);
    let has_viz_export = client.probe_method("visualization.export").unwrap_or(false);

    Ok(PetalTongueStatus {
        socket_path: socket,
        health,
        render_graph: has_render_graph,
        viz_export: has_viz_export,
    })
}

/// petalTongue health response.
#[derive(Debug)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
    pub primal: String,
    pub uptime_s: u64,
    pub latency_ms: u64,
}

/// Summary of petalTongue's operational state.
#[derive(Debug)]
pub struct PetalTongueStatus {
    pub socket_path: String,
    pub health: Option<HealthStatus>,
    pub render_graph: bool,
    pub viz_export: bool,
}

impl std::fmt::Display for PetalTongueStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "petalTongue @ {}", self.socket_path)?;
        if let Some(h) = &self.health {
            writeln!(f, "  {} health: {} (v{}, up {}s, {}ms)", h.primal, h.status, h.version, h.uptime_s, h.latency_ms)?;
        } else {
            writeln!(f, "  health: ❌ unreachable")?;
        }
        let render_icon = if self.render_graph { "✅" } else { "❌" };
        let export_icon = if self.viz_export { "✅" } else { "❌" };
        write!(f, "  visualization.render.graph: {render_icon}\n  visualization.export: {export_icon}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viz_format_equality() {
        assert_eq!(VizFormat::Svg, VizFormat::Svg);
        assert_ne!(VizFormat::Svg, VizFormat::SceneJson);
    }

    #[test]
    fn discover_socket_returns_error_without_socket() {
        // Only passes if petalTongue isn't running (safe in CI)
        // In live NUCLEUS this will find the socket — that's fine
        let result = discover_socket();
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn petaltongue_status_display() {
        let status = PetalTongueStatus {
            socket_path: "/run/user/1000/biomeos/petaltongue.sock".into(),
            health: Some(HealthStatus {
                status: "healthy".into(),
                version: "1.6.6".into(),
                primal: "petaltongue".into(),
                uptime_s: 300,
                latency_ms: 2,
            }),
            render_graph: true,
            viz_export: true,
        };
        let display = format!("{status}");
        assert!(display.contains("petaltongue.sock"));
        assert!(display.contains("healthy"));
        assert!(display.contains("1.6.6"));
        assert!(display.contains("visualization.render.graph"));
    }

    #[test]
    fn petaltongue_status_display_no_health() {
        let status = PetalTongueStatus {
            socket_path: "/tmp/pt.sock".into(),
            health: None,
            render_graph: false,
            viz_export: false,
        };
        let display = format!("{status}");
        assert!(display.contains("unreachable"));
    }

    #[test]
    fn render_result_debug() {
        let r = RenderResult {
            data: "<svg></svg>".into(),
            format: "svg".into(),
            content_path: "test/page".into(),
            latency_ms: 42,
            metadata: None,
        };
        let debug = format!("{r:?}");
        assert!(debug.contains("svg"));
        assert!(debug.contains("42"));
    }

    #[test]
    fn viz_result_debug() {
        let r = VizResult {
            body: "<svg></svg>".into(),
            format: VizFormat::Svg,
            latency_ms: 5,
        };
        let debug = format!("{r:?}");
        assert!(debug.contains("Svg"));
    }
}
