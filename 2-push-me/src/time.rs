use fugit::{Duration, Instant};
use nrf52840_hal::{pac::RTC0, Rtc};

type TickInstant = Instant<u64, 1, 32_768>;
type TickDuration = Duration<u64, 1, 32_768>;

pub struct Timer<'a> {
    ticker: &'a Ticker,
    end_time: TickInstant,
}

impl<'a> Timer<'a> {
    pub fn new(duration: TickDuration, ticker: &'a Ticker) -> Self {
        Self {
            ticker,
            end_time: ticker.now() + duration,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.ticker.now() >= self.end_time
    }
}

pub struct Ticker {
    rtc: Rtc<RTC0>,
}

impl Ticker {
    pub fn new(rtc0: RTC0) -> Self {
        let rtc = Rtc::new(rtc0, 0).expect("can create Rtc from RTC0");
        rtc.enable_counter();
        Self { rtc }
    }

    pub fn now(&self) -> TickInstant {
        TickInstant::from_ticks(self.rtc.get_counter() as u64)
    }
}
