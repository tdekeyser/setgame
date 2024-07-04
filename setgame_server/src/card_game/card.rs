use std::hash::Hash;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Card(pub Number, pub Shading, pub Color, pub Shape);

impl Card {
    pub fn build_deck() -> Vec<Self> {
        let mut cards = Vec::new();

        for number in [Number::One, Number::Two, Number::Three] {
            for shading in [Shading::Solid, Shading::Striped, Shading::Open] {
                for color in [Color::Red, Color::Green, Color::Purple] {
                    for shape in [Shape::Squiggle, Shape::Diamond, Shape::Oval] {
                        cards.push(Self(number.clone(), shading.clone(), color.clone(), shape));
                    }
                }
            }
        }

        cards
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum Color {
    Red,
    Green,
    Purple,
}

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum Number {
    One,
    Two,
    Three,
}

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum Shape {
    Squiggle,
    Diamond,
    Oval,
}

#[derive(PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum Shading {
    Solid,
    Striped,
    Open,
}

#[cfg(test)]
mod tests {
    use crate::card_game::card::Card;

    #[test]
    fn deck_has_all_possible_cards() {
        let deck = Card::build_deck();
        assert_eq!(deck.len(), 81);
    }
}
