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

    pub fn description(&self) -> &'static str {
        match self {
            ConditionType::Blinded => {
                "Automatically fails sight-based checks; attack rolls against have advantage; their attacks have disadvantage."
            }
            ConditionType::Charmed => {
                "Can't attack charmer; charmer has advantage on social checks."
            }
            ConditionType::Deafened => "Automatically fails hearing-based checks.",
            ConditionType::Frightened => {
                "Disadvantage on ability checks/attacks while source in sight; can't willingly move closer to source."
            }
            ConditionType::Grappled => {
                "Speed becomes 0; ends if grappler is incapacitated or moved away."
            }
            ConditionType::Incapacitated => "Can't take actions or reactions.",
            ConditionType::Invisible => {
                "Can't be seen without magic; attacks against have disadvantage; their attacks have advantage."
            }
            ConditionType::Paralyzed => {
                "Incapacitated; can't move/speak; auto fail Str/Dex saves; attacks have advantage and crit within 5 ft."
            }
            ConditionType::Petrified => {
                "Transformed to stone; incapacitated; attacks have advantage; resists all damage; immune to poison/disease."
            }
            ConditionType::Poisoned => "Disadvantage on attack rolls and ability checks.",
            ConditionType::Prone => {
                "Only crawl; attacks vs them have advantage if within 5 ft, otherwise disadvantage; their attacks have disadvantage."
            }
            ConditionType::Restrained => {
                "Speed 0; attacks vs have advantage; their attacks have disadvantage; Dex saves at disadvantage."
            }
            ConditionType::Stunned => {
                "Incapacitated; can't move; can speak falteringly; auto fail Str/Dex saves; attacks have advantage."
            }
            ConditionType::Unconscious => {
                "Incapacitated; drops prone; drops what holds; auto fail Str/Dex saves; attacks have advantage and crit within 5 ft."
            }
        }
    }

    /// Returns concise mechanical effects summary for combat reference
    pub fn mechanical_effects(&self) -> &'static str {
        match self {
            ConditionType::Blinded => "Attacks: disadv; Attacks vs: adv; Fails sight checks",
            ConditionType::Charmed => "Can't attack charmer; Charmer: adv on social",
            ConditionType::Deafened => "Fails hearing checks",
            ConditionType::Frightened => "Attacks/checks: disadv; Can't move closer",
            ConditionType::Grappled => "Speed: 0",
            ConditionType::Incapacitated => "No actions/reactions",
            ConditionType::Invisible => "Attacks: adv; Attacks vs: disadv",
            ConditionType::Paralyzed => "Attacks vs: adv + crit (5ft); Fails STR/DEX saves",
            ConditionType::Petrified => "Attacks vs: adv; Resist all damage",
            ConditionType::Poisoned => "Attacks/checks: disadv",
            ConditionType::Prone => "Attacks vs: adv (melee 5ft), disadv (ranged); Attacks: disadv",
            ConditionType::Restrained => {
                "Speed: 0; Attacks vs: adv; Attacks: disadv; DEX saves: disadv"
            }
            ConditionType::Stunned => "Attacks vs: adv; Fails STR/DEX saves",
            ConditionType::Unconscious => {
                "Prone; Attacks vs: adv + crit (5ft); Fails STR/DEX saves"
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timed_status_expires_below_zero() {
        let mut s = StatusEffect::new(ConditionType::Poisoned, 2, None);
        s.decrement_duration();
        assert_eq!(s.duration, 1);
        assert!(!s.is_expired());
        s.decrement_duration();
        assert_eq!(s.duration, -1);
        assert!(s.is_expired());
    }

    #[test]
    fn indefinite_status_not_expired() {
        let s = StatusEffect::new(ConditionType::Prone, 0, None);
        assert!(!s.is_expired());
    }

    #[test]
    fn all_conditions_have_mechanical_effects() {
        // Verify all 14 conditions have mechanical effects
        let all_conditions = ConditionType::all();
        assert_eq!(all_conditions.len(), 14);

        for condition in all_conditions {
            let effects = condition.mechanical_effects();
            assert!(
                !effects.is_empty(),
                "Condition {:?} has no mechanical effects",
                condition
            );
        }
    }

    #[test]
    fn mechanical_effects_are_concise() {
        // Verify mechanical effects are reasonably short (max 80 chars to prevent overflow)
        let all_conditions = ConditionType::all();

        for condition in all_conditions {
            let effects = condition.mechanical_effects();
            assert!(
                effects.len() <= 80,
                "Condition {:?} mechanical effects too long ({} chars): {}",
                condition,
                effects.len(),
                effects
            );
        }
    }
}
