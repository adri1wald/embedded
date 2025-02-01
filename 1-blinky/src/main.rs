#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;

use nrf52840_hal::{
    gpio::{self, Level, Output, Pin, PushPull},
    pac,
};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().expect("only called once");
    let port0 = gpio::p0::Parts::new(p.P0);

    let mut pins: [Pin<Output<PushPull>>; 4] = [
        port0.p0_13.into_push_pull_output(Level::High).degrade(),
        port0.p0_14.into_push_pull_output(Level::High).degrade(),
        port0.p0_15.into_push_pull_output(Level::High).degrade(),
        port0.p0_16.into_push_pull_output(Level::High).degrade(),
    ];
    // Drive pins up and down in cycle

    let mut pin_index = 0;

    loop {
        let pin = &mut pins[pin_index];

        let _ = pin.set_low();

        wait();

        let _ = pin.set_high();

        pin_index = (pin_index + 1) % 4;
    }
}

fn wait() {
    for _ in 0..200_000 {
        nop();
    }
}
