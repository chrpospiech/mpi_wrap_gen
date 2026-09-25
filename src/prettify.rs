// Copyright 2026 mpi_wrap_gen C. Pospiech
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// File: prettify.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/prettify.rs
// Author: Dr. Christoph Pospiech

use anyhow::{anyhow, bail, Context, Result};
use std::io::{Read, Write};
use std::process::{Command, Stdio};

#[cfg(test)]
mod prettify_content;

/// Pretty-prints generated source `content` which is supposed to be valid
/// `C` source code by piping it through the external formatter `clang-format`.
pub fn prettify_content(content: &str) -> Result<String> {
    let formatter = "clang-format";
    let mut child = Command::new(formatter)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .with_context(|| {
            format!(
                "Failed to start `{}`; is it installed and on PATH?",
                formatter
            )
        })?;
    let mut stdin = child.stdin.take().expect("stdin was piped");
    let content = content.to_owned();
    // A broken pipe here just means the formatter exited before reading all
    // input (e.g. it errored out early); the real failure is reflected in
    // its exit status or output, so don't treat this as a hard error.
    let write_handle = std::thread::spawn(move || match stdin.write_all(content.as_bytes()) {
        Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => Ok(()),
        result => result,
    });

    let mut stdout = child.stdout.take().expect("stdout was piped");
    let mut formatted = Vec::new();
    stdout
        .read_to_end(&mut formatted)
        .with_context(|| format!("Failed to read output from `{}`", formatter))?;

    write_handle
        .join()
        .map_err(|_| anyhow!("Failed to write input to `{}`", formatter))?
        .with_context(|| format!("Failed to write input to `{}`", formatter))?;

    let status = child
        .wait()
        .with_context(|| format!("Failed to wait for `{}`", formatter))?;
    if !status.success() {
        bail!(
            "`{}` exited with an error while formatting output",
            formatter
        );
    }
    String::from_utf8(formatted)
        .with_context(|| format!("`{}` produced output that is not valid UTF-8", formatter))
}
