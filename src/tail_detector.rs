#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TailClassification {
    Normal,
    Tail,
}

#[derive(Clone, Copy, Debug)]
pub struct RobustReference {
    pub median: u64,
    pub mad: u64,
    pub k: u32,
}

impl RobustReference {
    pub fn threshold(&self) -> u64 {
        let scale = (self.mad as f64) * 1.4826;
        let threshold = (self.median as f64) + (self.k as f64) * scale;
        threshold.round() as u64
    }

    pub fn classify(&self, latency_ticks: u64) -> TailClassification {
        if latency_ticks > self.threshold() {
            TailClassification::Tail
        } else {
            TailClassification::Normal
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_frozen_threshold() {
        let reference = RobustReference {
            median: 1_682_149,
            mad: 531_957,
            k: 5,
        };

        assert_eq!(reference.threshold(), 5_625_546);
    }

    #[test]
    fn classifies_latency_above_threshold_as_tail() {
        let reference = RobustReference {
            median: 1_682_149,
            mad: 531_957,
            k: 5,
        };

        assert_eq!(
            reference.classify(5_625_547),
            TailClassification::Tail
        );
    }

    #[test]
    fn classifies_latency_at_threshold_as_normal() {
        let reference = RobustReference {
            median: 1_682_149,
            mad: 531_957,
            k: 5,
        };

        assert_eq!(
            reference.classify(reference.threshold()),
            TailClassification::Normal
        );
    }
}
