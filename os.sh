#/bin/bash

# dirty script to use virt.lds 
cd ros
cargo build --target riscv32i-unknown-none-elf --bin ros --release    
cd ..
