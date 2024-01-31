use pest::{Parser, iterators::Pair};
use pest_derive::*;

use crate::{
  error::*,
  Operator,
  PointAward
};

#[derive(Parser)]
#[grammar = "parser.pest"]
pub struct MessageParser;

impl MessageParser {
  pub fn parse_message(message: &str) -> Result<Vec<PointAward>, ParseError> {
    let parsed = MessageParser::parse(Rule::message, message)?;

    let mut awards = Vec::new();
    for message in parsed {
      if let Rule::message = message.as_rule() {
        for pair in message.into_inner() {
          if let Rule::point_award = pair.as_rule() {
            awards.push(parse_point_award(pair)?);
          }
        }
      }
    }

    Ok(awards)
  }
}

/// Used to parse a point award from a pest pair.
fn parse_point_award(pair: Pair<Rule>) -> Result<PointAward, ParseError> {
  let mut award = PointAward::default();
  for award_pair in pair.into_inner() {
    match award_pair.as_rule() {
      Rule::mention  => { award.target = parse_mention(award_pair)?; },
      Rule::points   => { award.points = parse_points(award_pair)?; },
      Rule::reason   => { award.reason = parse_reason(award_pair)?; },
      Rule::operator => { award.operator = parse_operator(award_pair)?; },
      _ => {}
    }
  }
  Ok(award)
}

/// Used to parse a mention from a pest pair.
fn parse_mention(pair: Pair<Rule>) -> Result<String, ParseError> {
  match pair.into_inner().next() {
    Some(mention) => Ok(mention.as_str().to_string()),
    None => Err(ParseError::InvalidMention)
  }
}

/// Used to parse points from a pest pair.
fn parse_points(pair: Pair<Rule>) -> Result<i32, ParseError> {
  pair.as_str().trim().parse::<i32>().map_err(|_| ParseError::InvalidPointValue)
}

/// Used to parse a reason from a pest pair.
/// We also try to strip any leading/trailing whitespace if the reason exists.
fn parse_reason(pair: Pair<Rule>) -> Result<Option<String>, ParseError> {
  let reason = pair.as_str().trim().to_string();
  Ok(if reason.is_empty() { None } else { Some(reason) })
}

fn parse_operator(pair: Pair<Rule>) -> Result<Operator, ParseError> {
  match pair.as_str().trim() {
    "++" => Ok(Operator::Add),
    "--" => Ok(Operator::Subtract),
    _ => Err(ParseError::InvalidOperator)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse_message_simple_add() {
    let message = "<@U01T73NJVFV> ++ 10";
    let awards = MessageParser::parse_message(message).unwrap();
    assert_eq!(awards.len(), 1);
    assert_eq!(awards[0].target, "@U01T73NJVFV");
    assert_eq!(awards[0].operator, Operator::Add);
    assert_eq!(awards[0].points, 10);
    assert_eq!(awards[0].reason, None);
  }

  #[test]
  fn test_parse_message_simple_subtract() {
    let message = "<@U01T73NJVFV> -- 5 because I can, muhahaha";
    let awards = MessageParser::parse_message(message).unwrap();
    assert_eq!(awards.len(), 1);
    assert_eq!(awards[0].target, "@U01T73NJVFV");
    assert_eq!(awards[0].operator, Operator::Subtract);
    assert_eq!(awards[0].points, 5);
    assert_eq!(awards[0].reason, Some("because I can, muhahaha".to_string()));
  }

  #[test]
  fn test_parse_message_simple_no_points() {
    let message = "This is a standard message that mentions a user <@U01T73NJVFV> but doesn't grant points, thus should be disregarded";
    let awards = MessageParser::parse_message(message).unwrap();
    assert_eq!(awards.len(), 0);
  }

  #[test]
  fn test_parse_message_reason() {
    let message = "<@U01T73NJVFV> ++ 420 I like turtles";
    let awards = MessageParser::parse_message(message).unwrap();
    assert_eq!(awards.len(), 1);
    assert_eq!(awards[0].target, "@U01T73NJVFV");
    assert_eq!(awards[0].points, 420);
    assert_eq!(awards[0].reason, Some("I like turtles".to_string()));
  }

  #[test]
  fn test_parse_message_multiple() {
    let message = "<@U01T73NJVFV> ++ for doing awesome stuff <@U26G29CORQD> ++ 420";
    let awards = MessageParser::parse_message(message).unwrap();
    assert_eq!(awards.len(), 2);

    // Check the first award
    assert_eq!(awards[0].target, "@U01T73NJVFV");
    assert_eq!(awards[0].points, 1);
    assert_eq!(awards[0].reason, Some("for doing awesome stuff".to_string()));

    // Check the second award
    assert_eq!(awards[1].target, "@U26G29CORQD");
    assert_eq!(awards[1].points, 420);
    assert_eq!(awards[1].reason, None);
  }

  #[test]
  fn test_parse_message_long() {
    let message = "I'd like to introduce our new team member <@U01T73NJVFV> ++ 100 they are going to start working on the new incredible project! <@U01T73NJVFV> is the best person evar!";
    let awards = MessageParser::parse_message(message).unwrap();
    assert_eq!(awards.len(), 1);
    assert_eq!(awards[0].target, "@U01T73NJVFV");
    assert_eq!(awards[0].points, 100);
    assert_eq!(awards[0].reason, Some("they are going to start working on the new incredible project".to_string()));
  }
}
