#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

const RCC_BASE: usize = 0x5802_4400;
const RCC_AHB4ENR_OFFSET: usize = 0x0E0;
const RCC_AHB4ENR: usize = RCC_BASE + RCC_AHB4ENR_OFFSET;

const RCC_AHB4ENR_GPIOBEN: u32 = 1 << 1;

#[cortex_m_rt::entry]
fn main() -> ! {
    let rcc_ahb4enr_ptr: *mut u32 = RCC_AHB4ENR as *mut u32;
    let rcc_ahb4_current_value: u32 = unsafe { core::ptr::read_volatile(rcc_ahb4enr_ptr) };
    defmt::info!(
        "Current value of the RCC_AHB4ENR register: {=u32:#010x}",
        rcc_ahb4_current_value
    );

    let rcc_ahb4enr_modified_value: u32 = rcc_ahb4_current_value | RCC_AHB4ENR_GPIOBEN;
    unsafe {
        core::ptr::write_volatile(rcc_ahb4enr_ptr, rcc_ahb4enr_modified_value);
    }

    let rcc_ahb4enr_value_after_modify: u32 = unsafe { core::ptr::read_volatile(rcc_ahb4enr_ptr) };
    defmt::info!(
        "Modified value of the RCC_AHB4ENR register: {=u32:#010x}",
        &rcc_ahb4enr_value_after_modify
    );

    loop {}
}
