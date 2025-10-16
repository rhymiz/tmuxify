# tmuxify

Interactive tmuxp configuration generator for macOS (and Linux).

## Overview

`tmuxify` is a Rust CLI tool that makes it trivial to bootstrap tmux sessions per project with sensible defaults and repeatable layouts. It generates both a tmuxp configuration file and a `.envrc` file so that entering a project directory automatically starts or attaches to a tmux session via direnv.

## Features

- **Interactive wizard**: Answer a few prompts and get a ready-to-use tmux setup
- **Zero manual YAML editing**: Generate valid tmuxp configs without touching YAML
- **Safe writes**: Automatic backups of existing files (unless `--force`)
- **Flexible storage**: Store configs in `~/.tmuxp/` or project-local `.tmuxp.yaml`
- **direnv integration**: Auto-generate `.envrc` for seamless session management
- **Non-interactive mode**: Use flags for scripting and automation
- **Doctor command**: Validates dependencies and shell configuration

## Prerequisites

Install the required dependencies:

```bash
brew install tmux tmuxp direnv
```

Configure direnv in your shell (add to `~/.zshrc` or `~/.bashrc`):

```bash
eval "$(direnv hook zsh)"  # or bash
```

## Installation

### Quick install (recommended)

```bash
git clone <repository-url>
cd tmuxify
./install.sh
```

This will:
- Build the release binary
- Install to `~/.local/bin/tmuxify` (or custom location via `INSTALL_DIR` env var)
- Check if the installation directory is in your PATH

### Manual installation

```bash
git clone <repository-url>
cd tmuxify
cargo build --release
```

Then copy the binary to your PATH:

```bash
# Option 1: Install to ~/.local/bin (user-level)
cp target/release/tmuxify ~/.local/bin/

# Option 2: Install to /usr/local/bin (system-level, requires sudo)
sudo cp target/release/tmuxify /usr/local/bin/
```

### Uninstall

```bash
./uninstall.sh
```

Or manually remove the binary:

```bash
rm ~/.local/bin/tmuxify  # or wherever you installed it
```

## Usage

### Interactive mode (default)

```bash
tmuxify
```

This will guide you through:
1. Validating dependencies
2. Setting session name
3. Choosing where to store the config (home or project)
4. Creating windows with custom layouts
5. Configuring panes and commands
6. Previewing and confirming the configuration
7. Writing files and running `direnv allow`

### Doctor command

Check your system configuration:

```bash
tmuxify doctor
```

### Non-interactive mode

```bash
# Generate config for current directory, store in home
tmuxify --session myapp --tmuxp-location home

# Generate config for specific project, store locally
tmuxify --project ~/work/myapp --tmuxp-location project --session myapp

# Dry run (preview without writing)
tmuxify --dry-run

# Force overwrite without backups
tmuxify --force
```

### CLI Options

- `--dry-run`: Print planned YAML and .envrc without writing files
- `--force`: Overwrite existing files without creating backups
- `--project <PATH>`: Set project root directory (default: current directory)
- `--tmuxp-location <home|project>`: Where to store the tmuxp file
- `--session <NAME>`: Set session name (default: directory name)
- `--start-dir <PATH>`: Override start_directory in config

## Project Structure

```
src/
├── cli/           # Command-line argument parsing
│   ├── args.rs    # Clap argument definitions
│   └── commands.rs # Command dispatch logic
├── model/         # Data models
│   ├── config.rs  # Main tmuxp configuration
│   ├── pane.rs    # Pane definitions
│   └── window.rs  # Window and layout definitions
├── ops/           # Operations modules
│   ├── doctor.rs      # Diagnostics command
│   ├── interactive.rs # Interactive wizard
│   ├── validate.rs    # Dependency validation
│   └── write.rs       # File writing with backups
└── main.rs        # Entry point
```

## Example Generated Config

```yaml
session_name: myapp
start_directory: /Users/you/myapp
windows:
  - window_name: editor
    layout: main-horizontal
    panes:
      - shell_command:
          - nvim
      - shell_command:
          - git status
```

## Development

Run in development mode:

```bash
cargo run -- doctor
cargo run -- --dry-run
```

Build release binary:

```bash
cargo build --release
```

## License

[Add your license here]

## Contributing

[Add contribution guidelines here]
