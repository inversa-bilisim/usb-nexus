// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 Demli contributors

//! Exponential backoff with jitter for reconnect attempts.

use std::time::Duration;

use rand::Rng;

#[derive(Debug, Clone)]
pub struct Backoff {
    min: Duration,
    max: Duration,
    current: Duration,
}

impl Default for Backoff {
    fn default() -> Self {
        Backoff::new(Duration::from_millis(500), Duration::from_secs(30))
    }
}

impl Backoff {
    pub fn new(min: Duration, max: Duration) -> Self {
        Backoff { min, max, current: min }
    }

    /// Returns the next delay (with ±20% jitter) and doubles the base delay.
    pub fn next_delay(&mut self) -> Duration {
        let base = self.current;
        self.current = (self.current * 2).min(self.max);
        let jitter = rand::thread_rng().gen_range(0.8..1.2);
        base.mul_f64(jitter).min(self.max)
    }

    /// Called after a successful connection.
    pub fn reset(&mut self) {
        self.current = self.min;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grows_and_caps() {
        let mut b = Backoff::new(Duration::from_secs(1), Duration::from_secs(8));
        let d: Vec<_> = (0..6).map(|_| b.next_delay()).collect();
        assert!(d[0] <= Duration::from_millis(1200));
        assert!(d[3] >= Duration::from_millis(6400));
        assert!(d.iter().all(|x| *x <= Duration::from_secs(8)));
        b.reset();
        assert!(b.next_delay() <= Duration::from_millis(1200));
    }
}
