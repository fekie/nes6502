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
    /* let current_test = r#"{ "name": "00 71 9d", "initial": { "pc": 28841, "s": 0, "a": 79, "x": 83, "y": 118, "p": 171, "ram": [ [28841, 0], [28842, 113], [28843, 157], [65534, 203], [65535, 93], [24011, 124]]}, "final": { "pc": 24011, "s": 253, "a": 79, "x": 83, "y": 118, "p": 175, "ram": [ [256, 112], [510, 187], [511, 171], [24011, 124], [28841, 0], [28842, 113], [28843, 157], [65534, 203], [65535, 93]]}, "cycles": [ [28841, 0, "read"], [28842, 113, "read"], [256, 112, "write"], [511, 171, "write"], [510, 187, "write"], [65534, 203, "read"], [65535, 93, "read"]] }"#;
    let mut example: Example = serde_json::from_str(current_test).unwrap();
    example.initial_state.canonicalize();
    example.final_state.canonicalize();
    let mut cpu = Cpu::from_state(example.initial_state.clone());
    cpu.cycle();
    let final_state = cpu.state();
    println!("Final | Expected");
    assert_eq!(final_state, example.final_state);
    println!("Current test success!"); */

    /* let examples = load_examples();

    for example in examples {
        let mut cpu = Cpu::from_state(example.initial_state);
        println!("Running example {}", example.name);
        cpu.cycle();
        let final_state = cpu.state();
        assert_eq!(final_state, example.final_state);
    } */
}

fn load_examples() -> Vec<Example> {
    // load from 65x02/nes6502/v1 directory
    let mut all_examples = Vec::new();

    for (i, file) in std::fs::read_dir("65x02/nes6502/v1").unwrap().enumerate() {
        if i >= 1 {
            break;
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
