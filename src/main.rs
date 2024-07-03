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
    let current_test = r#"{ "name": "46 13 c2", "initial": { "pc": 4179, "s": 218, "a": 129, "x": 146, "y": 45, "p": 230, "ram": [ [4179, 70], [4180, 19], [4181, 194], [19, 87]]}, "final": { "pc": 4181, "s": 218, "a": 129, "x": 146, "y": 45, "p": 101, "ram": [ [19, 43], [4179, 70], [4180, 19], [4181, 194]]}, "cycles": [ [4179, 70, "read"], [4180, 19, "read"], [19, 87, "read"], [19, 87, "write"], [19, 43, "write"]] }"#;
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
    println!("Current test success!");

    /* let examples = load_examples();

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
    } */
}

fn load_examples() -> Vec<Example> {
    // load from 65x02/nes6502/v1 directory
    let mut all_examples = Vec::new();

    for (i, file) in std::fs::read_dir("65x02/nes6502/v1").unwrap().enumerate() {
        if !(70..100).contains(&i) {
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
