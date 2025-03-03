use embedded_hal::digital::InputPin;
use fugit::ExtU64;
use nrf52840_hal::gpio::{Input, Pin, PullUp};

use crate::{
    channel::Sender,
    grid::Position,
    time::{Ticker, Timer},
};

enum ButtonState<'a> {
    WaitForPress,
    Debounce(Timer<'a>),
}

pub struct ButtonTask<'a> {
    pin: Pin<Input<PullUp>>,
    ticker: &'a Ticker,
    position: Position,
    state: ButtonState<'a>,
    sender: Sender<'a, Position>,
}

impl<'a> ButtonTask<'a> {
    pub fn new(
        pin: Pin<Input<PullUp>>,
        ticker: &'a Ticker,
        position: Position,
        sender: Sender<'a, Position>,
    ) -> Self {
        Self {
            pin,
            ticker,
            position,
            state: ButtonState::WaitForPress,
            sender,
        }
    }

    pub fn poll(&mut self) {
        match self.state {
            ButtonState::WaitForPress => {
                if self.pin.is_low().expect("can read button input") {
                    self.sender.send(self.position);
                    self.state = ButtonState::Debounce(Timer::new(100.millis(), &self.ticker));
                }
            }
            ButtonState::Debounce(ref timer) => {
                if timer.is_ready() && self.pin.is_high().expect("can read button input") {
                    self.state = ButtonState::WaitForPress;
                }
            }
        }
    }
}
