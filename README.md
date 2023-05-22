# System-R

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



## Usage
```
$ cargo run --release -- -h TBD
```