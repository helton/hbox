# 🅷📦 - hbox

[![Crates.io](https://img.shields.io/crates/v/hbox?style=flat-square)](https://crates.io/crates/hbox)
[![Crates.io](https://img.shields.io/crates/d/hbox?style=flat-square)](https://crates.io/crates/hbox)
[![codecov](https://codecov.io/gh/helton/hbox/graph/badge.svg?token=F8INGHDIUS)](https://codecov.io/gh/helton/hbox)
[![License](https://img.shields.io/github/license/helton/hbox.svg)](https://github.com/helton/hbox/blob/main/LICENSE)
[![Docs](https://docs.rs/hbox/badge.svg)](http://docs.rs/hbox)

hbox is a Command Line Interface (CLI) that leverages container technology to manage packages, powered by Rust 🦀.

## Features

hbox offers the following features:

- **Container Isolation**: Uses containers to isolate packages, allowing multiple versions to coexist without conflict.
- **Robust Configuration Options**: Enables high customization through configuration files, allowing you to define package aliases and set up automatic volume mounts via `config.json`.
- **Support for Pipes**: Supports the use of pipes in `hbox run`, enabling efficient command chaining.
- **Convenient Shims**: Creates `shims` (alias shortcuts) for all installed packages, simplifying command entry from `hbox run <package alias> <commands>` to `<package alias> <commands>`.
- **Accessible Internal Binaries**: Provides direct access to internal binaries within images. Users can override the default entrypoint to access essential tools and utilities within containers directly.
- **Customizable Environment Variables**: Allows setting environment variables for each package, enabling finer control over runtime configurations.]()
- **Support for Docker and Podman**: Provides seamless support for both Docker and Podman container engines, offering flexibility in container runtime choices.

## Installation

To install hbox via `cargo`, run the following command:

```sh
cargo install hbox
```

## Setup

To ensure all hbox binaries and shims can be found, add these lines to your `.bashrc` or `.zshrc` file:

```sh
export HBOX_DIR="$HOME/.hbox"
export PATH="$HBOX_DIR/shims":$PATH
```

If you installed hbox via `cargo`, the `hbox` binary should already be available in your `PATH` environment variable when the shims are executed.

## Commands

```sh
> hbox help
CLI tool that leverages container technology to manage packages.

Usage: hbox <COMMAND>

Commands:
  info    Print debug information about the hbox environment and configuration
  list    List all installed packages and their versions
  add     Add and install a specific version of a package
  remove  Remove a specific version of a package
  use     Set the current version of a package as the default
  run     Run a command from a package
  config  Configure hbox settings
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Usage

Below are some examples demonstrating how you can use `hbox`:

```sh
> hbox version
0.6.0
> hbox info
[System Information]
OS Details:
  Name           : linux
  Architecture   : x86_64
  Family         : unix

[Application Configuration]
Version          : 0.6.0
Engine           : docker
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

## Configuration

### Configuration via config.json

The general configuration of hbox is managed by the `$HBOX_DIR/config.json` file. Currently, you can control the container engine, how logs are used, and enable some experimental features:

```json
{
  "engine": "docker",
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

You can edit this file manually or use the `hbox config` command to update it programmatically. The `hbox config` command provides a convenient way to read and write configuration settings without directly editing the JSON file. You can use `hbox config` or its alias `hbox configure`.

#### Examples:

- **Set a configuration value**:
  ```sh
  hbox config logs.enabled false
  hbox config logs.level debug
  hbox config engine podman
  ```

- **Get a configuration value**:
  ```sh
  hbox config logs.enabled
  hbox config engine
  ```

The command will either update the configuration file or print the current value of a configuration setting, depending on whether a value is provided.

By using the `hbox config` command, you can ensure that your configuration changes are applied correctly without the risk of introducing syntax errors into the JSON file.

#### Properties

The `config.json` file is used to control how hbox should behave. Below are the details of each property available in this configuration file.

| Property                      | Type      | Description                                                                                                                                           |
|-------------------------------|-----------|-------------------------------------------------------------------------------------------------------------------------------------------------------|
| `engine`                      | `string`  | Indicates what container engine to use. Possible values: `docker`, `podman`. Example: `docker`                                                        |
| `logs`                        | `object`  | Configuration for logging behavior.                                                                                                                   |
| `logs.enabled`                | `boolean` | Indicates if logging is enabled. Example: `true`                                                                                                      |
| `logs.level`                  | `string`  | Specifies the logging level. Possible values: `debug`, `info`, `warn`, `error`. Example: `debug`                                                      |
| `logs.strategy`               | `string`  | Strategy for handling log files. Possible values: `append`, `truncate`. Example: `truncate`                                                           |
| `experimental`                | `object`  | Configuration for experimental features.                                                                                                              |
| `experimental.capture_stdout` | `boolean` | Indicates if standard output should be captured and sent to logs. Regardless of this option, stdout will always be printed normally. Example: `false` |
| `experimental.capture_stderr` | `boolean` | Indicates if standard error should be captured and sent to logs. Regardless of this option, stderr will always be printed normally. Example: `false`  |

#### Property Details

- **engine**: Specifies whether to use `docker` or `podman` as a container engine. `docker` is the default.

- **logs**: This object configures logging behavior for hbox.
    - `enabled`: A boolean indicating if logging is enabled. If set to `true`, logging is active.
    - `level`: Specifies the verbosity of the logs. Options include:
        - `debug`: Detailed information typically useful for developers.
        - `info`: General information about the application's operation.
        - `warn`: Warnings about potentially problematic situations.
        - `error`: Error messages indicating serious issues.
    - `strategy`: Determines how log files are managed. Options include:
        - `append`: Adds new log entries to the end of existing log files.
        - `truncate`: Overwrites existing log files with new entries.

- **experimental**: This object contains settings for experimental features that are not yet fully supported.
    - `capture_stdout`: A boolean indicating if the standard output of commands should be captured and sent to logs. Regardless of this option, stdout will always be printed normally.
    - `capture_stderr`: A boolean indicating if the standard error of commands should be captured and sent to logs. Regardless of this option, stderr will always be printed normally.

These properties allow you to customize the behavior of hbox, particularly how it handles logging and experimental features, providing better control over the application's operation.

### Shims

hbox utilizes shims to effectively manage your installed packages. There are two types of shims: _package shims_ and _binary shims_.

#### Package Shims

For normal packages, the shim will call `hbox run` with the package name and any provided arguments. Example shim for a package named `node`:

```sh
#!/bin/sh
hbox run node "$@"
```

#### Binary Shims

For binaries within a package, the shim will include both the package and the binary name. This informs hbox which package and binary to use. Example shim for a binary `mysh` within the `busybox` package:

```sh
#!/bin/sh
hbox run busybox::mysh "$@"
```

### Package Registry/Index

The registry/index of packages in hbox is managed in the `$HBOX_DIR/index` directory. This directory has a sharded structure where each package has an individual file to store configuration information. In the future, this will be centralized in its own repository/server, allowing you to fetch it on demand.

#### Properties

The configuration below is for index (`$HBOX_DIR/index/<shard>/<package>.json`) and override (`$HBOX_DIR/overrides/<package>.json`) files:

| Property                | Type      | Description                                                                                                                                                                                                                                      |
|-------------------------|-----------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `image`                 | `string`  | The Docker image to be used for the package. Example: `"docker.io/busybox"`                                                                                                                                                                      |
| `ports`                 | `array`   | An array of port mappings for the container. Each port mapping has a `host` and `container`. Example: `[{"host": 8090, "container": 8091}, {"host": 8091, "container": 8092}]`                                                                   |
| `volumes`               | `array`   | An array of volume mappings for the container. Each volume mapping has a `source` and `target`. Example: `[{"source": ".", "target": "/app"}]`                                                                                                   |
| `current_directory`     | `string`  | The working directory inside the container. Example: `"/app"`                                                                                                                                                                                    |
| `binaries`              | `array`   | An array of binaries available in the container. Each binary has a `name`, `path`, and optional `cmd` and `wrap_args`. Example: `[{"name": "tree", "path": "/bin/tree"}, {"name": "mysh", "path": "/bin/sh", "cmd": ["-c"], "wrap_args": true}]` |
| `only_shim_binaries`    | `boolean` | Indicates if only the binaries in the package configuration should have shims created. Example: `true`                                                                                                                                           |
| `environment_variables` | `array`   | An array of environment variables to be set in the container. Each variable has a `name` and `value`. Example: `[{"name": "foo", "value": "$foo"}, {"name": "bar", "value": "$bar"}]`                                                            |

#### Property Details

- **image**: Specifies the Docker image to be used. This is a required property for defining the container image from which the package will be run.

- **ports**: Defines the port mappings between the host and the container. Each port mapping includes:
    - `host`: The port on the host machine.
    - `container`: The port inside the container.

- **volumes**: Defines the volume mappings between the host and the container. Each volume mapping includes:
    - `source`: The path on the host machine.
    - `target`: The path inside the container.

- **current_directory**: Sets the working directory inside the container. This is where commands will be executed by default.

- **binaries**: Lists the binaries available within the container. Each binary includes:
    - `name`: The name of the binary.
    - `path`: The path to the binary inside the container.
    - `cmd` (optional): An array of default command arguments.
    - `wrap_args` (optional): A boolean indicating if the arguments should be wrapped in quotes.

- **only_shim_binaries**: Indicates if only the binaries in the package configuration should have shims created. By default, a shim is created for the package name. If enabled, only the binaries will have shims. 

- **environment_variables**: Specifies environment variables to be set in the container. Each variable includes:
    - `name`: The name of the environment variable.
    - `value`: The value of the environment variable, which can reference host environment variables.

These properties allow you to customize how each package is run within its container, providing flexibility and control over the runtime environment.

#### Example

Example of a `$HBOX_DIR/index/g/golang.json`:

```json
{
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
}
```

### Override Configurations

To override package configurations, create an override file inside `$HBOX_DIR/overrides` instead of modifying the `$HBOX_DIR/index` folder. This directory is not sharded, so you can directly place your `<package>.json` files there. The override configurations will take precedence over the index configurations, and the contents will not be merged in memory.

For the properties of the override configuration, check [Package Registry/Index](README.md#package-registryindex).

#### Example

Example of a `$HBOX_DIR/overrides/busybox.json`:

```json
{
  "image": "docker.io/busybox",
  "ports": [
    {
      "host": 8090,
      "container": 8000
    },
    {
      "host": 8091,
      "container": 8001
    }
  ],  
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

In the example above, we define `mysh` as a binary pointing to `/bin/sh` inside the `busybox` image. You can use it like this:

```sh
> mysh ls -alh
```

We also set `-c` as the default command and specified that all arguments should be wrapped in quotes. The full command executed will be:

```sh
docker run -it --rm --name hbox-busybox-latest-qNDyEVzrUb -v .:/app -w /app -e HTTP_PROXY=$HTTP_PROXY -e HTTPS_PROXY=$HTTPS_PROXY -e NO_PROXY=$NO_PROXY --entrypoint /bin/sh docker.io/busybox:latest -c "ls -alh"
```

### Package Version Management

hbox maintains a directory `$HBOX_DIR/versions` that tracks the current version of each package. Each package has a file in this directory, managed by hbox, and should not be manually edited.

#### Properties

Each package installed with hbox has a version file located in the `$HBOX_DIR/versions` directory. These files are named `<package>.json` and track all versions of the package that are installed. Below are the details of each property available in a version file.

| Property   | Type     | Description                                                                               |
|------------|----------|-------------------------------------------------------------------------------------------|
| `name`     | `string` | The name of the package. Example: `"node"`                                                |
| `versions` | `array`  | An array of versions of the package that are installed. Example: `["latest", "14", "15"]` |
| `current`  | `string` | The version of the package that is currently set as active. Example: `"15"`               |

#### Property Details

- **name**: The name of the package. This is a required property and uniquely identifies the package.

- **versions**: An array listing all versions of the package that are installed. Each entry in the array is a string representing a version.

- **current**: The version of the package that is currently active. This is the version that will be used when the package is executed.

#### Example

Example of a version file for the `node` package:

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

This example indicates that the `node` package has three versions installed (`latest`, `14`, and `15`), with version `15` currently set as the active version.

### Logs

If you enable logs in your `$HBOX_DIR/config.json` file, they will appear in the `$HBOX_DIR/logs` folder. Use this to see what commands are executed under the hood that aren't normally displayed to the user.

**Note**: Be careful when sharing your logs, as they may contain sensitive information such as API keys and environment variables.

## Next steps

If you want to see my ideas for the future of the project, check out the [ROADMAP](ROADMAP.md).
