rustos
================

## Overview
Small Operation System written by Rust

## Requirements
- Ruby
- Rake
- Rust 1.5-nightly
- QEMU

## Building
```sh
rake build
```
## Running rustos
```sh
rake run
(into uefi shell on qemu)
uefibootloader fs0:\rustos
```

* Using [uefi-bootloader](https://github.com/fgken/uefi-bootloader)

