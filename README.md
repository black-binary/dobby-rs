# dobby-rs

Dobby is a lightweight, multi-platform, multi-architecture exploit hook framework.

This crate is a rusty binding of [Dobby](https://github.com/jmpews/Dobby).

Caution: Hooking is NOT SAFE! Use at your own risk.

## Quickstart

```rust
use dobby_rs::{resolve_symbol, hook, Address};

#[inline(never)]
#[no_mangle]
extern "C" fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[inline(never)]
#[no_mangle]
extern "C" fn sub(a: u64, b: u64) -> u64 {
    a - b
}

let addr = add as usize as Address;
let replace = sub as usize as Address;
unsafe {
    hook(addr, replace).unwrap();
}
```

## Supported Target

- Android
    - x86
    - x86_64
    - armv7
    - aarch64

- MacOS
    - x86_64
    - aarch64

- Linux
    - (WIP) x86
    - x86_64
