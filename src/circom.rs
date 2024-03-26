use std::{
    env,
    path::{Path, PathBuf},
};

use tempfile::{tempdir, TempDir};

use crate::suite::Language;

const CIRCOM_BINARY: &str = "TODO";
const SNARKJS_BINARY: &str = "/usr/local/bin/snarkjs";
const NODE_BINARY: &str = "/usr/local/bin/node";

pub struct Circom {
    temp_dir: Option<TempDir>,
}

impl Language for Circom {
    fn init(&mut self, _entry_point: &Path) -> Result<(), String> {
        self.new_temp_dir();
        Ok(())
    }

    fn compile(&self, entry_point: &Path) -> Result<PathBuf, String> {
        //circom circuit.circom --r1cs --wasm --sym
        let circom = circom()?;
        let mut command = std::process::Command::new(circom);
        let temp_directory_path = self.get_temp_dir().path();
        let output = command
            .arg(entry_point.join("main.circom"))
            .arg("--r1cs")
            .arg("--wasm")
            .arg("--sym")
            .arg("-o")
            .arg(temp_directory_path)
            .output();
        match output {
            Ok(output) => {
                if output.status.success() {
                    let str = String::from_utf8_lossy(&output.stdout).to_string();
                    if !str.contains("Everything went okay") {
                        return Err(format!("Error running circom: {}", str));
                    }
                } else {
                    let msg = String::from_utf8_lossy(&output.stderr).to_string(); //string_from_stderr...
                    return Err(msg);
                }
            }
            Err(msg) => return Err(format!("Error running circom: {}", msg)),
        }
        Ok(temp_directory_path.join("main.r1cs"))
    }

    fn info(&self, entry_point: &Path) -> Result<String, String> {
        //snarkjs r1cs info mon_fichier.r1cs

        let snarkjs = snarkjs()?;
        let mut command = std::process::Command::new(snarkjs);
        let output = command
            .arg("r1cs")
            .arg("info")
            .arg(entry_point)
            .output()
            .map_err(|c| format!("Error running snarkjs: {}", c))?;

        if output.status.success() {
            let str = String::from_utf8_lossy(&output.stdout).to_string();
            for line in str.lines() {
                if let Some(pos) = line.find("Constraints: ") {
                    let result = line[pos + 13..]
                        .parse::<u32>()
                        .map_err(|c| format!("Error running snarkjs: {}", c))?;
                    return Ok(result.to_string());
                }
            }
            Err(format!(
                "Error, could not get the number of constraints: {}",
                str
            ))
        } else {
            let msg = String::from_utf8_lossy(&output.stderr).to_string(); //string_from_stderr...
            Err(format!("Error running snarkjs: {}", msg))
        }
    }

    fn setup(&self, entry_point: &Path) -> Result<PathBuf, String> {
        //snarkjs groth16 setup mon.r1cs pot15_final.ptau ma.zkey
        let snarkjs = snarkjs().unwrap();
        let mut command = std::process::Command::new(snarkjs);

        let tau = powers_of_tau().unwrap();
        let temp_directory_path = self.get_temp_dir().path();
        let key_path = temp_directory_path.join("circom.key");
        command
            .arg("groth16")
            .arg("setup")
            .arg(entry_point)
            .arg(tau)
            .arg(&key_path)
            .output()
            .map_err(|c| format!("Error running snarkjs: {}", c))?;
        Ok(key_path)
    }

    fn execute(&self, entry_point: &Path) -> Result<PathBuf, String> {
        //node generate_witness.js multiplier2.wasm input.json witness.wtns
        let nodejs = nodejs()?;

        let witness_js = self
            .get_temp_dir()
            .path()
            .join("main_js")
            .join("generate_witness.js");
        let witness_js = assert_file(witness_js)?;
        let wasm_file = self.get_temp_dir().path().join("main_js").join("main.wasm");

        let temp_directory = self.get_temp_dir().path();
        let witnesses = temp_directory.join("witness.wts");
        let inputs = entry_point.join("input.json");

        let mut command = std::process::Command::new(nodejs);
        let output = command
            .arg(witness_js)
            .arg(wasm_file)
            .arg(inputs) //TODO on devrait l'avoir celui la
            .arg(&witnesses)
            .output()
            .map_err(|c| format!("Error running snarkjs: {}", c))?;

        if output.status.success() {
            let str = String::from_utf8_lossy(&output.stdout).to_string();
            assert!(str.is_empty());
            Ok(witnesses)
        } else {
            let msg = String::from_utf8_lossy(&output.stderr).to_string(); //string_from_stderr...
            Err(format!("Error running snarkjs: {}", msg))
        }
    }

    fn prove(&self, key: &Path, witness: &Path) -> Result<PathBuf, String> {
        //snarkjs groth16 prove ma.zkey witness.wtns proof.json public.json
        let temp_directory = tempdir().expect("could not create a temporary directory");
        let temp_directory_path = temp_directory.path();
        let proof_path = temp_directory_path.join("proof.json");
        let public = PathBuf::from("public.json");

        let snarkjs = snarkjs().unwrap();
        let mut command = std::process::Command::new(snarkjs);
        let output = command
            .arg("groth16")
            .arg("prove")
            .arg(key)
            .arg(witness)
            .arg(&proof_path)
            .arg(public)
            .output()
            .map_err(|c| format!("Error running snarkjs: {}", c))?;

        if output.status.success() {
            let str = String::from_utf8_lossy(&output.stdout).to_string();
            assert!(str.is_empty());
            Ok(proof_path)
        } else {
            let msg = String::from_utf8_lossy(&output.stderr).to_string(); //string_from_stderr...
            Err(format!("Error running snarkjs: {}", msg))
        }
    }

    fn done(&mut self) {
        self.close_temp_dir();
    }

    fn name(&self) -> String {
        String::from("Circom (groth16)")
    }
}
impl Circom {
    fn new_temp_dir(&mut self) {
        self.temp_dir = Some(tempdir().expect("could not create a temporary directory"));
    }

    fn close_temp_dir(&mut self) {
        self.temp_dir = None;
    }

    fn get_temp_dir(&self) -> &TempDir {
        self.temp_dir.as_ref().unwrap()
    }

    pub fn new() -> Circom {
        Circom { temp_dir: None }
    }
}

pub fn snarkjs() -> Result<PathBuf, String> {
    get_path(SNARKJS_BINARY)
}

pub fn circom() -> Result<PathBuf, String> {
    get_path(CIRCOM_BINARY)
}

pub fn nodejs() -> Result<PathBuf, String> {
    get_path(NODE_BINARY)
}

fn get_path(binary_path: &str) -> Result<PathBuf, String> {
    let binary_path = PathBuf::from(binary_path);
    assert_file(binary_path)
}

pub fn powers_of_tau() -> Result<PathBuf, String> {
    let path = env::current_dir().map_err(|c| format!("Error: {}", c))?;
    assert_file(path.join("tests").join("powersOfTau28_hez_final_15.ptau"))
}

fn assert_file(file: PathBuf) -> Result<PathBuf, String> {
    if file.is_file() {
        Ok(file)
    } else {
        Err("no file".into())
    }
}
