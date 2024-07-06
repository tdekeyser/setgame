use itertools::Itertools;
use rand::prelude::SliceRandom;
use rand::Rng;
use serde::Serialize;

use crate::card::Card;
use crate::triple::{SetStatus, Triple};

#[derive(Serialize)]
pub struct CardGame(Vec<Card>);

impl CardGame {
    pub fn generate_fixed(num_cards: usize, num_sets: usize) -> Self {
        let deck = Card::build_deck();
        let mut rng = rand::thread_rng();
        let mut g = CardGame::generate(&deck, num_cards, &mut rng);

        let mut count = 1;

        while g.count_sets() != num_sets {
            g = CardGame::generate(&deck, num_cards, &mut rng);
            count += 1;
        }

        println!("[game] generated {num_cards} cards with {num_sets} sets after {count} tries.");

        g
    }

    fn generate(deck: &Vec<Card>, num_cards: usize, rng: &mut impl Rng) -> Self {
        let mut idx: Vec<usize> = (0..deck.len()).collect();
        idx.shuffle(rng);

        Self(
            idx[0..num_cards].iter()
                .map(|idx| deck[*idx].clone())
                .collect()
        )
    }

    fn count_sets(&self) -> usize {
        self.0.iter().combinations(3)
            .map(|v| Triple::new(v[0].clone(), v[1].clone(), v[2].clone()))
            .filter(|t| t.is_a_set() == SetStatus::IsASet)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use crate::card::Card;
    use crate::card::Color::{Green, Purple, Red};
    use crate::card::Number::{One, Three, Two};
    use crate::card::Shading::Solid;
    use crate::card::Shape::{Diamond, Oval, Squiggle};
    use crate::game::CardGame;

    #[test]
    fn generate_a_game() {
        let mut rng = rand::thread_rng();
        assert_eq!(CardGame::generate(&Card::build_deck(), 12, &mut rng).0.len(), 12);
    }

    #[test]
    fn count_sets_in_a_game() {
        let game = CardGame(vec![
            Card::new(Three, Solid, Red, Diamond),
            Card::new(Two, Solid, Green, Squiggle),
            Card::new(One, Solid, Purple, Oval),
            Card::new(One, Solid, Purple, Diamond),
            Card::new(One, Solid, Purple, Squiggle),
        ]);

        assert_eq!(game.count_sets(), 2);
    }

    #[test]
    fn count_fixed_sets_in_a_game() {
        let game = CardGame::generate_fixed(12, 6);

        assert_eq!(game.count_sets(), 6);
    }
}
