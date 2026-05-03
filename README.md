# nullapt SDK

Language SDKs for building [nullapt](https://github.com/nullapt/nullapt) WASM skills.

## Packages

| Language | Package | Status |
|---|---|---|
| Rust | [`nullapt-sdk`](./rust) (crates.io) | stable |
| TypeScript/Node | [`@nullapt/sdk`](./typescript) (npm) | stable |
| Go | [`github.com/nullapt/sdk/go`](./go) (pkg.go.dev) | beta |

---

## Rust

```toml
# Cargo.toml
[dependencies]
nullapt-sdk = "0.1"
extism-pdk = "1"
```

```rust
use nullapt_sdk::prelude::*;

#[derive(Deserialize)]
struct Input { query: String }

#[derive(Serialize)]
struct Output { result: String }

#[plugin_fn]
pub fn my_tool(input: Json<Input>) -> FnResult<Json<Output>> {
    Ok(Json(Output { result: format!("got: {}", input.query) }))
}
```

Build:

```bash
cargo build --target wasm32-wasi --release
cp target/wasm32-wasi/release/my_skill.wasm skill.wasm
```

---

## TypeScript

```bash
npm install @nullapt/sdk
```

```typescript
import { validateManifest } from '@nullapt/sdk';
import skillJson from './SKILL.json';

validateManifest(skillJson); // throws if invalid
```

---

## Go (TinyGo)

```bash
go get github.com/nullapt/sdk/go
```

```go
package main

import "github.com/nullapt/sdk/go/pdk"

//export my_tool
func myTool() int32 {
    input := pdk.Input()
    pdk.Output([]byte("result: " + string(input)))
    return 0
}
```

Build:

```bash
tinygo build -o skill.wasm -target wasi .
```

---

## Contributing

See [CONTRIBUTING.md](https://github.com/nullapt/nullapt/blob/main/CONTRIBUTING.md).
