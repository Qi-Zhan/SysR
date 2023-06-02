# SysR

A learning and just for full system emulator project inspired by [nemu](https://github.com/NJU-ProjectN/nemu).
We aim to implement almost everything in a computer system, including CPU, Memory, IO, OS, etc.

## Architecture
```
+-----------------+   +-----------------+   +-----------------+
|                 |   |                 |   |                 |
|   Application   |   |   Application   |   |   Application   |
|                 |   |                 |   |                 |
+-----------------+   +-----------------+   +-----------------+
|                 |   |                 |   |                 |
|      ABI        |   |      ABI        |   |     ABI         |
|    syscall      |   |     syscall     |   |    syscall      |
+-------------------------------------------------------------+   
|                                                             |
|                      Operating System                       |
|                           ROS                               |
+-------------------------------------------------------------+
|                                                             |
|                           SBI                               |
|                           AM4R                              |
+-------------------------------------------------------------+
|                                                             |
|                    (CPU, IO, Memory)                        |
|                          REMU                               |
+-------------------------------------------------------------+
```

TBD



## Abstract Machine Example

A simple typing game: 
https://github.com/Qi-Zhan/SysR/assets/89050446/8f8076b9-20dd-4fd4-b0a2-c2f8752b8544


## Usage
```
$ cargo run --release -- -h TBD
```
