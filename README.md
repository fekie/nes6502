# nes6502

An emulated NES version of the 6502 microprocessor (which is a 6502 with the BCD (Binary Coded Decimal) functionality removed).

This was originally part of [my NES emulator](https://github.com/fekie/nes-emulator). It is being moved to its own repository to force better decoupling from the rest of the NES code, as well as making it easier to integrate [Tom Harte's 6502 Tests](https://github.com/SingleStepTests/65x02) which take up a lot of storage space and is only used for testing the CPU.

This cpu is now complete and verified to be correct according to all 256k of [Tom Harte's 6502 Tests](https://github.com/SingleStepTests/65x02). These can be ran by running the default binary (`$ cargo run --release`).

# Running Tests

1. After cloning the repository, download the json test files by running `$ git clone https://github.com/SingleStepTests/65x02` inside the repository.
2. Run `$ cargo run --release` to run the tests.
