run: userapp
	@cargo build --target riscv32i-unknown-none-elf --bin shell --release    
	@cd ros && cargo build --target riscv32i-unknown-none-elf --bin ros --release    
	@cargo run --bin run --release -- target/riscv32i-unknown-none-elf/release/ros 

play: game
	@cargo run --bin sdl --release --features="sdl" -- target/riscv32i-unknown-none-elf/release/examples/typing-game 

game:
	@cargo build --example typing-game --target riscv32i-unknown-none-elf --release

userapp: shell simple

shell:
	@cargo build --bin shell --target riscv32i-unknown-none-elf --release

simple:

clean:
	@cargo clean