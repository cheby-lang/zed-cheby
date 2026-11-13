//! Zed extension for Cheby: starts `cheby lsp` as the language server.
//!
//! The command comes from `lsp.cheby.binary.path` and `lsp.cheby.binary.arguments` in Zed's
//! settings, else `cheby` on the worktree's PATH with the argument `lsp`.

use zed_extension_api::{
    self as zed, LanguageServerId, Result, Worktree, serde_json::Value, settings::LspSettings,
};

/// The server's name in `extension.toml` and the key under `lsp` in Zed's settings.
const SERVER: &str = "cheby";

struct Cheby;

impl zed::Extension for Cheby {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let binary = LspSettings::for_worktree(SERVER, worktree)
            .ok()
            .and_then(|s| s.binary);
        let command = binary
            .as_ref()
            .and_then(|b| b.path.clone())
            .or_else(|| worktree.which("cheby"))
            .ok_or("`cheby` is not on PATH; set lsp.cheby.binary.path")?;
        let args = binary
            .as_ref()
            .and_then(|b| b.arguments.clone())
            .unwrap_or_else(|| vec!["lsp".to_owned()]);
        let mut env = worktree.shell_env();
        if let Some(extra) = binary.and_then(|b| b.env) {
            env.extend(extra);
        }
        Ok(zed::Command { command, args, env })
    }

    fn language_server_initialization_options(
        &mut self,
        _id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        Ok(LspSettings::for_worktree(SERVER, worktree)
            .ok()
            .and_then(|s| s.initialization_options))
    }

    /// Zed answers `workspace/configuration` by looking the section up in this value, so the
    /// settings sit under the `cheby` key.
    fn language_server_workspace_configuration(
        &mut self,
        _id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        Ok(LspSettings::for_worktree(SERVER, worktree)
            .ok()
            .and_then(|s| s.settings)
            .map(|settings| zed::serde_json::json!({ SERVER: settings })))
    }
}

zed::register_extension!(Cheby);
