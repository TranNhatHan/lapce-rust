use anyhow::Result;
use lapce_plugin::{
    LapcePlugin, PLUGIN_RPC,
    psp_types::{
        Request, // <- required for Initialize::METHOD
        lsp_types::{DocumentFilter, InitializeParams, Url, request::Initialize},
    },
    register_plugin,
};
use serde_json::Value;

#[derive(Default)]
struct State;

register_plugin!(State);

fn initialize(params: InitializeParams) -> Result<()> {
    // 1. Read `serverPath` from plugin settings
    // If empty or not set, default to "rust-analyzer"
    let mut server = params
        .initialization_options
        .as_ref()
        .and_then(|opts| opts.get("serverPath"))
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if server.is_empty() {
        server = "rust-analyzer";
    }

    // 2. Platform-specific adjustment for Windows executable
    #[cfg(windows)]
    {
        if !server.ends_with(".exe") {
            server = "rust-analyzer.exe";
        }
    }

    // 3. Construct a URL Lapce understands for PATH/executable
    // Lapce uses `urn:command` for executables in PATH
    let server_url = Url::parse(&format!("urn:{}", server))?;

    // 4. Start the LSP server
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
            // <- now works
            let params: InitializeParams = serde_json::from_value(params).unwrap();
            if let Err(e) = initialize(params) {
                PLUGIN_RPC.stderr(&format!("rust plugin error: {e}"));
            }
        }
    }
}
