# rcli - Rust Command Line Tools

English | [简体中文](README_cn.md) |

rcli is a command line tool written in Rust that offers various functionalities, including:
1. Convert CSV to YAML, JSON, TOML formats
2. Support Base64 encoding/decoding
3. Text signing verification
4. HTTP static server

## Installation

```bash
git clone https://github.com/yuxuetr/rcli.git
cd rcli
# Local source installation
cargo install --path .
```

## View Help

```bash
rcli --help
Usage: rcli <COMMAND>

Commands:
  csv      Show CSV, or convert CSV to others
  genpass  Generate random strength password
  base64   Base64 encode/decode
  text     Text sign/verify
  http     HTTP server
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Features List

### CSV Conversion

1. Convert a CSV file to YAML format, default output file `output.yaml` in the current directory

```bash
rcli csv -i assets/test.csv --format yaml
```

2. Convert a CSV file to JSON format, default output file `output.json` in the current directory

```bash
rcli csv -i assets/test.csv --format json
```

### Base64 Encoding/Decoding

1. Base64 Encode
After entering `rcli base64 encode`, input is expected from the command line. For example, input `hello`, then press `Ctrl + D` twice to generate Base64 encoding

```bash
rcli base64 encode
helloaGVsbG8=%
```

2. Base64 Decode

After entering `rcli base64 decode`, input is expected. You can input the already encoded content, for example `aGVsbG8=hello`, then press `Ctrl + D` twice to generate Base64 decoded content

```bash
rcli base64 decode
aGVsbG8=hello
```

### Generate Strong Password

Generate a high-strength password of specified length, evaluating the score of the code. A score of 4 indicates a password with very high strength

```bash
rcli genpass -l 32
```

### Text Signing Generation and Verification

1. Generate the key required for text signing

`format`: Indicates the format of the key, supporting two formats: `blake3` and `ed25519`

`output-dir`: Indicates the output directory for files, such as the fuxtures directory
- The blake3 format will generate the `blake3.txt` file in the `output-dir`
- The ed25519 format will generate the `ed25519.sk` and `ed25519.pk` files in the `output-dir`

```bash
rcli text generate --format blake3 --output-dir fuxtures
```

```bash
rcli text generate --format blake3 --output-dir fuxtures
```

2. Generate Text Signature

Specify the key generated above, and after entering the command, input content like `hello`. Press `Ctrl + D` twice to output the signature string.

If no `format` is specified, it defaults to `blake3`

```bash
rcli text sign -k fuxtures/blake3.txt
hellonhOxI28NvawT1HYkMC4rc9KXzYk2jJFW7PtX_lA4H_8
```

```bash
rcli text sign -k fuxtures/ed25519.sk --format ed25519
hellotOEBY_Hs_E2lQ0zLzcOj41TigJEjZdgtrECfCC_Z9RWX-jnkLbKleVpwsbLjISM_gyUKPFrrF7hqBimvI1shBg
```

3. Verify Signature

The default format is blake3, use the `sig` parameter to specify the text signed by the key. After execution, the command line awaits input. Input `hello` to verify if the signature of `sig` is the signature of `hello`. Output **Signature verified** indicates successful verification

```bash
rcli text verify -k fuxtures/blake3.txt --sig nhOxI28NvawT1HYkMC4rc9KXzYk2jJFW7PtX_lA4H_8
helloSignature verified
```

```bash
rcli text verify -k fuxtures/ed25519.pk --format ed25519 --sig tOEBY_Hs_E2lQ0zLzcOj41TigJEjZdgtrECfCC_Z9RWX-jnkLbKleVpwsbLjISM_gyUKPFrrF7hqBimvI1shBg
helloSignature verified
```

### Local Static Server

Serve a specified directory as a static web service.
- `dir`: Specifies the directory, default is the current directory where the command is executed.
- `port`: Web service port, default is 8009.

1. Start the service

```bash
rcli http serve
```

2. Access the service

```bash
# In the rcli project directory
curl http://127.0.0.1:8009/Cargo.toml
```
The displayed content is as follows:
```toml
[package]
name = "rcli"
version = "0.1.0"
edition = "2021"
author = ["xueTr54<jinpeng.ti@gmail.com>"]
license = "MIT"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
anyhow = "1.0.82"
axum = { version = "0.7.5", features = ["http2", "query", "tracing"] }
base64 = "0.22.1"
blake3 = "1.5.1"
clap = { version = "4.5.4", features = ["derive"] }
csv = "1.3.0"
ed25519-dalek = { version = "2.1.1", features = ["rand_core"] }
enum_dispatch = "0.3.13"
rand = "0.8.5"
serde = { version = "1.0.200", features = ["derive"] }
serde_json = "1.0.116"
serde_yaml = "0.9.34"
tokio = { version = "1.37.0", features = ["rt", "rt-multi-thread", "net", "fs", "macros"] }
toml = "0.8.12"
tower-http = { version = "0.5.2", features = ["compression-full", "cors", "trace", "fs"] }
tracing = "0.1.40"
tracing-subscriber = { version = "0.3.18", features = ["env-filter
```
