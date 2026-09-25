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
// File: prettify_content.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/prettify/prettify_content.rs
// Author: Dr. Christoph Pospiech

use super::prettify_content;
use anyhow::Result;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::sync::Mutex;
use tempfile::TempDir;

// Guards mutation of the process-wide `PATH` env var across tests in this file.
static PATH_LOCK: Mutex<()> = Mutex::new(());

/// Creates a fake `clang-format` executable (a shell script) in a temporary
/// directory and prepends that directory to `PATH` for the duration of the
/// returned guard, so tests do not depend on a real `clang-format` install.
struct FakeFormatter {
    _dir: TempDir,
    original_path: Option<std::ffi::OsString>,
}

impl FakeFormatter {
    fn new(script: &str) -> Self {
        let dir = TempDir::new().expect("failed to create temp dir");
        let script_path = dir.path().join("clang-format");
        {
            let mut file = std::fs::File::create(&script_path).expect("failed to create script");
            file.write_all(script.as_bytes())
                .expect("failed to write script");
        }
        std::fs::set_permissions(&script_path, std::fs::Permissions::from_mode(0o755))
            .expect("failed to set script permissions");

        let original_path = std::env::var_os("PATH");
        let new_path = match &original_path {
            Some(path) => std::env::join_paths(
                [dir.path().to_path_buf()]
                    .into_iter()
                    .chain(std::env::split_paths(path).collect::<Vec<_>>()),
            )
            .expect("failed to join PATH"),
            None => dir.path().as_os_str().to_owned(),
        };
        std::env::set_var("PATH", new_path);

        Self {
            _dir: dir,
            original_path,
        }
    }
}

impl Drop for FakeFormatter {
    fn drop(&mut self) {
        match &self.original_path {
            Some(path) => std::env::set_var("PATH", path),
            None => std::env::remove_var("PATH"),
        }
    }
}

#[test]
fn test_prettify_content_success() -> Result<()> {
    let _guard = PATH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let _formatter = FakeFormatter::new("#!/bin/sh\nsed 's/$/ formatted/'\n");
    let output = prettify_content("int main() {}")?;
    assert_eq!(output, "int main() {} formatted");
    Ok(())
}

#[test]
fn test_prettify_content_formatter_error_exit() {
    let _guard = PATH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let _formatter = FakeFormatter::new("#!/bin/sh\nexit 1\n");
    let result = prettify_content("int main() {}");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("exited with an error"));
}

#[test]
fn test_prettify_content_invalid_utf8_output() {
    let _guard = PATH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let _formatter = FakeFormatter::new("#!/bin/sh\nprintf '\\377\\376'\n");
    let result = prettify_content("int main() {}");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not valid UTF-8"));
}

#[test]
fn test_prettify_content_missing_formatter() {
    let _guard = PATH_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let original_path = std::env::var_os("PATH");
    std::env::set_var("PATH", "");
    let result = prettify_content("int main() {}");
    match original_path {
        Some(path) => std::env::set_var("PATH", path),
        None => std::env::remove_var("PATH"),
    }
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Failed to start"));
}
