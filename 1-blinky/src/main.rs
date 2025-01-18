#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use nrf52840_pac::Peripherals;
use panic_halt as _;

#[derive(Clone, Copy)]
enum Pin {
    P13,
    P14,
    P15,
    P16,
}

use Pin::*;

impl Pin {
    fn index(&self) -> usize {
        match self {
            Pin::P13 => 13,
            Pin::P14 => 14,
            Pin::P15 => 15,
            Pin::P16 => 16,
        }
    }

    fn bitmask(&self) -> u32 {
        let pin = self.index();
        1 << pin
    }
}

#[entry]
fn main() -> ! {
    let p = Peripherals::take().expect("only called once");

    const PINS: [Pin; 4] = [P13, P14, P16, P15];

    // Set all pins to high (led=off) to start

    let mut bitmask: u32 = 0;

    for pin in PINS {
        bitmask |= pin.bitmask();
    }

    p.P0.outset.write(|w| unsafe { w.bits(bitmask) });

    // Configure all pins as output

    for pin in PINS {
        p.P0.pin_cnf[pin.index()].write(|w| w.dir().output());
    }

    // Drive pins up and down in cycle

    let mut pin_index = 0;

    loop {
        let pin = PINS[pin_index];
        pin_index = (pin_index + 1) % 4;

        p.P0.outclr.write(|w| unsafe { w.bits(pin.bitmask()) });
        wait();

        p.P0.outset.write(|w| unsafe { w.bits(pin.bitmask()) });
        wait();
    }
}

fn wait() {
    for _ in 0..200_000 {
        nop();
    }
}
