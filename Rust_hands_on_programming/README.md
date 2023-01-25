# Table of content
- [Table of content](#table-of-content)
- [Introduction](#introduction)
- [Setup environment, build and test](#setup-environment-build-and-test)
- [How to use the program](#how-to-use-the-program)
- [Design](#design)
- [Error handling](#error-handling)

# Introduction

These programs have the aim to take a file and copy it. To do it, the program ipc_sendfile read the file and sends the data of the file to the program ipc_receivefile which will write the data to a new file. The exchange is done by IPC.

The IPC methods implemented are:
- [] Queue message passing
- [] Pipe
- [] Shared memory 

# Setup environment, build and test

To setup your environment, you shall execute `setup_environment.sh`:
```bash
$ ./setup_environment.sh
```

This script will check if Bazel and g++ are already installed and install them if not.

To build the program and get the test report, you can execute the `build_and_test.sh` file:

```bash
$ ./build_and_test.sh
```

The binary and the test report will be in the folder `output`.

# How to use the program
You can use the program with the argument `--help` and command `--queue --file myFile`, `--pipe --file myFile` or `--shm --file myFile`.

# Design
![Design](Documentation/Current_Design.PNG)
