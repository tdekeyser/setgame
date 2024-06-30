use std::collections::HashSet;
use std::hash::Hash;

use crate::{Error, Result};

#[derive(PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Purple,
}

#[derive(PartialEq, Eq, Hash)]
enum Number {
    One,
    Two,
    Three,
}

#[derive(PartialEq, Eq, Hash)]
enum Shape {
    Squiggle,
    Diamond,
    Oval,
}

#[derive(PartialEq, Eq, Hash)]
enum Shading {
    Solid,
    Striped,
    Open,
}

struct Card(Number, Shading, Color, Shape);

struct Triple(Card, Card, Card);

impl Triple {
    fn is_set(&self) -> Result<bool> {
        let number_check = Number::all_same_or_all_different(&[&self.0.0, &self.1.0, &self.2.0]);
        let shading_check = Shading::all_same_or_all_different(&[&self.0.1, &self.1.1, &self.2.1]);
        let color_check = Color::all_same_or_all_different(&[&self.0.2, &self.1.2, &self.2.2]);
        let shape_check = Shape::all_same_or_all_different(&[&self.0.3, &self.1.3, &self.2.3]);

        if number_check && shading_check && color_check && shape_check {
            Ok(true)
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

            Err(Error::NotASet(reasons.join(" ")))
        }
    }
}

trait AllSameOrAllDifferent<F> where F: Eq + Hash + AllSameOrAllDifferent<F> {
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
    use crate::domain::*;
    use crate::domain::Color::*;
    use crate::domain::Number::*;
    use crate::domain::Shading::*;
    use crate::domain::Shape::*;

    #[test]
    fn is_set_all_similar_or_all_different() -> Result<()> {
        Triple(
            Card(Three, Solid, Red, Diamond),
            Card(Two, Solid, Green, Squiggle),
            Card(One, Solid, Purple, Oval),
        ).is_set()?;

        Ok(())
    }

    #[test]
    #[should_panic(expected = "Numbers are not all the same or different.")]
    fn not_a_set_not_all_different() {
        Triple(
            Card(Three, Solid, Red, Diamond),
            Card(Three, Solid, Green, Squiggle),
            Card(One, Solid, Purple, Oval),
        ).is_set().unwrap();
    }

    #[test]
    #[should_panic(expected = "Shadings are not all the same or different.")]
    fn not_a_set_not_all_same() {
        Triple(
            Card(Three, Solid, Red, Diamond),
            Card(Two, Solid, Green, Squiggle),
            Card(One, Open, Purple, Oval),
        ).is_set().unwrap();
    }
}
