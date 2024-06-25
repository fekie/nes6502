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
    let current_test = r#"{ "name": "00 35 26", "initial": { "pc": 59521, "s": 242, "a": 4, "x": 71, "y": 56, "p": 97, "ram": [ [59521, 0], [59522, 53], [59523, 38], [65534, 21], [65535, 35], [8981, 229]]}, "final": { "pc": 8981, "s": 239, "a": 4, "x": 71, "y": 56, "p": 101, "ram": [ [496, 113], [497, 131], [498, 232], [8981, 229], [59521, 0], [59522, 53], [59523, 38], [65534, 21], [65535, 35]]}, "cycles": [ [59521, 0, "read"], [59522, 53, "read"], [498, 232, "write"], [497, 131, "write"], [496, 113, "write"], [65534, 21, "read"], [65535, 35, "read"]] }"#;
    let example: Example = serde_json::from_str(current_test).unwrap();
    let mut cpu = Cpu::from_state(example.initial_state.clone());
    assert_eq!(cpu.state(), example.initial_state);
    cpu.cycle();
    let final_state = cpu.state();
    println!("Final | Expected");
    assert_eq!(final_state, example.final_state);
    println!("Current test success!");

    let examples = load_examples();

    for example in examples {
        let mut cpu = Cpu::from_state(example.initial_state);
        dbg!("aaaa");
        cpu.cycle();
        let final_state = cpu.state();
        println!("Final | Expected");
        assert_eq!(final_state, example.final_state);
    }
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
