#[macro_use]
extern crate lazy_static;

// #[macro_use] extern crate lalrpop_util;
// lalrpop_mod!(pub parser); // synthesized by LALRPOP

pub mod lib {
  use rand::{rngs::SmallRng, Rng, SeedableRng};
  use regex::{Regex, Captures};
  use evalexpr::eval;

  // TODO: try generating docs
  pub struct Dice {
    rolls: u32,
    sides: u32,
  }
  impl Dice {
    pub fn new(rolls: u32, sides: u32) -> Dice {
      return Dice {
        rolls: rolls,
        sides: sides,
      };
    }

    pub fn from_string(string: &String) -> Dice {
      // parse into rolls and sides, with regex validation
      lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^(\d+)d(\d+)$").unwrap();
      }

      let captures = PATTERN.captures(string).unwrap();

      // Parse the captures as u32s.
      let rolls = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
      let sides = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();

      return Dice::new(rolls, sides);
    }

    pub fn roll(&self, random_seed: Option<u64>) -> u32 {
      // TODO: may be better to move this to a static area instead of making it
      // for every roll
      let mut rng = match random_seed {
        Some(inner) => SmallRng::seed_from_u64(inner),
        None => SmallRng::from_entropy(),
      };

      let mut result = 0;
      for _ in 0..self.rolls {
        result += rng.gen_range(1, self.sides);
      }
      return result;
    }
  }

  // TODO: errors need to bubble up properly and not panic
  pub fn solve_dice_expression(expression: String, random_seed: Option<u64>) -> i64 {
    let pattern = Regex::new(r"(\d+)d(\d+)").unwrap();
    
    // For every match on the Dice expression regex, roll it in-place.
    let rolled_expression = pattern.replace(&expression, |caps: &Captures| {
      let dice = Dice::from_string(&caps.get(0).unwrap().as_str().to_string());
      return format!("{}", dice.roll(random_seed));
    });

    // Calculate the result
    let result = match eval(&rolled_expression).unwrap() {
      evalexpr::Value::Int(inner) => inner,
      _ => panic!("Not an int, something went wrong")
    };

    return result;
  }

  #[cfg(test)]
  mod tests {
    use crate::lib::*;

    const TEST_SEED: u64 = 42;

    #[test]
    fn solve_dice_expression_can_do_basic_math() {
      assert_eq!(4, solve_dice_expression(String::from("2 + 2"), None));
    }

    #[test]
    fn seeded_rolls_are_deterministic() {
      let seed = Some(TEST_SEED);
      let rolls = ["2d6", "1d20", "2d8", "9d4", "1d12"];
      for s in &rolls {
        let a = Dice::from_string(&s.to_string());
        let b = Dice::from_string(&s.to_string());
        assert_eq!(a.roll(seed), b.roll(seed));
      }
    }


  }
}
