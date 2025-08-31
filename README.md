# BLE ESP32 Template

a cargo-generate template for an esp32 project with bluetooth low energy (nimble) and ws2812 smart led control.

### Prerequisites

- Rust with ESP-IDF support installed
- [cargo-generate](https://github.com/cargo-generate/cargo-generate)

### Usage

to create a new project from this template:

```bash
cargo generate --git https://github.com/plahutazan/ble_esp_template.git --name my-project
```

---

if esp32s3
``` rust-toolchain.toml
[toolchain]
channel = "esp"
```

if esp32c3
``` rust-toolchain.toml
[toolchain]
channel = "nightly"
components = ["rust-src"]
---

###### made by
##### PRAZVAL