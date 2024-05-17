# 🅷📦 - hbox

[![Crates.io](https://img.shields.io/crates/v/hbox?style=flat-square)](https://crates.io/crates/hbox)
[![Crates.io](https://img.shields.io/crates/d/hbox?style=flat-square)](https://crates.io/crates/hbox)
[![codecov](https://codecov.io/gh/helton/hbox/graph/badge.svg?token=F8INGHDIUS)](https://codecov.io/gh/helton/hbox)
[![License](https://img.shields.io/github/license/helton/hbox.svg)](https://github.com/helton/hbox/blob/main/LICENSE)
[![Docs](https://docs.rs/hbox/badge.svg)](http://docs.rs/hbox)

hbox is a Command Line Interface (CLI) that leverages container technology to manage packages, powered by Rust 🦀.

## Features

hbox offers the following features:

- **Container Isolation**: hbox uses containers to isolate packages, allowing multiple versions of a package to coexist without conflict.
- **Robust Configuration Options**: hbox enables high customization through configuration files. You can define package aliases and setup automatic volume mounts via `config.json`.
- **Support for Pipes**: hbox supports the use of pipes in `hbox run`, which allows you to chain commands efficiently.
- **Convenient Shims**: hbox creates `shims` (alias shortcuts) for all installed packages, simplifying command entry from `hbox run <package alias> <commands>` to `<package alias> <commands>`.
- **Accessible Internal Binaries**: hbox has the ability to provide direct access to internal binaries within images. Users can override the default entrypoint, meaning essential tools and utilities within containers can be accessed directly. This feature further expands the capabilities of hbox `shims`, making it even more convenient to launch and utilize container tools.
- **Customizable Environment Variables**: hbox supports setting environment variables for each package, enabling finer control over runtime configurations.

## Commands

```sh
> hbox help
CLI tool that leverages container technology to manage packages.

Usage: hbox <COMMAND>

Commands:
  info    Print debug information
  list    List all installed packages and their versions
  add     Add a specific version of a package
  remove  Remove a package
  use     Set current version of a package
  run     Run the package
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Installation

To install hbox via `cargo`, run the following command:

```sh
cargo install hbox
```

## Setup

### Shims and Shell Configuration

hbox utilizes shims and a configuration file to effectively manage your installed packages. For the successful addition of `$HBOX_DIR/shims` at the correct priority level to your path, these lines of code should be added to your `.bashrc` or `.zshrc` file:

```sh
export HBOX_DIR="$HOME/.hbox"
export PATH="$HBOX_DIR/shims":$PATH
```

If you installed hbox via `cargo` the `hbox` binary should already be available on your `PATH` env var when the shims are executed.

### Package Registry/Index via index.json

The registry/index of packages in hbox is managed by the `$HBOX_DIR/index.json` file (example below).  This file is intended to keep information about usual package configuration.
In the future this will be centralized in on its own repo/server, so you can fetch it on demand.

```json
{
  "packages": {
    "busybox": {
      "image": "docker.io/busybox",
      "volumes": [
        {
          "source": ".",
          "target": "/app"
        }
      ],
      "current_directory": "/app",
      "binaries": [
        {
          "name": "tree",
          "path": "/bin/tree"
        }
      ],
      "only_shim_binaries": true,
      "environment_variables": [
        {
          "name": "FOO",
          "value": "abc123"
        },
        {
          "name": "HTTP_PROXY",
          "value": "$HTTP_PROXY"
        },
        {
          "name": "HTTPS_PROXY",
          "value": "$HTTPS_PROXY"
        },
        {
          "name": "NO_PROXY",
          "value": "$NO_PROXY"
        }
      ]
    },
    "golang": {
      "image": "docker.io/golang",
      "volumes": [
        {
          "source": ".",
          "target": "/app"
        }
      ],
      "current_directory": "/app",
      "binaries": [
        {
          "name": "go",
          "path": "/usr/local/go/bin/go"
        },
        {
          "name": "gofmt",
          "path": "/usr/local/go/bin/gofmt"
        }
      ],
      "only_shim_binaries": true
    },
    "curl": {
      "image": "docker.io/curlimages/curl"
    },
    "aws": {
      "image": "docker.io/amazon/aws-cli",
      "volumes": [
        {
          "source": "~/.aws",
          "target": "/root/.aws"
        }
      ],
      "current_directory": "/root/.aws"
    },
    "lambda_python": {
      "image": "public.ecr.aws/lambda/python"
    },
    "jq": {
      "image": "ghcr.io/jqlang/jq"
    },
    "terraform": {
      "image": "docker.io/hashicorp/terraform"
    },
    "opa": {
      "image": "docker.io/openpolicyagent/opa"
    },
    "fga": {
      "image": "docker.io/openfga/cli"
    }
  }
}
```

For now you can use the `index.json` to also override the registry of any container image. By default, we pull from `docker.io` if no configuration is found for a given package.
In the future is planned to split the override configuration from the common index/registry.

### Configuration via config.json

The general configuration of hbox is managed by the `$HBOX_DIR/config.json` file:

```json
{
  "debug": false
}
```

### Package Version Management via versions.json

hbox also creates and maintains a `$HBOX_DIR/versions.json` file that keeps track of the current version of each package. This file is under the management of hbox itself and shouldn't be manually edited:

```json
{
  "packages": [
    {
      "name": "aws",
      "versions": [
        "latest"
      ],
      "current": "latest"
    },
    {
      "name": "jq",
      "versions": [
        "latest",
        "1.7rc2"
      ],
      "current": "1.7rc2"
    },
    {
      "name": "node",
      "versions": [
        "latest",
        "14",
        "15"
      ],
      "current": "15"
    }
  ]
}
```

## Usage

Below are some examples demonstrating how you can use `hbox`:

```sh
> hbox version
0.1.1
> hbox list
> hbox add jq
latest: Pulling from jqlang/jq
...
Added 'jq' version latest.
> hbox list jq
- jq:
  - latest ✔
> jq --version
jq-1.7.1
> hbox add node latest
latest: Pulling from library/node
...
Added 'node' version latest.
> hbox list
- jq:
  - latest ✔
- node:
  - latest ✔
> hbox list node
- node:
  - latest ✔
> node --version
v22.0.0
> hbox add node 14 --set-default
'node' version 14 set as default.
14: Pulling from library/node
...
Added 'node' version 14.
> hbox list node
- node:
  - 14 ✔
  - latest
> node --version
v14.21.3
> hbox use node latest
'node' set to version latest
> node --version
v22.0.0
> hbox list node
- node:
  - 14
  - latest ✔
```

These examples should provide a quick start guide for you to understand the basic operations that you can perform with hbox.

If you want to see my ideas for the future of the project, check out the [ROADMAP](ROADMAP.md).
