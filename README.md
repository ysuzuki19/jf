# jf

```
This tool is currently in development.

Please note that it is not yet stable
and may undergo significant changes.
```

Command Line Tool for Running Job.

`jf` allows you to easily run commands or shell-scripts.

Furthermore, `jf` offers some running modes out-of-the-box.

# Running Mode

`jf` has some built-in running mode.

| mode         | description                            |
| :----------- | :------------------------------------- |
| `command`    | normal shell command                   |
| `shell`      | hard coded shell-script                |
| `parallel`   | run some jobs parallel                 |
| `sequential` | run some jobs sequential               |
| `watch`      | run job with watching some dir or file |

The default mode is `command`.

# Dependencies

- Rust
- Cargo

# Installation

```bash
$ git clone https://github.com/ysuzuki19/jf
$ cd jf
jf $ cargo install --path .
```

# Job Definition

You can define job in `jf.toml`.

`description` is optional parameter for `jf description <job>`.

## Common Config

All modes have parameter of fo

```toml
description = "this is sample job" # optional; description of this job for `jf description <job>`
visibility = "private"             # or "public", default is "public"
```

## Modes

### command

```toml
[job.test]
mode = "command"              # optional; you can skip this line because default mode is "command"
command = "cargo"             # required; shell command
args = ["test"]               # required; command arguments
```

### shell

```toml
[job.hello_world]
mode = "shell" # required;
script = """
echo Hello
echo World
"""            # required; shell script to run
```

### parallel

```toml
[job.test-build]
mode = "parallel"         # required;
jobs = ["test", "build"]  # required; job names defined in `jf.toml`
```

### sequential

```toml
[job.test-run]
mode = "sequential"    # required;
jobs = ["test", "run"] # required; job names defined in `jf.toml`
```

### watch

```toml
[job.live-test]
mode = "watch"                        # required;
job = "cargo-test"                    # required; job name defined in `jf.toml`
watch_list = ["src/**", "Cargo.toml"] # required; watch list (glob pattern)
```

# Setup Completion

By the following command, you can setup completion.

```bash
$ source <(jf completion bash) # or zsh, powershell, elvish, fish
```

If you want to setup completion permanently, command is following.

```bash
$ echo "source <(jf completion bash)" >> ~/.bashrc
```
