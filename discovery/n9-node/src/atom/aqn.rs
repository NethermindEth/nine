use derive_more::From;
use nom::{
    bytes::complete::{tag, take_while1},
    multi::separated_list1,
    AsChar, IResult, Parser,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::iter;
use std::str::FromStr;
use thiserror::Error;

/// Helper function to parse a valid identifier component
fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanum() || c == '_')(input)
}

/// Function to parse an AQN
fn aqn(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag("."), identifier).parse(input)
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse AQN: {0}")]
    FailedToParse(String),
    #[error("Unexpected input remaining: {0}")]
    Remaining(String),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, From, Ord, Hash, Clone, Deserialize, Serialize)]
pub struct Aqn {
    components: Vec<String>,
}

impl Aqn {
    pub fn from_iter<'a>(components: impl IntoIterator<Item = &'a str>) -> Self {
        let components = components.into_iter().map(String::from).collect();
        Aqn { components }
    }
}

impl FromStr for Aqn {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match aqn(s) {
            Ok((remaining, components)) if remaining.is_empty() => Ok(Aqn::from_iter(components)),
            Ok((remaining, _)) => Err(Error::Remaining(remaining.into())),
            Err(err) => Err(Error::FailedToParse(err.to_string())),
        }
    }
}

impl fmt::Display for Aqn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.components.join("."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aqn_parsing() {
        let input = "app.module.scope.component";
        let expected = Aqn::from_iter(["app", "module", "scope", "component"]);
        let result = Aqn::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_aqn_parsing_with_error() {
        let input = "app.module..scope.component";
        let result = Aqn::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_aqn_display() {
        let aqn = Aqn::from_iter(["app", "module", "scope", "component"]);
        assert_eq!(aqn.to_string(), "app.module.scope.component");
    }
}
