#![allow(dead_code)]

mod test;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Card 
{
    power: i32
}

impl Card {

    fn battle(&self, opponent: &Card) -> Card {
        if self.power > opponent.power {
            *self
        }
        else {
            *opponent
        }
    }
}