# Concepts

Concepts used across several lessons. Lessons link here instead of repeating them.

## Pointers

**Address:** memory is a row of numbered bytes; a byte's number is its address. A register is 4 bytes at a
fixed address.

**Pointer:** a variable that holds an address, plus the type of what lives there. The type tells the
compiler how many bytes to read (`u32` = 4).

### Two kinds

| | reference `&u32` / `&mut u32` | raw pointer `*const u32` / `*mut u32` |
|---|---|---|
| Is | a borrow of a value Rust knows about | just a number |
| Promises | non-null, aligned, valid; `&mut`: nobody else writes it | nothing |
| Compiler may | read once and keep the value, drop or merge writes | (with `volatile`) nothing: every access happens, in order |
| Create | safe | safe |
| Read / write through it | safe | `unsafe` |
| Use for | normal variables | hardware registers, C code |

### The symbols

`*` and `&` mean different things in a type and in an expression.

| Written | Where | Meaning | C |
|---|---|---|---|
| `&u32`, `&mut u32` | type | reference to a `u32` (read-only / writable) | |
| `*const u32`, `*mut u32` | type | raw pointer to a `u32` (read-only / writable) | `const uint32_t *`, `uint32_t *` |
| `&x`, `&mut x` | expression | point at `x`: make a reference (borrow) | `&x` |
| `*r` | expression | follow `r`: the value at the address | `*r` |
| `0x4000_0000 as *mut u32` | expression | turn a number into a raw pointer | `(uint32_t *)0x40000000` |
| `&*p` | expression | follow raw pointer `p`, point at the target: raw pointer → reference (`unsafe`) | `p` |

Read `&*p` from the inside out: `*p` is "the `u32` at that address", `&` makes a reference to it.

### Why registers need raw pointers

A register breaks every promise of `&`: hardware changes its value, writes have side effects (`BSRR`), some
reads have side effects (clearing a flag). So: raw pointer + `read_volatile` / `write_volatile`. A plain `*p`
may be skipped or merged by the optimizer.

```rust
// Reference: the compiler may read once, keep the value, and loop forever.
let status: &u32 = unsafe { &*(SOME_STATUS as *const u32) };
while *status & READY == 0 {}

// Raw pointer + volatile: the register is read again on every pass.
let status_ptr = SOME_STATUS as *const u32;
while unsafe { core::ptr::read_volatile(status_ptr) } & READY == 0 {}
```
