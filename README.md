# mpi_wrap_gen

Create wrapper files for MPI profiler.

## Overview

`mpi_wrap_gen` parses the official MPI standard PDF (e.g. `mpi31-report.pdf`) to extract
MPI function signatures and generates C source files needed for MPI profiling wrappers —
such as identifier headers and label arrays.

This tool replaces an earlier Perl-based workflow that relied on a plain-text conversion of the MPI standard.

## Features

- Extracts MPI function signatures directly from the MPI standard PDF
- Identifies chapter boundaries via PDF outline (bookmark) tree
- Parses function names and parameter lists from Chapter 3 of the MPI standard
- Generates C header and source files for MPI tracing

## Requirements

- Rust 1.70+ (edition 2021)

## Building

```bash
cargo build --release
```

## Usage

```bash
mpi_wrap_gen [OPTIONS] [INPUT_FILE]
```

### Options

| Option | Description |
|--------|-------------|
| `-v`, `--verbose` | Enable verbose output (default: true) |
| `-D`, `--dry-run` | Perform a dry run without making changes |

### Arguments

| Argument | Description |
|----------|-------------|
| `INPUT_FILE` | Path to the MPI standard PDF |

Note: The CLI may fall back to an implementation-defined default path for the MPI standard PDF,
but this path is environment-specific and may not exist on your system. It is recommended to
always provide `INPUT_FILE` explicitly as the path to your local copy of the MPI standard
(for example, `mpi31-report.pdf` in your project or downloads directory).
### Example

```bash
mpi_wrap_gen /path/to/mpi31-report.pdf
```

## Project Structure

```text
src/
├── main.rs         # Entry point: loads PDF, orchestrates extraction
├── cmdline.rs      # CLI argument parsing (clap)
├── chapters.rs     # PDF outline traversal, chapter-page mapping
└── mpi_calls.rs    # MPI function signature extraction and parsing
examples/
├── LMX_mpitrace_ids.h      # Example generated identifier header
└── LMX_mpitrace_labels.c   # Example generated label array
```

## License

Apache License 2.0 — see [LICENSE](LICENSE) for details.
