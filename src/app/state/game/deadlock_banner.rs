/// Presentation-only timing in the application's 60 Hz wall-clock frame units.
#[derive(Default)]
pub(super) struct DeadlockBanner {
    observed: Option<usize>,
    started: Option<u64>,
}

impl DeadlockBanner {
    pub fn observe(&mut self, overcharge_at: Option<usize>, frame: u64) {
        if overcharge_at != self.observed {
            self.observed = overcharge_at;
            self.started = overcharge_at.map(|_| frame);
        }
    }

    /// 250 ms in from the left, 1000 ms centered, 250 ms out to the right.
    pub fn offset(&self, frame: u64, travel: f64) -> Option<f64> {
        let elapsed = frame.saturating_sub(self.started?);
        match elapsed {
            0..15 => Some(-travel * (1.0 - elapsed as f64 / 15.0)),
            15..75 => Some(0.0),
            75..90 => Some(travel * (elapsed - 75) as f64 / 15.0),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timing_repetition_and_undo() {
        let mut banner = DeadlockBanner::default();
        assert_eq!(banner.offset(100, 300.0), None);
        banner.observe(Some(8), 100);
        assert_eq!(banner.offset(100, 300.0), Some(-300.0));
        assert!((banner.offset(110, 300.0).unwrap() + 100.0).abs() < 1e-9);
        assert_eq!(banner.offset(115, 300.0), Some(0.0));
        assert_eq!(banner.offset(174, 300.0), Some(0.0));
        assert_eq!(banner.offset(185, 300.0), Some(200.0));
        assert_eq!(banner.offset(190, 300.0), None);
        banner.observe(Some(8), 200);
        assert_eq!(banner.offset(200, 300.0), None);
        banner.observe(None, 201);
        banner.observe(Some(8), 202);
        assert_eq!(banner.offset(202, 300.0), Some(-300.0));
        banner.observe(None, 203);
        assert_eq!(banner.offset(203, 300.0), None);
    }
}
