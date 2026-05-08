use serde::Serialize;
use std::time::Instant;

#[derive(Clone, Serialize)]
pub struct Bucket {
    pub tokens: f64,
    pub refill_rate: f64,
    pub capacity: f64,

    #[serde(skip)]
    last_refill: Instant,
}

impl Bucket {
    pub fn new(capacity: f64, refill_rate: f64) -> Self {
        Self {
            tokens: capacity,
            refill_rate,
            capacity,
            last_refill: Instant::now(),
        }
    }

    pub fn try_consume(&mut self) -> bool {
        if self.can_consume() {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    fn can_consume(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();

        self.tokens = (self.tokens + elapsed * self.refill_rate)
            .min(self.capacity)
            .max(0.0);

        self.last_refill = now;

        self.tokens >= 1.0
    }
}