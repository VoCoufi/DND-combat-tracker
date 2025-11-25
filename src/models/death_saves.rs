use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeathSaves {
    pub successes: u8,
    pub failures: u8,
    pub is_stable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeathSaveOutcome {
    Ongoing,
    Stabilized,
    Died,
    Revived,
}

impl DeathSaves {
    pub fn add_success(&mut self) -> DeathSaveOutcome {
        if self.successes < 3 {
            self.successes += 1;
        }

        if self.successes >= 3 {
            self.is_stable = true;
            DeathSaveOutcome::Stabilized
        } else {
            DeathSaveOutcome::Ongoing
        }
    }

    pub fn add_failure(&mut self, count: u8) -> DeathSaveOutcome {
        self.is_stable = false;
        self.failures = (self.failures + count).min(3);

        if self.failures >= 3 {
            DeathSaveOutcome::Died
        } else {
            DeathSaveOutcome::Ongoing
        }
    }
}
