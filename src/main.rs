use std::{fs::File, path::PathBuf};

use anyhow::Result;
use flate2::read::GzDecoder;
use lapce_plugin::{
    Http, LapcePlugin, PLUGIN_RPC,
    psp_types::{
        Request,
        lsp_types::{DocumentFilter, InitializeParams, MessageType, Url, request::Initialize},
    },
    register_plugin,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use zip::ZipArchive;

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

fn initialize(params: InitializeParams) -> Result<()> {
    let server_path = params
        .initialization_options
        .as_ref()
        .and_then(|options| options.get("serverPath"))
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty());

    if let Some(server_path) = server_path {
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
        return Ok(());
    }

    PLUGIN_RPC.start_lsp(
        Url::parse("urn:rust-analyzer")?,
        Vec::new(),
        vec![DocumentFilter {
            language: Some("rust".to_string()),
            scheme: None,
            pattern: None,
        }],
        params.initialization_options.clone(),
    );

    let arch = match std::env::var("VOLT_ARCH").as_deref() {
        Ok("x86_64") => "x86_64",
        Ok("aarch64") => "aarch64",
        _ => return Ok(()),
    };

    let os = match std::env::var("VOLT_OS").as_deref() {
        Ok("linux") => "unknown-linux-gnu",
        Ok("macos") => "apple-darwin",
        Ok("windows") => "pc-windows-msvc",
        _ => return Ok(()),
    };

    let is_windows = os == "pc-windows-msvc";
    let binary_name = if is_windows {
        format!("rust-analyzer-{}-{}.exe", arch, os)
    } else {
        format!("rust-analyzer-{}-{}", arch, os)
    };
    let archive_name = if is_windows {
        format!("rust-analyzer-{}-{}.zip", arch, os)
    } else {
        format!("rust-analyzer-{}-{}.gz", arch, os)
    };
    let binary_path = PathBuf::from(&binary_name);
    let archive_path = PathBuf::from(&archive_name);

    if !binary_path.exists() {
        let result: Result<()> = {
            let url = format!(
                "https://github.com/rust-lang/rust-analyzer/releases/latest/download/{}",
                archive_name
            );

            let mut resp = Http::get(&url)?;
            let body = resp.body_read_all()?;
            std::fs::write(&archive_path, body)?;

            if is_windows {
                let file = File::open(&archive_path)?;
                let mut zip = ZipArchive::new(file)?;
                let mut extracted = false;
                for i in 0..zip.len() {
                    let mut entry = zip.by_index(i)?;
                    if entry.name().ends_with("rust-analyzer.exe") {
                        let mut out = File::create(&binary_path)?;
                        std::io::copy(&mut entry, &mut out)?;
                        extracted = true;
                        break;
                    }
                }
                if !extracted {
                    anyhow::bail!("rust-analyzer.exe not found in zip");
                }
            } else {
                let mut gz = GzDecoder::new(File::open(&archive_path)?);
                let mut out = File::create(&binary_path)?;
                std::io::copy(&mut gz, &mut out)?;
            }

            std::fs::remove_file(&archive_path)?;
            Ok(())
        };

        if result.is_err() {
            PLUGIN_RPC.window_show_message(
                MessageType::ERROR,
                "can't download rust-analyzer, please configure serverPath in settings".to_string(),
            );
            return Ok(());
        }
    }

    let volt_uri = std::env::var("VOLT_URI")?;
    let server_uri = Url::parse(&volt_uri)?.join(&binary_name)?;

    PLUGIN_RPC.start_lsp(
        server_uri,
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
