# Q&A: revision material

Page numbers refer to RM0433 Rev 8.

## memory.x

**Q: Why is it safe to make `FLASH` smaller than the real flash, but dangerous to make `RAM` larger than the real RAM?**
A: A `FLASH` that is too small only means less room: if the program doesn't fit, the linker stops with an error.
A `RAM` that is too large moves the stack: the stack starts at `ORIGIN + LENGTH`, so it starts above the real
RAM. The first push writes to memory that doesn't exist and the chip faults at the very first instructions.
Statics can also land in that missing memory.

**Q: Flash is two banks of 1 MB. Can it be one region?**
A: Yes. Bank 1 is `0x0800_0000–0x080F_FFFF`, bank 2 is `0x0810_0000–0x081F_FFFF` (Table 15, p. 153). They
follow each other without a gap, so one 2M region works. Mapping only bank 1 keeps bank 2 free for updates
or stored data.

**Q: One `FLASH` region of 2 MB, or bank 1 only?**
A: Only one operation runs at a time on a given bank (p. 151). Erasing or programming the bank you run from
stalls the CPU until it's finished. Running from bank 1 while writing bank 2 works in parallel
(read-while-write). Keeping bank 2 out of `FLASH` makes sure the linker never puts code there.

**Q: Where is the flash size stored on the chip?**
A: Section 61.2 "Flash size", p. 3292: base address `0x1FF1_E880`, value in KB.
`probe-rs read --chip STM32H753ZITx b32 0x1FF1E880 1`, lower 16 bits.

**Q: Why can't `RAM` be ITCM at `0x0000_0000`?**
A: Address 0 is the null pointer. A variable placed there would have the address 0, and Rust references can
never be null. ITCM is also small (64K), and no DMA except MDMA reaches it. It's meant for fast code.

**Q: Which DMAs can't reach DTCM?**
A: All except MDMA. DTCM is reached by the Cortex-M7 and by MDMA through the M7's AHBS port (section 2.4,
p. 136). DMA1, DMA2, BDMA, the Ethernet DMA and the USB OTG DMA can't reach it. That's why `RAM` is AXI SRAM.

**Q: SRAM1, SRAM2 and SRAM3 are consecutive. One region or three?**
A: Three. The Ethernet MAC or USB OTG HS can access one SRAM section while the CPU accesses another at the
same time (section 2.4, p. 136). ST suggests SRAM3 for Ethernet and USB buffers. Separate regions let you
place buffers so that they don't compete.

**Q: Are SRAM1–3 clocked after reset?**
A: No. `RCC_AHB2ENR` (p. 455): `SRAM1EN`, `SRAM2EN`, `SRAM3EN` = 0 after reset ("interface clock is
disabled"). Set them before using these RAMs.

**Q: What has to be enabled before writing to the backup SRAM?**
A: Two things:
- the clock, `BKPRAMEN` in `RCC_AHB4ENR` (p. 457);
- write access to the backup domain, `DBP` in `PWR_CR1` (p. 266). After reset the backup domain is
  write-protected.

To keep its contents on VBAT, also enable the backup regulator: `BREN` in `PWR_CR2`, then wait for `BRRDY`.

**Q: Is `memory.x` the linker?**
A: No. `memory.x` holds only the `MEMORY { }` part of a linker script. `link.x` (from cortex-m-rt) holds
`SECTIONS { }` and pulls `memory.x` in with `INCLUDE`. The linker itself is the program `rust-lld`.

## build.rs

**Q: Is `build.rs` like a Makefile?**
A: Cargo is the Makefile. `build.rs` is one extra pre-build rule in it, written in Rust, that runs on the PC
before the firmware is compiled.

**Q: Why is `unwrap()` fine in `build.rs`, but risky in firmware?**
A: In `build.rs`, a panic fails the build: you see the message at once and nothing broken reaches the board.
In firmware, a panic stops the device at runtime, possibly with nobody watching. Also, cargo always sets
`OUT_DIR`, so that `unwrap()` can only fail when the script is run by hand.

**Q: `join` returns a new path. How do you change the path itself?**
A: Make the variable mutable and use `push`:
```rust
let mut out_dir = PathBuf::from(...);
out_dir.push("memory.x");
```
Rust variables are read-only by default; `mut` allows changes (the opposite of C's `const`). `join` is better
here: `out_dir` still means "the folder", which is needed again for `rustc-link-search`.

**Q: When can `fs::copy("memory.x", …).unwrap()` fail, and what do you see?**
A: When `memory.x` isn't in the project root (renamed, moved, deleted). The build stops with
`failed to run custom build command` … `No such file or directory (os error 2)`: a clear message at build
time instead of a confusing linker error later.

**Q: Why does the compiler warn "unused `Result` that must be used"?**
A: `Result` is marked `#[must_use]`: dropping one without looking at it is almost always a bug, because the
error disappears silently. Handle it (`.unwrap()`, `?`, `match`). Don't silence it with `let _ = …`: that
says "I'm ignoring this error on purpose". In C, the equivalent is a return code nobody checks.

**Q: How do I check whether a `Result` is `Ok` or `Err`?**
A: `Result<T, E>` is an enum, `Ok(T)` or `Err(E)` (a tagged union you can't read without checking the tag).

| Way | On `Err` |
|---|---|
| `x.unwrap()` | panics with the error |
| `x.expect("msg")` | panics with `msg` + the error |
| `match x { Ok(v) => …, Err(e) => … }` | you decide; both cases must be covered |
| `if let Err(e) = x { … }` | handle only the case you care about |
| `x?` | returns the error to the caller (only in functions that return `Result`) |

Build scripts and tools: `unwrap`/`expect` (failing stops the build, which is right). Firmware: `match`/`?`.

**Q: So `build.rs` controls cargo by printing?**
A: Yes. Cargo captures the script's stdout and reads it line by line: lines starting with `cargo:` are
instructions (`rustc-link-search`, `rerun-if-changed`, …), everything else is ignored. A normal `cargo build`
doesn't show them. See them with `cargo build -vv` or in
`target/thumbv7em-none-eabihf/debug/build/nucleo-h753-rust-<hash>/output`.

**Q: Why can't `println!("{}", path)` print a `PathBuf`?**
A: `{}` needs the `Display` trait ("can show itself as text"). `PathBuf` doesn't have it on purpose: a path
can contain bytes that aren't valid UTF-8. `path.display()` returns a printable view.

**Q: `rerun-if-changed`: the copy in `OUT_DIR` or the original `memory.x`?**
A: The original in the project root, the file you edit. Watching the copy means edits to `memory.x` don't
re-run the script: the linker keeps the old copy and the old memory map, silently.

**Q: "borrow of moved value": what happened?**
A: Passing a variable **by value** (`fs::copy(a, dest)`) moves it: ownership goes to the function, the
variable can't be used afterwards. Pass a reference (`&dest`) to lend it instead. C has no equivalent; there,
both sides would keep using the same pointer and nothing checks who owns it.

## Lesson 01

**Q: `undefined symbol: _defmt_acquire` / `_defmt_write` / `_defmt_release`: what's missing?**
A: The defmt **transport**. `defmt` only declares these functions (lock, write bytes, unlock); a transport crate
such as `defmt-rtt` provides them. It is never called by name, so it must be linked with `use defmt_rtt as _;`.
C equivalent: a function declared in a header, but no `.c` file implementing it in the build.

**Q: Which crates need `use … as _;`?**
A: Crates whose names you never use, but whose code must be linked: the defmt transport (`defmt_rtt`) and the
panic handler (`panic_probe`). Crates you call by name (`defmt::info!`, `cortex_m_rt::entry`) are linked
anyway.

**Q: What is a pointer, and what do `*`, `&` and `&*` mean?**
A: A pointer is a variable that holds an address plus the type of what lives there. Rust has references
(`&u32`, checked, always valid) and raw pointers (`*const u32` / `*mut u32`, unchecked, just a number).
In a type: `&u32`, `*mut u32` name pointer types. In an expression: `&x` makes a reference to `x`, `*r`
follows a pointer to the value, `addr as *mut u32` turns a number into a raw pointer, `&*p` follows raw
pointer `p` and makes a reference to the target (raw → reference, `unsafe`). Making a pointer is safe;
using a raw pointer is `unsafe`. Registers: `read_volatile` / `write_volatile`, not plain `*p`.
Details: lessons-concepts.md → Pointers.

**Q: Why is the register address `usize` and not `u32`?**
A: The address and the contents are different things. Address: `usize`, the integer type exactly as wide as a
pointer (32 bits on the M7, 64 on the PC), and the type Rust uses for offsets, indexes and sizes. Contents:
`u32`, because the register is 32 bits wide. The pointer `*mut u32` joins them: "at this address lives a
`u32`". C equivalent: `uintptr_t` vs `uint32_t`.

**Q: Can constants and variables be defined outside `fn main`?**
A: `const` and `static` yes, `let` no.
- `const NAME: T = …;` top level fine, type required; pasted in where used, no address (`#define` with a type).
- `static NAME: T = …;` one copy at a fixed address for the whole program (C global).
- `static mut` every access is `unsafe`; avoid for now.
- `let` only inside functions (local, on the stack).
Register addresses: `const` at the top of the file.
