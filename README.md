# zed-cheby

Cheby support for [Zed](https://zed.dev): syntax highlighting, the outline, brackets and indentation from the [tree-sitter-cheby](https://github.com/cheby-lang/tree-sitter-cheby) grammar, and the Cheby language server, `cheby lsp`.

The extension is not in Zed's extension registry yet. Install it as a dev extension, as below.

## Install

1. Install `cheby` and put it on your `PATH`. From a checkout of [cheby](https://github.com/cheby-lang/cheby):

   ```sh
   cargo install --path crates/cheby_cli --locked
   ```

2. Clone this repository. Zed builds the extension from source, which needs Rust installed through `rustup`.
3. In Zed, open the Extensions page (`cmd-shift-x`), press Install Dev Extension and choose the clone.
4. Open a Cheby package. The server starts with the first `.cheby` file you open.

After you pull a new version, press Rebuild on Cheby in the Extensions page. If the build fails, `zed: open log` in the command palette shows why.

## The language server

The extension runs `cheby lsp`, with `cheby` found on the `PATH` of the project's shell. It shows the diagnostics `cheby check` reports as you type, and gives hover, go to definition, references, call hierarchy, workspace symbols (`cmd-t`), name colours, completion, signature help, inlay hints and formatting. Hover shows the values of constants, which the server evaluates when the package first checks clean and after each save.

To run another binary, or to change the server's settings, add a `cheby` entry under `lsp` in Zed's `settings.json` (`cmd-,`):

```json
{
  "lsp": {
    "cheby": {
      "binary": {
        "path": "/path/to/cheby",
        "arguments": ["lsp"]
      },
      "settings": {
        "analysis": { "debounceMs": 100 },
        "inlayHints": { "parameterNames": true }
      }
    }
  },
  "inlay_hints": { "enabled": true }
}
```

- `binary.path` names the `cheby` to run. Without it, the extension looks for `cheby` on `PATH`. `binary.arguments` defaults to `["lsp"]`, and `binary.env` adds variables to the server's environment.
- `settings` is the server's `cheby` object. The extension also passes `initialization_options` through, with the same shape. A key in `settings` wins over the same key there.
- Zed shows no inlay hints until `inlay_hints.enabled` is `true`.

The settings the server serves today:

| Setting                         | Default | What it does                                                           |
| ------------------------------- | ------- | ---------------------------------------------------------------------- |
| `analysis.target`               | `null`  | Check for `"native"` or `"js"` only, as `cheby check --target` does    |
| `analysis.debounceMs`           | `40`    | The quiet time after an edit before the server checks again, 0 to 1000 |
| `analysis.requestWaitMs`        | `10000` | How long a request waits for the check of its text, 100 to 60000       |
| `completion.autoImport`         | `true`  | Offer the items of modules the file does not import, with the import   |
| `completion.typeDriven`         | `true`  | Rank items of the expected type first, and offer functions after `xs.` |
| `inlayHints.enable`             | `true`  | Every inlay hint                                                       |
| `inlayHints.letTypes`           | `true`  | The type after a `let` of one name                                     |
| `inlayHints.patternBinders`     | `true`  | The type after each name of a destructuring `let`                      |
| `inlayHints.useBinders`         | `true`  | The type after each `use` binder                                       |
| `inlayHints.closureTypes`       | `true`  | A closure's parameter and result types                                 |
| `inlayHints.localFnTypes`       | `true`  | A local `fn`'s parameter and result types                              |
| `inlayHints.pipeStages`         | `true`  | The type after each stage of a multi-line pipeline                     |
| `inlayHints.dyn`                | `true`  | An implicit `dyn` conversion                                           |
| `inlayHints.parameterNames`     | `false` | `name: ` before a literal argument                                     |
| `constants.evaluate`            | `true`  | Evaluate constants, for hover and the back-end diagnostics             |
| `constants.childTimeoutSeconds` | `60`    | The time limit of one evaluation, 1 to 3600                            |
| `semanticTokens.enabled`        | `true`  | Colour names by what they are, over tree-sitter's highlighting         |

The server also accepts `diagnostics.explainLinks`, `inlayHints.constants`, `lenses.*`, `ownership.*` and `clientCommands`, which do nothing yet. A value of the wrong type is ignored with a warning in the server's log, and the key keeps its earlier value. To read the server's log, search the command palette for `language server logs`.

## Tips

- If no list appears after `::`, as after accepting a module's name, type the first letter of the item you want, and the module's items are listed.
- Zed asks for no signature while a completion's placeholders are active. Run `editor: show signature help` (`editor::ShowSignatureHelp`) from the command palette, or bind it to a key, to show it then.
- The outline (`cmd-shift-o`) comes from this extension's tree-sitter queries, not from the server.

## Known issues

- The `std/*.cheby` files of the cheby repository show false errors when opened: the server checks them as single files, not as the standard library.
- An unpaired `"` while typing marks string errors down the rest of the file until it is closed.
- On Windows, constants are not evaluated until Cheby's M6, so hover shows no values and the back-end diagnostics of `cheby check` do not appear.
