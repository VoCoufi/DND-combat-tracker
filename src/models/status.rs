use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionType {
    Blinded,
    Charmed,
    Deafened,
    Frightened,
    Grappled,
    Incapacitated,
    Invisible,
    Paralyzed,
    Petrified,
    Poisoned,
    Prone,
    Restrained,
    Stunned,
    Unconscious,
}

impl ConditionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConditionType::Blinded => "Blinded",
            ConditionType::Charmed => "Charmed",
            ConditionType::Deafened => "Deafened",
            ConditionType::Frightened => "Frightened",
            ConditionType::Grappled => "Grappled",
            ConditionType::Incapacitated => "Incapacitated",
            ConditionType::Invisible => "Invisible",
            ConditionType::Paralyzed => "Paralyzed",
            ConditionType::Petrified => "Petrified",
            ConditionType::Poisoned => "Poisoned",
            ConditionType::Prone => "Prone",
            ConditionType::Restrained => "Restrained",
            ConditionType::Stunned => "Stunned",
            ConditionType::Unconscious => "Unconscious",
        }
    }

    pub fn all() -> Vec<ConditionType> {
        vec![
            ConditionType::Blinded,
            ConditionType::Charmed,
            ConditionType::Deafened,
            ConditionType::Frightened,
            ConditionType::Grappled,
            ConditionType::Incapacitated,
            ConditionType::Invisible,
            ConditionType::Paralyzed,
            ConditionType::Petrified,
            ConditionType::Poisoned,
            ConditionType::Prone,
            ConditionType::Restrained,
            ConditionType::Stunned,
            ConditionType::Unconscious,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusEffect {
    pub condition: ConditionType,
    pub duration: i32,
    pub source: Option<String>,
}

impl StatusEffect {
    pub fn new(condition: ConditionType, duration: i32, source: Option<String>) -> Self {
        Self {
            condition,
            duration,
            source,
        }
    }

    pub fn decrement_duration(&mut self) {
        if self.duration > 0 {
            self.duration -= 1;
            if self.duration == 0 {
                // Move to negative to mark as expired after ticking down from a timed duration
                self.duration = -1;
            }
        }
    }

    pub fn is_expired(&self) -> bool {
        self.duration < 0
    }
}
