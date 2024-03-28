use assert_cmd::Command;
use std::path::{Path, PathBuf};

use crate::suite::Language;

const NARGO_BINARY: &str = "nargo";

impl Language for Noir {
    fn init(&mut self, _entry_point: &Path) -> Result<(), String> {
        Ok(())
    }

    fn compile(&self, entry_point: &Path) -> Result<PathBuf, String> {
        let mut cmd = Command::new(NARGO_BINARY);
        cmd.arg("--program-dir").arg(entry_point);
        cmd.arg("compile");
        cmd.arg("--force");

        let output = cmd.output().expect("Failed to execute command");

        if !output.status.success() {
            Err(format!(
                "`nargo compile` failed with: {}",
                String::from_utf8(output.stderr).unwrap_or_default()
            ))
        } else {
            Ok(entry_point.to_path_buf())
        }
    }

    fn info(&self, entry_point: &Path) -> Result<String, String> {
        let mut cmd = Command::new(NARGO_BINARY);
        cmd.arg("--program-dir").arg(entry_point);
        cmd.arg("info");
        cmd.arg("info");

        let output = cmd.output().expect("Failed to execute command");
        let (acir_size, circuit_size) = if output.status.success() {
            let json: serde_json::Value =
                serde_json::from_slice(&output.stdout).unwrap_or_else(|e| {
                    panic!(
                        "JSON was not well-formatted {{:?}}\n\n{{:?}}",
                        e,
                        std::str::from_utf8(&output.stdout)
                    )
                });

            let num_opcodes = &json["programs"][0]["acir_opcodes"];
            let num_opcodes = &json["programs"][0]["gates"];
        } else {
            return Err(format!(
                "`nargo info` failed with: {}",
                String::from_utf8(output.stderr).unwrap_or_default()
            ));
        };
        let result = format!("{} ({} acir)", circuit_size, acir_size);
        Ok(result)
    }

    fn setup(&self, _entry_point: &Path) -> Result<PathBuf, String> {
        Ok(PathBuf::new())
    }

    fn execute(&self, entry_point: &Path) -> Result<PathBuf, String> {
        let mut cmd = Command::new(NARGO_BINARY);
        cmd.arg("--program-dir").arg(entry_point);
        cmd.arg("execute");

        let output = cmd.output().expect("Failed to execute command");
        if !output.status.success() {
            let msg = String::from_utf8(output.stderr).unwrap_or_default();
            Err(format!("`nargo execute` failed with: {}", msg))
        } else {
            Ok(entry_point.to_path_buf())
        }
    }

    fn prove(&self, _key: &Path, witness: &Path) -> Result<PathBuf, String> {
        let mut cmd = Command::new(NARGO_BINARY);
        cmd.arg("--program-dir").arg(witness);
        cmd.arg("prove");

        let output = cmd.output().expect("Failed to execute command");
        if !output.status.success() {
            Err(format!(
                "`nargo execute` failed with: {}",
                String::from_utf8(output.stderr).unwrap_or_default()
            ))
        } else {
            Ok(PathBuf::new())
        }
    }

    fn done(&mut self) {}

    fn name(&self) -> String {
        String::from("Noir")
    }
}

pub struct Noir {}

fn parse_nargo_info(stdout: Vec<u8>) -> Result<(u32, u32), String> {
    let str = String::from_utf8_lossy(&stdout).to_string();
    for line in str.lines() {
        let mut next = 0;
        let mut acir_size = 0;
        let circuit_size;
        for word in line.split('|') {
            if next == 1 {
                circuit_size = word
                    .trim()
                    .parse::<u32>()
                    .map_err(|c| format!("Error parsing nargo info: {}", c))?;
                return Ok((acir_size, circuit_size));
            } else if next == 2 {
                acir_size = word
                    .trim()
                    .parse::<u32>()
                    .map_err(|c| format!("Error parsing nargo info: {}", c))?;
                next -= 1;
            } else if word.contains("Bounded") {
                next = 2;
            }
        }
    }
    Err(format!(
        "Error, could not get the number of constraints: {}",
        str
    ))
}
