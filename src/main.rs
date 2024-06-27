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
    /* let current_test = r#"{ "name": "10 98 49", "initial": { "pc": 41379, "s": 218, "a": 248, "x": 28, "y": 116, "p": 32, "ram": [ [41379, 16], [41380, 152], [41381, 73], [41277, 175]]}, "final": { "pc": 41277, "s": 218, "a": 248, "x": 28, "y": 116, "p": 32, "ram": [ [41277, 175], [41379, 16], [41380, 152], [41381, 73]]}, "cycles": [ [41379, 16, "read"], [41380, 152, "read"], [41381, 73, "read"]] }"#;
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
        if !(0..20).contains(&i) {
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
