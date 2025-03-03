use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use fugit::ExtU64;
use nrf52840_hal::gpio::{Output, Pin, PushPull};
use rtt_target::rprintln;

use crate::{
    channel::Receiver,
    grid::{Grid, Position},
    time::{Ticker, Timer},
};

enum LedState<'a> {
    Toggle,
    Wait(Timer<'a>),
}

pub struct LedTask<'a> {
    grid: Grid<Pin<Output<PushPull>>>,
    position: Position,
    ticker: &'a Ticker,
    state: LedState<'a>,
    receiver: Receiver<'a, Position>,
}

impl<'a> LedTask<'a> {
    pub fn new(
        grid: Grid<Pin<Output<PushPull>>>,
        ticker: &'a Ticker,
        receiver: Receiver<'a, Position>,
    ) -> Self {
        Self {
            grid,
            position: Position::TopLeft,
            ticker,
            state: LedState::Toggle,
            receiver,
        }
    }

    pub fn poll(&mut self) {
        match self.state {
            LedState::Toggle => {
                rprintln!("Blinking LED {}", self.position);
                self.grid.get_mut(self.position).toggle().ok();
                // Start timer
                self.state = LedState::Wait(Timer::new(500.millis(), &self.ticker));
            }
            LedState::Wait(ref timer) => {
                if timer.is_ready() {
                    self.state = LedState::Toggle;
                }
                if let Some(pos) = self.receiver.receive() {
                    self.move_to(pos);
                    self.state = LedState::Toggle;
                }
            }
        }
    }

    fn move_to(&mut self, pos: Position) {
        rprintln!("Moving to {}", pos);
        // turn off current LED
        self.grid.get_mut(self.position).set_high().ok();
        // set new LED
        self.position = pos;
        // turn off new LED
        self.grid.get_mut(self.position).set_high().ok();
    }
}
