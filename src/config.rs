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
// File: config.rs
// Description: ~/tools/Rust/mpi_wrap_gen/src/config.rs
// Author: Dr. Christoph Pospiech

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Deserialize)]
pub struct Templates {
    pub header: String,
    pub ch_init: String,
    pub ch_body: String,
    pub bar_init: String,
    pub bar_body: String,
    pub footer: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub mpi_pdf: PathBuf,
    pub output_file: PathBuf,
    pub templates: Templates,
    pub chapters: Vec<u32>,
    #[serde(default)]
    pub excludes: Vec<String>,
}

/// This function reads the configuration from a YAML
/// file and returns a Config struct.
/// It uses the `serde_yaml_ng` crate to parse the YAML file and
/// the `anyhow` crate for error handling.
pub fn read_config(config_path: &str) -> Result<Config> {
    let config_content = std::fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read config file: {}", config_path))?;
    let config: Config = serde_yaml_ng::from_str(&config_content)
        .with_context(|| format!("Failed to parse config file: {}", config_path))?;
    Ok(config)
}

/// This function prints the configuration to the console for debugging purposes.
pub fn print_config(config: &Config) {
    println!("MPI PDF Path: {}", config.mpi_pdf.display());
    println!("Output File Path: {}", config.output_file.display());
    println!("Header Template:\n{}", config.templates.header);
    println!("Chapter Init Template:\n{}", config.templates.ch_init);
    println!("Chapter Body Template:\n{}", config.templates.ch_body);
    println!("Barrier Init Template:\n{}", config.templates.bar_init);
    println!("Barrier Body Template:\n{}", config.templates.bar_body);
    println!("Footer Template:\n{}", config.templates.footer);
    println!("Chapters: {:?}", config.chapters);
    println!("Excludes: {:?}", config.excludes);
}
