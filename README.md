# Slice

> **A fast, modern replacement for `head`, `tail`, and `sed` with context awareness.**

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
![Rust](https://img.shields.io/badge/built_with-Rust-red)
![Platform](https://img.shields.io/badge/platform-linux%20%7C%20macos%20%7C%20windows-lightgrey)

**Slice** is a command-line utility designed for developers who need to inspect specific parts of massive log files instantly. It unifies the functionality of multiple legacy tools (`head`, `tail`, `sed`) into a single, high-performance binary with modern features like context awareness, regex matching, and syntax highlighting.

## Features

- **Unified Logic:** No need to pipe `head | tail`. Slice handles head, tail, and specific ranges natively in one command.
- **Context Awareness:** View lines *around* your target range with `--context`, just like `grep -C`.
- **Regex Matching:** Start slicing after a specific pattern with `--after-match`.
- **Smart Buffering:** Uses a ring buffer to efficiently manage memory, even on multi-GB files.
- **Syntax Highlighting:** Automatically colors line numbers (green for matches, blue for context) for easier log scanning.
- **Human Indexing:** Uses 1-based indexing (Line 1 is "1", not "0") — no mental math required.
- **Multiple Files:** Process multiple files seamlessly with automatic headers.
- **Live Monitoring:** Follow files in real-time with `--follow` (like `tail -f`).
- **Byte Slicing:** Extract specific byte ranges for binary file inspection.

## Installation

### Quick Install (Mac & Linux)
The fastest way to install. This script detects your OS/architecture and downloads the correct binary from the latest release.

```bash
curl -fsSL https://raw.githubusercontent.com/rosshhun/slice/main/scripts/install.sh | sh
```

### Homebrew (macOS & Linux)

```bash
brew tap rosshhun/slice
brew install slice
```

Once tapped, you can upgrade with:

```bash
brew upgrade slice
```

### Windows

1. Download the latest `.zip` file from the [Releases Page](https://github.com/rosshhun/slice/releases).
2. Extract `slice.exe`.
3. Add the folder containing `slice.exe` to your system `PATH`.

### From Source (Requires Rust)

```bash
git clone https://github.com/rosshhun/slice.git
cd slice
cargo install --path .
```

## Usage

### Basic Slicing

```bash
# First 10 lines (like 'head')
slice --head 10 app.log

# Last 10 lines (like 'tail')
slice --tail 10 app.log

# Specific range (like 'sed') - Prints lines 50 through 100
slice -n 50-100 app.log
# or
slice --lines 50-100 app.log

# Middle 20 lines of the file
slice --middle 20 app.log

# Open-ended range (line 500 to EOF)
slice -n 500- app.log
```

### Advanced Context

The killer feature of Slice is adding context to any range.

```bash
# "What happened right before the error on line 500?"
# Prints lines 490-510 (10 lines before, 10 lines after)
slice -n 500-500 --context 10 app.log

# Or use asymmetric context
slice -n 500-500 -B 20 -A 5 app.log  # 20 before, 5 after
```

### Regex-Based Slicing

Start slicing after a specific pattern appears.

```bash
# Show everything after "ERROR" appears
slice --after-match "ERROR" --tail 100 app.log

# Combine with ranges for powerful filtering
slice --after-match "Connection established" -n 1-50 app.log
```

### Live Monitoring

```bash
# Follow a log file in real-time (like 'tail -f')
slice --follow --tail 50 /var/log/app.log

# Follow and show line numbers
slice -f -N --tail 20 app.log
```

### Byte-Level Slicing

```bash
# Extract first 1KB of a binary file
slice --bytes 0-1024 firmware.bin

# Note: Byte mode is exclusive — you can't combine it with line-based options
```

### The "Bookends" View

Check the start and end of a file simultaneously to verify integrity.

```bash
slice --head 5 --tail 5 app.log
```

### Multiple Files

```bash
# Process multiple files with automatic headers
slice --tail 5 error.log access.log
# Output:
# ==> error.log <==
# [last 5 lines]
#
# ==> access.log <==
# [last 5 lines]

# Suppress headers with --quiet
slice -q --head 10 *.log
```

## Flags & Options

### Input
| Flag | Description |
|------|-------------|
| `FILE...` | Files to process. Reads from stdin if omitted. |

### Line Selectors (Composable)
| Short | Long | Description |
|-------|------|-------------|
|  | `--head <N>` | Print the first N lines. |
| `-t` | `--tail <N>` | Print the last N lines. |
| `-n` | `--lines <RANGE>` | Print a specific range (e.g., `10-20`, `50:100`, `100-`). |
| `-m` | `--middle <N>` | Print N lines from the exact center of the file. |
|  | `--after-match <PATTERN>` | Start slicing after the first line matching this regex. |

### Byte Selector (Exclusive)
| Short | Long | Description |
|-------|------|-------------|
| `-c` | `--bytes <RANGE>` | Extract byte range (e.g., `0-1024`). Cannot be combined with line options. |

### Context Options
| Short | Long | Description |
|-------|------|-------------|
| `-C` | `--context <N>` | Print N lines before and after matches (like `grep -C`). |
| `-B` | `--before-context <N>` | Print N lines before matches. |
| `-A` | `--after-context <N>` | Print N lines after matches. |

### Display & Behavior
| Short | Long | Description |
|-------|------|-------------|
| `-N` | `--show-line-numbers` | Always show line numbers in output. |
|  | `--color <MODE>` | Control coloring: `auto` (default), `always`, `never`. |
| `-q` | `--quiet` | Suppress file name headers when processing multiple files. |
| `-f` | `--follow` | Output appended data as the file grows (like `tail -f`). |

## Examples

### Debugging a Production Log

```bash
# Find the context around a specific error
slice -n 1523-1523 --context 10 production.log

# Compare the start and end of a rotated log
slice --head 3 --tail 3 app.log.1

# Watch for new errors in real-time
slice --follow --after-match "FATAL" app.log
```

### Quick File Inspection

```bash
# Preview the first 20 lines of multiple config files
slice --head 20 config/*.conf

# Check the last 100 lines of today's log
slice --tail 100 /var/log/app/$(date +%Y-%m-%d).log

# Find and display lines around a specific event
slice -n 5000-5000 -C 25 -N huge-trace.log
```

### Performance Testing

```bash
# Slice works efficiently even on huge files
slice --middle 50 giant-10gb-log.txt

# Extract a specific chunk without loading the entire file
slice -n 1000000-1000100 massive-dataset.csv
```

### Advanced Workflows

```bash
# Combine head + tail for "bookends" with line numbers
slice --head 10 --tail 10 -N server.log

# Extract everything after deployment timestamp
slice --after-match "2024-.*Deployment started" --tail 1000 deploy.log

# Monitor multiple log files simultaneously
slice -f --tail 20 app.log error.log access.log
```

## Feature Status

### Core Features
- [x] **Head/Tail/Middle** — Extract first N, last N, or middle N lines
- [x] **Range Slicing** — Specify exact line ranges (e.g., `50-100`, `500-`)
- [x] **Context Lines** — Show lines before/after matches (`-C`, `-A`, `-B`)
- [x] **Line Numbers** — Display with color-coded syntax highlighting
- [x] **Multiple Files** — Process multiple files with automatic headers
- [x] **Quiet Mode** — Suppress headers for cleaner output
- [x] **Color Control** — Auto/Always/Never color modes
- [x] **Smart Buffering** — Ring buffer for efficient memory usage
- [x] **Human Indexing** — 1-based line numbers (no zero-indexing confusion)

### Planned Features
- [ ] **Regex Matching** (`--after-match`) — Start slicing after pattern match
- [ ] **Byte Slicing** (`--bytes`) — Extract specific byte ranges
- [ ] **Live Monitoring** (`--follow`) — Real-time file following like `tail -f`
- [ ] **Stdin Support** — Read from standard input when no files specified

> **Note:** Features marked with `[ ]` have CLI flags defined but are not yet functional. Contributions welcome!

## Architecture

Slice uses several optimizations for performance:

- **Ring buffer** for context management (no heap allocations per line)
- **Reusable string buffer** to avoid repeated allocations during line reading
- **Early termination** when max range + context is reached
- **Lazy line counting** only when needed (tail/middle operations)


## Acknowledgments

- Inspired by the classic Unix tools: `head`, `tail`, `sed`, and `grep`
- Built with [Rust](https://www.rust-lang.org/) for maximum performance and safety
- Uses [Clap](https://github.com/clap-rs/clap) for robust CLI argument parsing
- Uses [colored](https://github.com/colored-rs/colored) for terminal color output

---
