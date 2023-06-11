strip="riscv64-unknown-elf-strip"

run: userapp
	@cargo build --target riscv32i-unknown-none-elf --bin shell --release    
	@cd ros && cargo build --target riscv32i-unknown-none-elf --bin ros --release    
	@cargo run --bin run --release -- target/riscv32i-unknown-none-elf/release/ros 

debug: userapp
	@cargo build --target riscv32i-unknown-none-elf --bin shell --release    
	@cd ros && cargo build --target riscv32i-unknown-none-elf --bin ros --release    
	@cargo run --bin run --release -- target/riscv32i-unknown-none-elf/release/ros -- debug

play: game
	@cargo run --bin sdl --release --features="sdl" -- target/riscv32i-unknown-none-elf/release/examples/typing-game 

game:
	@cargo build --example typing-game --target riscv32i-unknown-none-elf --release

userapp: shell simple1 simple2

shell:
	@cargo build --bin shell --target riscv32i-unknown-none-elf --release
	@$(strip) target/riscv32i-unknown-none-elf/release/shell

simple1:
	@cargo build --bin simple1 --target riscv32i-unknown-none-elf --release
	@$(strip) target/riscv32i-unknown-none-elf/release/simple1

simple2:
	@cargo build --bin simple2 --target riscv32i-unknown-none-elf --release
	@$(strip) target/riscv32i-unknown-none-elf/release/simple2

clean:
	@cargo clean