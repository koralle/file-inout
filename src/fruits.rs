#[cfg(test)]
use speculate;

#[cfg(test)]
use rstest::*;

use std::str::FromStr;
pub mod fruits {

    #[derive(Debug, PartialEq)]
    pub enum Fruit {
        Apple,
        Banana,
        Orange,
        Strawberry,
        Lemon,
    }

    impl ToString for Fruit {
        fn to_string(&self) -> String {
            match self {
                Self::Apple => "apple".into(),
                Self::Banana => "banana".into(),
                Self::Orange => "orange".into(),
                Self::Strawberry => "strawberry".into(),
                Self::Lemon => "lemon".into(),
            }
        }
    }
}

impl FromStr for fruits::Fruit {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "apple" => Ok(Self::Apple),
            "banana" => Ok(Self::Banana),
            "orange" => Ok(Self::Orange),
            "strawberry" => Ok(Self::Strawberry),
            "lemon" => Ok(Self::Lemon),
            _ => Err(anyhow::anyhow!("Unsupported fruit type: {}", s)),
        }
    }
}

#[cfg(test)]
speculate::speculate! {
    describe "FruitはString型へ変換が可能" {
        #[rstest(arg, expected,
            case(fruits::Fruit::Apple, "apple"),
            case(fruits::Fruit::Banana, "banana"),
            case(fruits::Fruit::Orange, "orange"),
            case(fruits::Fruit::Strawberry, "strawberry"),
            case(fruits::Fruit::Lemon, "lemon"),
        )]
        fn test_to_string(arg: fruits::Fruit, expected: String) {
            assert_eq!(arg.to_string(), expected);
        }
    }


    describe "strからFruit型への変換が可能" {
        #[rstest(arg, expected,
            case("apple", fruits::Fruit::Apple),
            case("banana", fruits::Fruit::Banana),
            case("orange", fruits::Fruit::Orange),
            case("strawberry", fruits::Fruit::Strawberry),
            case("lemon", fruits::Fruit::Lemon)
        )]
        fn test_from_str(arg: &str, expected: fruits::Fruit) {
            assert_eq!(fruits::Fruit::from_str(arg).unwrap(), expected)
        }
    }

    describe "strからFruits型に変換できない場合もある" {
        #[rstest]
        fn test_from_str_with_error() {
            assert_eq!(fruits::Fruit::from_str("egg").is_err(), true)
        }
    }
}
