// https://github.com/sulami/arduino-nano-33/blob/master/src/time.rs
use arduino_nano33iot as bsp;
use bsp::hal::clock::{GClock, GenericClockController};
use bsp::hal::prelude::*;
use bsp::hal::timer::{TimerCounter, TimerCounter5};

macro_rules! from {
    ($stone:expr) => { {
        let gclk0 = $stone.clock.gclk0();

        base::components::timer::Timer::new(
            $stone.tc5,
            &mut $stone.clock,
            &gclk0,
            &mut $stone.pm
        )
    } };
}
pub(crate) use from;

pub struct Timer {
    tc: TimerCounter5,
    millis: u64,
}

impl Timer {
    pub fn new(
        tc5: bsp::pac::TC5,
        clocks: &mut GenericClockController,
        gclk0: &GClock,
        pm: &mut bsp::pac::PM,
    ) -> Self {
        let timer_clock = clocks.tc4_tc5(gclk0).unwrap();
        let mut timer = TimerCounter::tc5_(&timer_clock, tc5, pm);
        timer.start(1.khz());
        Timer {
            tc: timer,
            millis: 0,
        }
    }

    pub fn tick(&mut self) {
        nb::block!(self.tc.wait()).ok();
        self.millis += 1;
    }

    pub fn millis(&self) -> u64 {
        self.millis
    }
}
