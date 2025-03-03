#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52840_hal::{
    clocks::LfOscConfiguration,
    gpio::{self, Level},
    pac, Clocks,
};
use panic_halt as _;
use rtt_target::rtt_init_print;

mod button;
mod channel;
mod grid;
mod led;
mod time;

use button::ButtonTask;
use channel::Channel;
use grid::{Grid, Position};
use led::LedTask;
use time::Ticker;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let p = pac::Peripherals::take().expect("only called once");
    let port0 = gpio::p0::Parts::new(p.P0);
    Clocks::new(p.CLOCK)
        .set_lfclk_src_external(LfOscConfiguration::NoExternalNoBypass)
        .start_lfclk();

    let ticker = Ticker::new(p.RTC0);
    let channel: Channel<Position> = Channel::new();

    let led_grid = Grid::new([
        port0.p0_13.into_push_pull_output(Level::High).degrade(),
        port0.p0_14.into_push_pull_output(Level::High).degrade(),
        port0.p0_16.into_push_pull_output(Level::High).degrade(),
        port0.p0_15.into_push_pull_output(Level::High).degrade(),
    ]);
    let button_grid = Grid::new([
        port0.p0_11.into_pullup_input().degrade(),
        port0.p0_12.into_pullup_input().degrade(),
        port0.p0_25.into_pullup_input().degrade(),
        port0.p0_24.into_pullup_input().degrade(),
    ]);

    let mut led_task = LedTask::new(led_grid, &ticker, channel.get_receiver());
    let mut button_task_grid =
        button_grid.map(|pin, pos| ButtonTask::new(pin, &ticker, pos, channel.get_sender()));

    loop {
        led_task.poll();
        for button_task in &mut button_task_grid {
            button_task.poll();
        }
    }
}
