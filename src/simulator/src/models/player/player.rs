use crate::core::{EventType, SimulationContext};
use crate::models::shared::fullname::FullName;
use crate::player::skills::*;
use crate::utils::{DateUtils, IntegerUtils};
use std::fmt::{Display, Formatter, Result};

use chrono::NaiveDate;

pub struct Player {
      id: u32,
      full_name: FullName,
      birth_date: NaiveDate,
      skills: PlayerSkills,
      prefered_foot: PlayerFoot
}

impl Player {
      pub fn new(
            id: u32,
            full_name: FullName,
            birth_date: NaiveDate,
            skills: PlayerSkills,
      ) -> Player {
            Player {
                  id: id,                  
                  full_name: full_name,
                  birth_date: birth_date,
                  skills: skills,
                  prefered_foot: PlayerFoot::Right
            }
      }

      pub fn simulate(&mut self, context: &mut SimulationContext) {
            if DateUtils::is_birthday(self.birth_date, context.date) {
                  context.send(EventType::Birthday(self.id));
            }

            let change_val = IntegerUtils::random(-3,3) as u8;

            self.skills.train(change_val);
      }
}

pub enum PlayerFoot{
      Left,
      Right,
      Both
}

//DISPLAY
impl Display for Player {
      fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}, {}", self.full_name, self.birth_date)
      }
}