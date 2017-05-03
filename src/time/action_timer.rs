use timer::Timer;
use timer::Guard;
use chrono::Duration;

pub struct ActionTimer {
    timer: Option<Timer>,
    guard: Option<Guard>,
}

impl ActionTimer {
    pub fn new(timer: Timer) -> ActionTimer {
        ActionTimer {
            timer: Some(timer),
            guard: None,
        }
    }

    pub fn schedule<F>(&mut self, cb: F)
        where F: 'static  + FnMut() + Send {
        match self.guard {
            Some(ref g) => drop(g),
            None => (),
        }

        match self.timer {
            Some(ref t) => {
                self.guard = Some(t.schedule_repeating(Duration::seconds(2), cb));
            },
            None => (),
        }
    }
}

