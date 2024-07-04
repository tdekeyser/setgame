use std::collections::HashSet;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::card_game::card::{Card, Color, Number, Shading, Shape};

#[derive(PartialEq, Debug, Serialize)]
pub enum SetStatus {
    IsASet,
    NotASet(String),
}

#[derive(Deserialize)]
pub struct Triple(Card, Card, Card);

impl Triple {
    pub fn new(one: Card, two: Card, three: Card) -> Self {
        Self(one, two, three)
    }

    pub fn is_a_set(&self) -> SetStatus {
        let number_check = Number::all_same_or_all_different(&[&self.0.0, &self.1.0, &self.2.0]);
        let shading_check = Shading::all_same_or_all_different(&[&self.0.1, &self.1.1, &self.2.1]);
        let color_check = Color::all_same_or_all_different(&[&self.0.2, &self.1.2, &self.2.2]);
        let shape_check = Shape::all_same_or_all_different(&[&self.0.3, &self.1.3, &self.2.3]);

        if number_check && shading_check && color_check && shape_check {
            SetStatus::IsASet
        } else {
            let mut reasons = vec![];

            if !number_check {
                reasons.push("Numbers are not all the same or different.");
            }
            if !shading_check {
                reasons.push("Shadings are not all the same or different.");
            }
            if !color_check {
                reasons.push("Colors are not all the same or different.");
            }
            if !shape_check {
                reasons.push("Shapes are not all the same or different.");
            }

            SetStatus::NotASet(reasons.join(" "))
        }
    }
}

trait AllSameOrAllDifferent<F>
    where
        F: Eq + Hash + AllSameOrAllDifferent<F>,
{
    fn all_same_or_all_different(features: &[&F]) -> bool {
        F::all_same(features) || F::all_different(features)
    }

    fn all_same(features: &[&F]) -> bool {
        if let Some(first) = features.first() {
            features.iter().all(|item| item == first)
        } else {
            true
        }
    }

    fn all_different(features: &[&F]) -> bool {
        features.len() == features.iter().collect::<HashSet<_>>().len()
    }
}

impl AllSameOrAllDifferent<Color> for Color {}

impl AllSameOrAllDifferent<Number> for Number {}

impl AllSameOrAllDifferent<Shape> for Shape {}

impl AllSameOrAllDifferent<Shading> for Shading {}


#[cfg(test)]
mod tests {
    use crate::card_game::card::{Color, Number};
    use crate::card_game::card::Color::*;
    use crate::card_game::card::Number::*;
    use crate::card_game::card::Shading::*;
    use crate::card_game::card::Shape::*;
    use crate::card_game::triple::{AllSameOrAllDifferent, Card, SetStatus, Triple};

    #[test]
    fn is_set_all_similar_or_all_different() {
        assert_eq!(
            Triple(
                Card(Three, Solid, Red, Diamond),
                Card(Two, Solid, Green, Squiggle),
                Card(One, Solid, Purple, Oval),
            ).is_a_set(),
            SetStatus::IsASet
        )
    }

    #[test]
    fn not_a_set_not_all_different() {
        assert_eq!(
            Triple(
                Card(Three, Solid, Red, Diamond),
                Card(Three, Solid, Green, Squiggle),
                Card(One, Solid, Purple, Oval),
            ).is_a_set(),
            SetStatus::NotASet("Numbers are not all the same or different.".into()))
    }

    #[test]
    fn not_a_set_not_all_same() {
        assert_eq!(
            Triple(
                Card(Three, Solid, Red, Diamond),
                Card(Two, Solid, Green, Squiggle),
                Card(One, Open, Purple, Oval),
            ).is_a_set(),
            SetStatus::NotASet("Shadings are not all the same or different.".into())
        )
    }

    #[test]
    fn all_same() {
        assert!(Number::all_same(&[&Number::One, &Number::One, &Number::One]));
        assert!(!Color::all_same(&[&Color::Red, &Color::Green, &Color::Red]));
    }

    #[test]
    fn all_different() {
        assert!(!Number::all_different(&[&Number::One, &Number::One, &Number::One]));
        assert!(Color::all_different(&[&Color::Red, &Color::Green, &Color::Purple]));
    }
}
