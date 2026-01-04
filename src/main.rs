use anyhow::Result;
use lapce_plugin::{
    LapcePlugin, PLUGIN_RPC,
    psp_types::{
        Request,
        lsp_types::{DocumentFilter, InitializeParams, Url, request::Initialize},
    },
    register_plugin,
};
use serde_json::Value;

#[derive(Default)]
struct State;

register_plugin!(State);

fn initialize(params: InitializeParams) -> Result<()> {
    // Read `serverPath` from volt.toml config
    // Default is "" → automatically use "rust-analyzer" from PATH
    let server = params
        .initialization_options
        .as_ref()
        .and_then(|opts| opts.get("serverPath"))
        .and_then(|v| v.as_str())
        .unwrap_or(""); // keep empty string if not set

    // Treat empty string as "rust-analyzer"
    let server = if server.is_empty() {
        "rust-analyzer"
    } else {
        server
    };

    // Lapce expects a command-style URL for PATH executables
    let server_url = Url::parse(&format!("urn:{}", server))?;

    PLUGIN_RPC.start_lsp(
        server_url,
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
        if method == Initialize::METHOD {
            let params: InitializeParams = serde_json::from_value(params).unwrap();
            if let Err(e) = initialize(params) {
                PLUGIN_RPC.stderr(&format!("rust plugin error: {e}"));
            }
        }
    }
}
