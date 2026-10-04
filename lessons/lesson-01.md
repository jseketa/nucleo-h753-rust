# 01 · raw registers

## Goal

Blink LD1 (green, PB0) with nothing but addresses.

**Done when:** LD1 blinks.

## Step 01 · hello ✓                                  tag: `lesson-01-step01`

1. `#![no_std]`, `#![no_main]`, `#[cortex_m_rt::entry]`          → Crates
2. Link the transport and the panic handler with `use … as _;`    → Crates
3. One `defmt::info!` message, then an endless loop

Done when: the message appears in the log.

## Step 02 · port B clock on                           tag: `lesson-01-step02`

1. Find the address of `RCC_AHB4ENR` and port B's bit            → Registers, p. 457
2. Make a raw pointer to it                                       → snippet `address as pointer`
3. Read it and log the value                                      → snippets `volatile read / write`, `log in hex`
4. Set port B's bit and write the value back
5. Read it again and log it

Done when: the log shows before and after; after = your calculation.

## Step 03 · PB0 to output                             tag: `lesson-01-step03`

1. Find the address of `GPIOB_MODER` and PB0's two bits          → Registers, p. 540
2. Make a raw pointer to it                                       → snippet `address as pointer`
3. Read it and log the value (should be port B's reset value)    → p. 540
4. Clear PB0's two bits, set them to output                       → snippet `clear and set a field`, Registers
5. Write it back, read it again and log it

Done when: before = the reset value, after = your calculation.

## Step 04 · LD1 on                                    tag: `lesson-01-step04`

1. Find the addresses of `GPIOB_BSRR` and `GPIOB_ODR`            → p. 543, p. 542
2. Read `ODR` and log it
3. Write PB0's set bit to `BSRR` (no read first)                  → Registers
4. Read `ODR` again and log it

Done when: LD1 lights up, and `ODR` shows PB0 set.

## Step 05 · blink                                     tag: `lesson-01-step05`

1. In the endless loop: set PB0, wait, reset PB0, wait            → Registers, snippet `busy wait`
2. Change the wait count until the blink is easy to see

Done when: LD1 blinks.

<details>
<summary>Check values (work them out first)</summary>

| Step | Register | Before | After |
|---|---|---|---|
| 02 | `RCC_AHB4ENR` | `0x00000000` | `0x00000002` |
| 03 | `GPIOB_MODER` | `0xfffffebf` | `0xfffffebd` |
| 04 | `GPIOB_ODR` | `0x00000000` | `0x00000001` |

</details>

## Reference

**Crates**

| Crate | Supplies |
|---|---|
| `cortex-m-rt` | vector table, reset handler, `#[entry]` |
| `defmt` | log macros (`info!`, …) |
| `defmt-rtt` | log transport over RTT; linked with `use defmt_rtt as _;` |
| `panic-probe` | panic handler; linked with `use panic_probe as _;` |
| `core` | built in: `ptr::read_volatile`, `ptr::write_volatile`, `hint::spin_loop` |

**Registers**: address = peripheral base (Table 8, p. 132) + offset (register page)

| Register | Page | Key fact |
|---|---|---|
| `RCC_AHB4ENR` | 457 | one clock-enable bit per GPIO port; all clocks start off |
| `GPIOB_MODER` | 540 | 2 bits per pin: `00` in, `01` out, `10` alt. function, `11` analog |
| `GPIOB_ODR` | 542 | output state of each pin; read it to check |
| `GPIOB_BSRR` | 543 | write-only; bit n sets pin n, bit n+16 resets it |

LD1 = PB0, on when HIGH (UM2407 p. 27). Read back after every write.

**Snippets**

```rust
// address as pointer
const SOME_REG: usize = SOME_BASE + SOME_OFFSET;
let some_reg_ptr = SOME_REG as *mut u32;

// volatile read / write
let value = unsafe { core::ptr::read_volatile(some_reg_ptr) };
unsafe { core::ptr::write_volatile(some_reg_ptr, value | (1 << 3)) };

// clear and set a field (2 bits per pin)
let new = (value & !(0b11 << (PIN * 2))) | (0b10 << (PIN * 2));

// log in hex: =u32 type, # 0x prefix, 0 zero-padded, 10 wide, x hex
defmt::info!("SOME_REG = {=u32:#010x}", value);

// busy wait
for _ in 0..N {
    core::hint::spin_loop();
}
```

**Pointers**: registers go through a raw pointer + volatile, never `&` → [lessons-concepts.md → Pointers](lessons-concepts.md#pointers)

## Syntax

| Syntax | Meaning | C |
|---|---|---|
| `#![attr]` | attribute for the whole crate (`no_std`, `no_main`) | `#pragma` |
| `#[attr]` | attribute for the item below (`#[cortex_m_rt::entry]`) | `__attribute__((…))` |
| `fn name() -> ! { }` | function that never returns | `__attribute__((noreturn))` |
| `use path as _;` | link a crate without using its name | adding a `.c` file to the build |
| `a::b::c` | path: crate / module / item | (C++ `::`) |
| `name!(…)` | macro call, the `!` marks a macro | a `#define` macro |
| `const NAME: usize = …;` | compile-time constant, type required | `#define` / `static const` |
| `let name = …;` | variable, read-only, type inferred | `const auto` |
| `let name: u32 = …;` | variable with an explicit type | `const uint32_t name` |
| `let mut name = …;` | variable that may change | a normal variable |
| `let x = …;` twice | the second `x` is a new variable (shadowing) | (not allowed) |
| `value as *mut u32` | cast | `(uint32_t *)value` |
| `unsafe { … }` | block where you vouch for what the compiler can't check | |
| `0x5802_0400`, `0b11` | hex / binary literal, `_` as separator | `0x58020400`, `0b11` |
| `1 << n`, `a \| b`, `a & b` | shift, OR, AND | same |
| `!x` | bitwise NOT on integers | `~x` |
| `loop { }` | endless loop | `while (1) { }` |
| `for _ in 0..N { }` | runs N times (0 to N−1), `_` = counter not used | `for (i = 0; i < N; i++)` |
| `u32`, `usize` | 32-bit unsigned, pointer-width unsigned | `uint32_t`, `uintptr_t` |

## Pain points

(written at the end of the lesson)

## Learned

(written at the end of the lesson)
