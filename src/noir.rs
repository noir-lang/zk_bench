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
        cmd.arg("--json");

        let output = cmd.output().expect("Failed to execute command");
        let (acir_size, circuit_size) = if output.status.success() {
            let json: serde_json::Value =
                serde_json::from_slice(&output.stdout).unwrap_or_else(|e| {
                    panic!(
                        "JSON was not well-formatted {:?}\n\n{:?}",
                        e,
                        std::str::from_utf8(&output.stdout)
                    )
                });

            let num_opcodes = json["programs"][0]["acir_opcodes"].as_u64().unwrap();
            let num_gates = json["programs"][0]["circuit_size"].as_u64().unwrap();
            (num_opcodes, num_gates)
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
