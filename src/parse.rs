use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{all_consuming, into},
    multi::many0,
    sequence::pair,
    IResult,
};

use crate::ast::{Body, BodyPart, Command, Newline};

fn parse_newline(input: &str) -> IResult<&str, Newline> {
    let (input, _) = tag("\n")(input)?;
    Ok((input, Newline {}))
}

fn parse_space(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag(" ")(input)?;
    Ok((input, ()))
}

fn parse_argument(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("argument")(input)?;
    Ok((input, "argument".to_owned()))
}

fn parse_space_prefixed_argument(input: &str) -> IResult<&str, String> {
    let (input, (_, argument)) = pair(parse_space, parse_argument)(input)?;
    Ok((input, argument))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("(")(input)?;
    let (input, name) = tag("hardcoded_command")(input)?;
    let (input, arguments) = many0(parse_space_prefixed_argument)(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((
        input,
        Command {
            name: name.to_owned(),
            arguments,
        },
    ))
}

fn parse_body_part(input: &str) -> IResult<&str, BodyPart> {
    let (input, body_part) = alt((into(parse_newline), into(parse_command)))(input)?;
    Ok((input, body_part))
}

fn parse_body(input: &str) -> IResult<&str, Body> {
    let (input, parts) = many0(parse_body_part)(input)?;
    Ok((input, Body { parts }))
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

pub fn parse_body_complete(input: &str) -> Result<Body, ParseError> {
    match all_consuming(parse_body)(input) {
        Ok((_, body)) => Ok(body),
        Err(s) => {
            eprintln!("Invalid body!");
            Err(ParseError {
                message: s.to_string(),
            }) // TODO: convert parse error
        }
    }
}
