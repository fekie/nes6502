use nes6502::{Cpu, CpuState, Interrupts, Mapper};
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

struct Memory([u8; 0x10000]);

impl Memory {
    pub fn new() -> Self {
        Self([0; 0x10000])
    }
}

impl Mapper for Memory {
    fn read(&self, address: u16) -> u8 {
        self.0[address as usize]
    }

    fn write(&mut self, address: u16, byte: u8) {
        self.0[address as usize] = byte
    }
}

#[derive(Default)]
pub struct InterruptsContainer {
    pub interrupt: bool,
    pub non_maskable_interrupt: bool,
}

impl InterruptsContainer {
    fn new() -> Self {
        Self::default()
    }
}

impl Interrupts for InterruptsContainer {
    fn interrupt_state(&self) -> bool {
        self.interrupt
    }

    fn set_interrupt_state(&mut self, new_state: bool) {
        self.interrupt = new_state;
    }

    fn non_maskable_interrupt_state(&self) -> bool {
        self.non_maskable_interrupt
    }

    fn set_non_maskable_interrupt_state(&mut self, new_state: bool) {
        self.non_maskable_interrupt = new_state;
    }
}

fn main() {
    let examples = load_tests();

    for example in examples {
        let memory = Memory::new();
        let interrupts = InterruptsContainer::new();

        let mut cpu = Cpu::from_state(example.initial_state, memory, interrupts);
        println!("Running test {}", example.name);
        let (_, success, instruction) = cpu.cycle_debug();

        if !success {
            // skip invalid instruction
            continue;
        }

        let final_state = cpu.state();

        if final_state != example.final_state {
            dbg!(instruction.unwrap());
            assert_eq!(final_state, example.final_state);
        }
    }

    println!("All tests completed!");
}

fn load_tests() -> Vec<Example> {
    // load from 65x02/nes6502/v1 directory
    let mut all_examples = Vec::new();

    let dir = match std::fs::read_dir("65x02/nes6502/v1") {
        Ok(x) => x,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!("Required tests not found. Please clone the repository located at https://github.com/SingleStepTests/65x02 to this folder.");
                std::process::exit(1);
            }
            _ => panic!("{}", e),
        },
    };

    for file in dir {
        let file = file.unwrap();
        let path = file.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".json") {
            println!("Reading test file {:?}", file.file_name());
            let bytes = std::fs::read(path).unwrap();
            let examples: Vec<Example> = sonic_rs::from_slice(&bytes).unwrap();
            all_examples.extend(examples);
        }
    }

    all_examples
}
