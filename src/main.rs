use anyhow::Result;
use lapce_plugin::{
    LapcePlugin, PLUGIN_RPC,
    psp_types::{
        Request,
        lsp_types::{DocumentFilter, InitializeParams, MessageType, Url, request::Initialize},
    },
    register_plugin,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default)]
struct State {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    arch: String,
    os: String,
    configuration: Configuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    language_id: String,
    options: Option<Value>,
}

register_plugin!(State);

use std::process::Command;

fn initialize(params: InitializeParams) -> Result<()> {
    // 1. Resolve server path:
    //    - use explicit serverPath if provided
    //    - otherwise default to "rust-analyzer"
    let server_path = params
        .initialization_options
        .as_ref()
        .and_then(|options| options.get("serverPath"))
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("rust-analyzer");

    // 2. Verify that rust-analyzer is available
    let available = Command::new(server_path)
        .arg("--version")
        .output()
        .is_ok();

    if !available {
        PLUGIN_RPC.window_show_message(
            MessageType::WARNING,
            format!(
                "rust-analyzer not found: '{}'. Please install rust-analyzer \
                 or configure 'serverPath' in the plugin settings.",
                server_path
            ),
        );
        return Ok(());
    }

    // 3. Start LSP
    PLUGIN_RPC.start_lsp(
        Url::parse(&format!("urn:{}", server_path))?,
        Vec::new(),
        vec![DocumentFilter {
            language: Some("rust".to_string()),
            scheme: None,
            pattern: None,
        }],
        params.initialization_options,
    );

    Ok(())
}


impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        #[allow(clippy::single_match)]
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                if let Err(e) = initialize(params) {
                    PLUGIN_RPC.stderr(&format!("plugin returned with error: {e}"))
                }
            }
            _ => {}
        }
    }
}
