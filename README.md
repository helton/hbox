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

### Package Registry/Index

The registry/index of packages in hbox is managed in the `$HBOX_DIR/index` directory.
Inside there a sharded structure where every package as an individual file is intended to keep information about usual package configuration.
In the future this will be centralized in on its own repo/server, so you can fetch it on demand.

Example of a `$HBOX_DIR/index/g/golang.json`:

```json
{
  "image": "docker.io/golang",
  "volumes": [
    {
      "source": ".",
      "target": "//app"
    }
  ],
  "current_directory": "//app",
  "binaries": [
    {
      "name": "go",
      "path": "//usr/local/go/bin/go"
    },
    {
      "name": "gofmt",
      "path": "//usr/local/go/bin/gofmt"
    }
  ],
  "only_shim_binaries": true
}
```

### Override configurations

If you want to override configurations of a package, don't change the `$HBOX_DIR/index` folder, but create an override file inside `$HBOX_DIR/overrides`.
This directory is not sharded, so you can just place your `<package>.json` files there.
As the name implies, the override configuration will take precedence over the index ones. The files contents won't be merged in memory.

Example of a `$HBOX_DIR/overrides/busybox.json`:

```json
{
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
    },
    {
      "name": "mysh",
      "path": "/bin/sh",
      "cmd": ["-c"],
      "wrap_args": true
    }
  ],
  "only_shim_binaries": true,
  "environment_variables": [
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
}
```

In the example above, we defined a `mysh` as a binary pointing to `/bin/sh` inside the `busybox` image.
Now we can use it like this:

```sh
> mysh ls -alh
```

Also note that we defined `-c` as the default command, and we also defined that all args should be wrapped in quotes.
Under the hood the full command executed will be like this:

```sh
docker run -it --rm --name hbox-busybox-latest-qNDyEVzrUb -v .://app -w //app -e HTTP_PROXY=$HTTP_PROXY -e HTTPS_PROXY=$HTTPS_PROXY -e NO_PROXY=$NO_PROXY --entrypoint //bin/sh docker.io/busybox:latest -c "ls -alh"
```

### Configuration via config.json

The general configuration of hbox is managed by the `$HBOX_DIR/config.json` file.
For now you control how logs are used and enable some experimental features:

```json
{
  "logs": {
    "enabled": true,
    "level": "debug",
    "strategy": "truncate"
  },
  "experimental": {
    "capture_stdout": false,
    "capture_stderr": false
  }
}
```

### Package Version Management

hbox also creates and maintains a directory `$HBOX_DIR/versions` that keeps track of the current version of each package.
Every package has a file there and this is under the management of hbox itself and shouldn't be manually edited.

Example of a `$HBOX_DIR/versions/node.json`

```json
{
  "name": "node",
  "versions": [
    "latest",
    "14",
    "15"
  ],
  "current": "15"
}
```

### Logs

If you enable logs in you `$HBOX_DIR/config.json` file, your logs will appear in the `$HBOX_DIR/logs` folder.
Use this to see what commands are executed under the hood that aren't displayed normally for the user.

**Note**: Be extra careful when sharing your logs, they may contain information you might not want to share, like api keys, environment variables, etc.

## Usage

Below are some examples demonstrating how you can use `hbox`:

```sh
> hbox version
0.4.0
> hbox info
[System Information]
OS Details:
  Name           : linux
  Architecture   : x86_64
  Family         : unix

[Application Configuration]
Version          : 0.4.0
Directories and Files:
  base dir       : /home/helton/.hbox
  config file    : /home/helton/.hbox/config.json
  overrides dir  : /home/helton/.hbox/overrides
  versions dir   : /home/helton/.hbox/versions
  logs dir       : /home/helton/.hbox/logs
  shims dir      : /home/helton/.hbox/shims
  index dir      : /home/helton/.hbox/index
Environment Vars:
  HBOX_DIR       : /home/helton/.hbox
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
Added 'node' version 'latest'. Current version is 'latest'.
> hbox list
- [jq]
  - versions
    - latest ✔
- [node]
  - versions
    - latest ✔
> hbox list node
- [node]
  - versions
    - latest ✔
> node --version
v22.0.0
> hbox add node 14 --set-default
14: Pulling from library/node
...
Added 'node' version '14'. Current version is '14'.
> hbox list node
- [node]
  - versions
    - 14 ✔
    - latest
> node --version
v14.21.3
> hbox use node latest
Package 'node' set to version 'latest'
> node --version
v22.0.0
> hbox list node
- [node]
  - versions
    - 14
    - latest ✔
```

These examples should provide a quick start guide for you to understand the basic operations that you can perform with hbox.

If you want to see my ideas for the future of the project, check out the [ROADMAP](ROADMAP.md).
