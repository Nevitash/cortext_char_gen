use derive_builder::Builder;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
pub struct Die {
    sides: i32,
    effect_die: bool,
    modifier: i32,
}

impl Default for Die {
    fn default() -> Self {
        Self {
            sides: 6,
            effect_die: false,
            modifier: 0,
        }
    }
}

impl Die {
    pub fn roll(&self) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=self.get_current_sides())
    }
    pub fn step_up(&mut self) -> i32 {
        if self.sides + self.modifier + 2 <= 12 {
            self.modifier += 2;
        }
        self.get_current_sides()
    }
    pub fn step_down(&mut self) -> i32 {
        if self.sides + self.modifier - 2 >= 4 {
            self.modifier -= 2;
        }
        self.get_current_sides()
    }

    pub fn get_current_sides(&self) -> i32 {
        return self.sides + self.modifier;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d6_test(){
        let die = Die::default();
        let result = die.roll();
        assert!(result >= 1 && result <= 6)
    }

    #[test]
    fn d12_test(){
        let mut die = Die::default();
        die.step_up();
        die.step_up();
        die.step_up();
        let result = die.roll();
        assert!(die.get_current_sides() == 12);
        assert!(result >= 1 && result <= 12)
    }

    #[test]
    fn step_up_too_much_test(){
        let mut die = Die::default();
        die.step_up();
        die.step_up();
        die.step_up();
        die.step_up();
        let result = die.roll();
        assert!(die.get_current_sides() == 12);
        assert!(result >= 1 && result <= 12)
    }

    #[test]
    fn d4_test(){
        let mut die = Die::default();
        die.step_down();
        let result = die.roll();
        assert!(die.get_current_sides() == 4);
        assert!(result >= 1 && result <= 4)
    }

    #[test]
    fn step_down_too_much_test(){
        let mut die = Die::default();
        die.step_down();
        die.step_down();
        let result = die.roll();
        assert!(die.get_current_sides() == 4);
        assert!(result >= 1 && result <= 4)
    }

    #[test]
    fn step_up_and_step_down_test(){
        let mut die = Die::default();
        assert!(die.get_current_sides() == 6);
        let mut result = die.roll();
        assert!(result >= 1 && result <= 6);
        die.step_down();
        assert!(die.get_current_sides() == 4);
        result = die.roll();
        assert!(result >= 1 && result <= 4);
        die.step_up();
        assert!(die.get_current_sides() == 6);
        result = die.roll();
        assert!(result >= 1 && result <= 6);
        die.step_up();
        assert!(die.get_current_sides() == 8);
        result = die.roll();
        assert!(result >= 1 && result <= 8);
    }
}
