use nes6502::{Cpu, CpuState};
use serde::{Deserialize, Serialize};

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
    /* let current_test = r#"{ "name": "01 d5 ad", "initial": { "pc": 27905, "s": 135, "a": 228, "x": 42, "y": 121, "p": 171, "ram": [ [27905, 1], [27906, 213], [27907, 173], [213, 122], [255, 110], [0, 192], [49262, 26]]}, "final": { "pc": 27907, "s": 135, "a": 254, "x": 42, "y": 121, "p": 169, "ram": [ [0, 192], [213, 122], [255, 110], [27905, 1], [27906, 213], [27907, 173], [49262, 26]]}, "cycles": [ [27905, 1, "read"], [27906, 213, "read"], [213, 122, "read"], [255, 110, "read"], [0, 192, "read"], [49262, 26, "read"]] }"#;
    let mut example: Example = serde_json::from_str(current_test).unwrap();
    example.initial_state.canonicalize();
    example.final_state.canonicalize();
    let mut cpu = Cpu::from_state(example.initial_state.clone());
    cpu.cycle_debug();
    let final_state = cpu.state();
    println!("Final | Expected");
    assert_eq!(final_state, example.final_state);
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
        if i > 10 {
            continue;
        }

        let file = file.unwrap();
        println!("Reading file {:?}", file.file_name());
        let path = file.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".json") {
            let file = std::fs::File::open(path).unwrap();
            let examples: Vec<Example> = serde_json::from_reader(file).unwrap();
            all_examples.extend(examples);
        }
    }

    all_examples
}
