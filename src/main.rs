use std::io::Read;

use nes6502::{Cpu, CpuState};
use sonic_rs::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Example {
    name: String,
    #[serde(rename = "initial")]
    initial_state: CpuState,
    #[serde(rename = "final")]
    final_state: CpuState,
    cycles: Vec<Vec<CyclePart>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CyclePart {
    Integer(u64),
    String(String),
}

fn main() {
    /* let current_test = r#"{ "name": "58 aa 12", "initial": { "pc": 12360, "s": 147, "a": 15, "x": 154, "y": 104, "p": 34, "ram": [ [12360, 88], [12361, 170], [12362, 18]]}, "final": { "pc": 12361, "s": 147, "a": 15, "x": 154, "y": 104, "p": 34, "ram": [ [12360, 88], [12361, 170], [12362, 18]]}, "cycles": [ [12360, 88, "read"], [12361, 170, "read"]] }"#;
    let mut example: Example = serde_json::from_str(current_test).unwrap();
    example.initial_state.canonicalize();
    example.final_state.canonicalize();
    let mut cpu = Cpu::from_state(example.initial_state.clone());
    let (_, success, instruction) = cpu.cycle_debug();

    let final_state = cpu.state();
    println!("Final | Expected");
    if final_state != example.final_state {
        dbg!(instruction.unwrap());
        assert_eq!(final_state, example.final_state);
    }
    println!("Current test success!"); */

    let examples = load_examples();

    for example in examples {
        let mut cpu = Cpu::from_state(example.initial_state);
        println!("Running example {}", example.name);
        let (_, success, instruction) = cpu.cycle_debug();
        if !success {
            // invalid instruction
            continue;
        }
        let final_state = cpu.state();

        if final_state != example.final_state {
            dbg!(instruction.unwrap());
            assert_eq!(final_state, example.final_state);
        }
    }
}

fn load_examples() -> Vec<Example> {
    // load from 65x02/nes6502/v1 directory
    let mut all_examples = Vec::new();

    for (i, file) in std::fs::read_dir("65x02/nes6502/v1").unwrap().enumerate() {
        if !(90..100).contains(&i) {
            continue;
        }

        let file = file.unwrap();
        println!("Reading file {:?}", file.file_name());
        let path = file.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".json") {
            let bytes = std::fs::read(path).unwrap();
            let examples: Vec<Example> = sonic_rs::from_slice(&bytes).unwrap();
            all_examples.extend(examples);
        }
    }

    all_examples
}
