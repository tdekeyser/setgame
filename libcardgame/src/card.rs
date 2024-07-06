use std::hash::Hash;

use serde::{Deserialize, Serialize};

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

#[derive(Clone, Serialize, Deserialize)]
pub struct Card(Number, Shading, Color, Shape);

impl Card {
    pub fn build_deck() -> Vec<Self> {
        let mut cards = Vec::with_capacity(81);

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

    pub fn new(number: Number, shading: Shading, color: Color, shape: Shape) -> Self {
        Self(number, shading, color, shape)
    }

    pub fn number(&self) -> &Number {
        &self.0
    }

    pub fn shading(&self) -> &Shading {
        &self.1
    }

    pub fn color(&self) -> &Color {
        &self.2
    }

    pub fn shape(&self) -> &Shape {
        &self.3
    }
}

#[cfg(test)]
mod tests {
    use crate::card::Card;

    #[test]
    fn deck_has_all_possible_cards() {
        let deck = Card::build_deck();
        assert_eq!(deck.len(), 81);
    }
}
