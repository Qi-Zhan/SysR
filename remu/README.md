# REMU (Rust Emulator)

```
  _____                       _           _                 _      _       _       _ 
 | ____|  _ __ ___    _   _  | |   __ _  | |_    ___       / \    | |     | |     | |
 |  _|   | '_ ` _ \  | | | | | |  / _` | | __|  / _ \     / _ \   | |     | |     | |
 | |___  | | | | | | | |_| | | | | (_| | | |_  |  __/    / ___ \  | |___  | |___  |_|
 |_____| |_| |_| |_|  \__,_| |_|  \__,_|  \__|  \___|   /_/   \_\ |_____| |_____| (_)
```


We aim to implement a full system emulator with the following features:
* CPU emulation support various architectures (x86, ARM, RISC-V, etc.), now riscv32i is supported.
* Memory management support various memory management schemes (MMU, TLB, etc.), now sv32 is supported.
* Device emulation support various devices (UART, VGA, etc.), now UART is supported.
* Debug support various debug features (assembly simualtor,  etc.), now 

Based on the low-level emulation, we can implement our own **OS** and run it on the emulator.

## Usage
```
$ cargo run TBD
```

## Architecture
```
+-----------------+   +-----------------+   +-----------------+
|                 |   |                 |   |                 |
|   Application   |   |   Application   |   |   Application   |
|                 |   |                 |   |                 |
+-----------------+   +-----------------+   +-----------------+
|                 |   |                 |   |                 |
|      ABI        |   |      ABI        |   |      ABI        |
|                 |   |     syscall     |   |                 |
+-------------------------------------------------------------+   
|                                                             |
|                      Operating System                       |
|                           ROS(还没写呢)                       |
+-------------------------------------------------------------+
|                                                             |
|                            SBI                              |
|                           RAM(大概吧)                        |
+-------------------------------------------------------------+
|                                                             |
|                            CPU                              |
|                          REMU(你正看到的)                     |
+-------------------------------------------------------------+
```
