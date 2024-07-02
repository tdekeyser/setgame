use itertools::Itertools;
use rand::Rng;
use serde::Serialize;

use crate::card_game::card::{Card, SetStatus, Triple};

#[derive(Serialize)]
pub struct CardGame(Vec<Card>);

impl CardGame {
    pub fn generate_fixed(num_cards: usize, num_sets: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut g = CardGame::generate(num_cards, &mut rng);

        while g.count_sets() != num_sets {
            g = CardGame::generate(num_cards, &mut rng);
        }

        g
    }

    fn generate(num_cards: usize, rng: &mut impl Rng) -> Self {
        Self(
            (0..num_cards)
                .map(|_| Card::random(rng))
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
    use crate::card_game::card::Card;
    use crate::card_game::features::Color::{Green, Purple, Red};
    use crate::card_game::features::Number::{One, Three, Two};
    use crate::card_game::features::Shading::Solid;
    use crate::card_game::features::Shape::{Diamond, Oval, Squiggle};
    use crate::card_game::game::CardGame;

    #[test]
    fn generate_a_game() {
        let mut rng = rand::thread_rng();
        assert_eq!(CardGame::generate(12, &mut rng).0.len(), 12);
    }

    #[test]
    fn count_sets_in_a_game() {
        let game = CardGame(vec![
            Card(Three, Solid, Red, Diamond),
            Card(Two, Solid, Green, Squiggle),
            Card(One, Solid, Purple, Oval),
            Card(One, Solid, Purple, Diamond),
            Card(One, Solid, Purple, Squiggle),
        ]);

        assert_eq!(game.count_sets(), 2);
    }

    #[test]
    fn count_fixed_sets_in_a_game() {
        let game = CardGame::generate_fixed(12, 6);

        assert_eq!(game.count_sets(), 6);
    }
}
