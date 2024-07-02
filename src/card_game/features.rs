use std::collections::HashSet;
use std::hash::Hash;

use rand::prelude::Distribution;
use rand::distributions::Standard;
use rand::Rng;
use serde::Serialize;

#[derive(PartialEq, Eq, Hash, Clone, Serialize)]
pub enum Color {
    Red,
    Green,
    Purple,
}

#[derive(PartialEq, Eq, Hash, Clone, Serialize)]
pub enum Number {
    One,
    Two,
    Three,
}

#[derive(PartialEq, Eq, Hash, Clone, Serialize)]
pub enum Shape {
    Squiggle,
    Diamond,
    Oval,
}

#[derive(PartialEq, Eq, Hash, Clone, Serialize)]
pub enum Shading {
    Solid,
    Striped,
    Open,
}

impl AllSameOrAllDifferent<Color> for Color {}

impl AllSameOrAllDifferent<Number> for Number {}

impl AllSameOrAllDifferent<Shape> for Shape {}

impl AllSameOrAllDifferent<Shading> for Shading {}


pub trait AllSameOrAllDifferent<F>
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

macro_rules! impl_distribution {
    ($enum_name:ident, $($variant:ident),+) => {
        impl Distribution<$enum_name> for Standard {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $enum_name {
                match rng.gen_range(0..=2) {
                    $(x if x == $enum_name::$variant as u32 => $enum_name::$variant,)+
                    _ => unreachable!(),
                }
            }
        }
    };
}

impl_distribution!(Color, Red, Green, Purple);
impl_distribution!(Number, One, Two, Three);
impl_distribution!(Shape, Squiggle, Diamond, Oval);
impl_distribution!(Shading, Solid, Striped, Open);

#[cfg(test)]
mod tests {
    use rand::Rng;
    use crate::card_game::features::{AllSameOrAllDifferent, Color, Number, Shading};

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

    #[test]
    fn generate_random() {
        let mut rng = rand::thread_rng();
        let shade: Shading = rng.gen();
        assert!(shade == Shading::Striped || shade == Shading::Open || shade == Shading::Solid);
    }

}
