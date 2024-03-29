# System-R

A learning and just for fun computer system emulator project inspired by [nemu](https://github.com/NJU-ProjectN/nemu).

We aim to implement almost everything in a computer system, including CPU, Memory, IO, OS, Compiler etc.

## Architecture

```text
+-----------------+   +-----------------+   +-----------------+
|                 |   |                 |   |                 |
|   Application   |   |   Application   |   |   Application   |
|    compiler     |   |                 |   |                 |
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
|                      Abstract Machine                       |
|                           RAM                               |
+-------------------------------------------------------------+
|                                                             |
|                    (CPU, IO, Memory)                        |
|                          REMU                               |
+-------------------------------------------------------------+
```

See details in README.md in each directory.

## Abstract Machine Example

A simple typing game:
![sdl](https://github.com/Qi-Zhan/SysR/assets/89050446/df7e74dc-1460-4214-af57-317aace421dd)

```shell
make play # sdl is required
```

## OS Example

A simple OS run on REMU based on RAM:
![asciicast](os.png)

```shell
make run
```
