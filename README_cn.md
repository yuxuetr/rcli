# rcli - Rust Command Line Tools

rcli是一个用Rust编写的命令行工具，提供多种功能，包括:

1. CSV转换为YAML,JSON,TOML格式
2. 支持Base64编码/解码
3. 文本签名验证
4. HTTP静态服务器

## 安装

```bash
git clone https://github.com/yuxuetr/rcli.git
cd rcli
# 本地源码安装
cargo install --path .
```

## 查看帮助

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

## 功能列表

### CSV转换功能

1. 将CSV文件转换为YAML格式, 默认在当前目录下生成转换后的文件`output.yaml`

  ```bash
  rcli csv -i assets/test.csv --format yaml
  ```

2. 将CSV文件转换为JSON格式，默认在当前目录下生成转换后的文件`output.json`

  ```bash
  rcli csv -i assets/test.csv --format json
  ```

### Base64编解码

1. Base64编码
   输入`rcli base64 encode`后是等待命令行输入，例如输入`hello`, 然后按两下`Ctrl + D`会生成Base64编码

  ```bash
  rcli base64 encode
  helloaGVsbG8=%
  ```

2. Base64解码

   输入`rcli base64 decode`后等待输入，可以将已经编码过的内容输入，例如`aGVsbG8=hello`，然后按两下
   `Ctrl + D`会生成Base64解码的内容

  ```bash
  rcli base64 decode
  aGVsbG8=hello
  ```

### 生成高强度密码

生成指定长度的高强度密码，会评估代码的分数，分数是4是强度特别高的密码

  ```bash
  rcli genpass -l 32
  ```

### 文本签名生成与验证

1. 生成文本签名所需的key
   - `format`: 表示key的格式, 有两种格式支持: `blake3`和`ed25519`
   - `output-dir`: 表示输出文件的目录，如: `fuxtures`目录
   - `blake3`格式会`output-dir`生成`blake3.txt`文件
   - `ed25519`格式会在`output-dir`生成`ed25519.sk`和`ed25519.pk`

  ```bash
  rcli text generate --format blake3 --output-dir fuxtures
  ```

  ```bash
  rcli text generate --format blake3 --output-dir fuxtures
  ```

2. 生成文本签名

指定上面生成的key后，出现命令等待输入界面，输入内容如`hello`，按两下`Ctrl + D`会输出签名字符串。

如果不指定`format`则，默认`blake3`

  ```bash
  rcli text sign -k fuxtures/blake3.txt
  hellonhOxI28NvawT1HYkMC4rc9KXzYk2jJFW7PtX_lA4H_8
  ```

  ```bash
  rcli text sign -k fuxtures/ed25519.sk --format ed25519
  hellotOEBY_Hs_E2lQ0zLzcOj41TigJEjZdgtrECfCC_Z9RWX-jnkLbKleVpwsbLjISM_gyUKPFrrF7hqBimvI1shBg
  ```

3. 验证签名

默认格式是blake3，使用`sig`参数指定经过key签名后的文本，执行后是命令行等待输入窗口，
输入`hello`会验证sig的签名是否是hello的签名，输出**Signature verified**则表明验证成功

  ```bash
  rcli text verify -k fuxtures/blake3.txt --sig nhOxI28NvawT1HYkMC4rc9KXzYk2jJFW7PtX_lA4H_8
  helloSignature verified
  ```

  ```bash
  rcli text verify -k fuxtures/ed25519.pk --format ed25519 --sig tOEBY_Hs_E2lQ0zLzcOj41TigJEjZdgtrECfCC_Z9RWX-jnkLbKleVpwsbLjISM_gyUKPFrrF7hqBimvI1shBg
  helloSignature verified
  ```

### 本地静态服务器

将指定目录作为静态服务Web服务

- `dir`: 参数指定目录，默认是当前目录，也就是命令执行所在目录
- `port`: Web服务端口，默认8009

1. 启动服务

  ```bash
  rcli http serve
  ```

2. 访问服务

  ```bash
  # 在rcli项目目录下
  curl http://127.0.0.1:8009/Cargo.toml
  ```

显示内容如下:

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
tracing-subscriber = { version = "0.3.18", features = ["env-filter"] }
zxcvbn = "2.2.2"
```

```bash
# 在$HOME目录下有个`planning.org`文件
curl http://127.0.0.1:8009/planning.org
```

显示内容如下:

```orgmode
#+title: Planning

+ Rust AIBot
  - Rust Gemini RESTful API[https://ai.google.dev/tutorials/rest_quickstart]
  - Rust docx parser[https://docs.rs/docx/latest/docx/]

+ Moble LLM Lite
  - MobileDiffusion[https://blog.research.google/2024/01/mobilediffusion-rapid-text-to-image.html?utm_source=substack&utm_medium=email]
```
