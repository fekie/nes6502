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
    /* let current_test = r#"{ "name": "60 8d 97", "initial": { "pc": 52705, "s": 245, "a": 80, "x": 138, "y": 4, "p": 239, "ram": [ [52705, 96], [52706, 141], [52707, 151], [501, 184], [502, 199], [503, 217], [55751, 188], [55752, 144]]}, "final": { "pc": 55752, "s": 247, "a": 80, "x": 138, "y": 4, "p": 239, "ram": [ [501, 184], [502, 199], [503, 217], [52705, 96], [52706, 141], [52707, 151], [55751, 188], [55752, 144]]}, "cycles": [ [52705, 96, "read"], [52706, 141, "read"], [501, 184, "read"], [502, 199, "read"], [503, 217, "read"], [55751, 188, "read"]] }"#;
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
        if !(108..110).contains(&i) {
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
