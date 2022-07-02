use std::time::Duration;
use std::time::Instant;

// SDL2 à déjà un système d'horloge,
// mais elle ne fonctionne pas exactement comment je voudrais donc j'ai refait la mienne

pub struct Clock {
    start: Instant,
    delay: Duration,
    next_tick: u32,
}

impl Clock {
    pub fn new(delay: Duration) -> Self {
        Self {
            start: Instant::now(),
            delay,
            next_tick: 1,
        }
    }

    pub fn tick(&mut self) -> bool {
        if self.start.elapsed() > self.delay * self.next_tick {
            self.next_tick += 1;
            true
        } else {
            false
        }
    }
}
