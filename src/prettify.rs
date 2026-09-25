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

use crate::config::Languages;
use anyhow::{bail, Context, Result};
use std::io::Write;
use std::process::{Command, Stdio};

/// Pretty-prints generated source `content` by piping it through an
/// external formatter chosen based on `language` (`clang-format` for C,
/// `fprettify` for Fortran).
pub fn prettify_content(language: &Languages, content: &str) -> Result<String> {
    let formatter = match language {
        Languages::C => "clang-format",
        Languages::Fortran => "fprettify",
    };
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
    child
        .stdin
        .take()
        .expect("stdin was piped")
        .write_all(content.as_bytes())?;
    let output = child
        .wait_with_output()
        .with_context(|| format!("Failed to read output from `{}`", formatter))?;
    if !output.status.success() {
        bail!(
            "`{}` exited with an error while formatting output",
            formatter
        );
    }
    String::from_utf8(output.stdout)
        .with_context(|| format!("`{}` produced output that is not valid UTF-8", formatter))
}
