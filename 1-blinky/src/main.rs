#![no_std]
#![no_main]

use core::ptr::write_volatile;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    const GPIO_P0_POS: u32 = 0x5000_0000;

    const GPIO_P0_PIN13_OFFSET: u32 = 0x734;
    const GPIO_P0_PIN14_OFFSET: u32 = 0x738;
    const GPIO_P0_PIN15_OFFSET: u32 = 0x73C;
    const GPIO_P0_PIN16_OFFSET: u32 = 0x740;

    const GPIO_P0_OUTSET_OFFSET: u32 = 0x508;
    const GPIO_P0_OUTCLR_OFFSET: u32 = 0x50C;

    // Set all positions to high (led=off) to start

    const GPIO_P0_OUTSET_ADDR: *mut u32 = (GPIO_P0_POS + GPIO_P0_OUTSET_OFFSET) as *mut u32;
    const GPIO_P0_OUTCLR_ADDR: *mut u32 = (GPIO_P0_POS + GPIO_P0_OUTCLR_OFFSET) as *mut u32;

    const GPIO_P0_OUT_POSITIONS: [u32; 4] = [13, 14, 16, 15];

    for position in GPIO_P0_OUT_POSITIONS {
        unsafe {
            write_volatile(GPIO_P0_OUTSET_ADDR, (true as u32) << position);
        }
    }

    // Set P0.13-P0.16 to output

    const GPIO_P0_PIN_OFFSETS: [u32; 4] = [
        GPIO_P0_PIN13_OFFSET,
        GPIO_P0_PIN14_OFFSET,
        GPIO_P0_PIN15_OFFSET,
        GPIO_P0_PIN16_OFFSET,
    ];

    const DIR_OUTPUT_POS: u32 = 0;
    const PINCNF_DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS;

    for offset in GPIO_P0_PIN_OFFSETS {
        let pincnf_addr: *mut u32 = (GPIO_P0_POS + offset) as *mut u32;

        unsafe {
            // volatile write to prevent compiler from optimizing it out
            write_volatile(pincnf_addr, PINCNF_DRIVE_LED);
        }
    }

    // Drive pins in up and down in order 13, 14, 16, 15

    let mut position_index = 0;
    loop {
        let position = GPIO_P0_OUT_POSITIONS[position_index];
        position_index = (position_index + 1) % 4;
        unsafe {
            write_volatile(GPIO_P0_OUTCLR_ADDR, (true as u32) << position);
        }
        wait();
        unsafe {
            write_volatile(GPIO_P0_OUTSET_ADDR, (true as u32) << position);
        }
        wait();
    }
}

fn wait() {
    for _ in 0..200_000 {
        nop();
    }
}
