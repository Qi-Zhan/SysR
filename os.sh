#/bin/bash

cargo build --target riscv32i-unknown-none-elf --bin shell --release    
# dirty script to use virt.lds 
cd ros
cargo build --target riscv32i-unknown-none-elf --bin ros --release    
cd ..
cargo run --bin debugger --release -- target/riscv32i-unknown-none-elf/release/ros 

