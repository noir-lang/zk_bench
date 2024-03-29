use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{env, fs};

use crate::circom::Circom;
use crate::{noir, utils};

pub trait Language {
    fn init(&mut self, entry_point: &Path) -> Result<(), String>;
    fn name(&self) -> String;
    fn compile(&self, entry_point: &Path) -> Result<PathBuf, String>;
    fn info(&self, entry_point: &Path) -> Result<String, String>;
    fn setup(&self, entry_point: &Path) -> Result<PathBuf, String>;
    fn execute(&self, entry_point: &Path) -> Result<PathBuf, String>;
    fn prove(&self, key: &Path, witness: &Path) -> Result<PathBuf, String>;
    fn done(&mut self);
}

pub fn run() -> Result<(), String> {
    let main_path = env::current_dir()
        .map_err(|c| format!("Error: {}", c))?
        .join("tests");
    let mut circom = Circom::new();
    let mut noir = noir::Noir {};
    let tests = fs::read_dir(main_path)
        .unwrap()
        .flatten()
        .filter(|c| c.path().is_dir());
    for test in tests {
        //let test_name = test.file_name();
        benchme(test.path(), &mut circom)?;
        benchme(test.path(), &mut noir)?;
    }
    Ok(())
}

pub fn benchme<T>(circuit_path: PathBuf, lang: &mut T) -> Result<String, String>
where
    T: Language,
{
    lang.init(&circuit_path)?;

    //1. compile:
    let start = Instant::now();
    let r1cs_file = lang.compile(&circuit_path)?;
    let compilation_duration = utils::get_time(start);

    //2. info
    let constraints = lang.info(&r1cs_file)?;

    //3. setup
    let start = Instant::now();
    let key = lang.setup(&r1cs_file)?;
    let setup_duration = utils::get_time(start);

    //4. witness generation
    let start = Instant::now();
    let witness_path = lang.execute(&circuit_path)?;
    let exec_duration = utils::get_time(start);

    //5. prove
    let start = Instant::now();
    lang.prove(&key, &witness_path)?;
    let prove_duration = utils::get_time(start);
    let test_name = circuit_path.file_name().unwrap().to_str().unwrap();
    let result = format!(
        "Testing {} with {}:
    {} constraints in {}
    setup generated in {}
    execution in {}
    prove in {}",
        test_name,
        lang.name(),
        constraints,
        compilation_duration,
        setup_duration,
        exec_duration,
        prove_duration
    );
    println!("{}", result);
    lang.done();
    Ok(result)
}
