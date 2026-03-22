<div align="center">
    <h1>assembler</h1>
    <p>A Rust to Lua transpiler for Factorio's modding api</p>

## Testing

Integration tests parse the real Factorio API files. Point `FACTORIO_HOME` at your Factorio install directory and they will be picked up automatically:

```sh
# Linux (Steam)
FACTORIO_HOME=~/.steam/steam/steamapps/common/Factorio cargo test

# macOS (Steam)
FACTORIO_HOME=~/Library/Application\ Support/Steam/steamapps/common/Factorio cargo test

# Windows (Steam, PowerShell)
$env:FACTORIO_HOME="C:\Program Files (x86)\Steam\steamapps\common\Factorio"; cargo test
```

The JSON files are read from `$FACTORIO_HOME/doc-html/runtime-api.json` and `$FACTORIO_HOME/doc-html/prototype-api.json`. Integration tests are silently skipped when `FACTORIO_HOME` is not set.

</div>
