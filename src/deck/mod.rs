#![allow(dead_code)]

mod test;

use super::card::Card;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Deck 
{
    deck: Vec<Card>
}

impl Deck {

    fn shuffle(&mut self) {
        
    }
}