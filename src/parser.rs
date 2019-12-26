use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::alphanumeric1 as alnum,
    combinator::map,
    multi::separated_list,
    sequence::{delimited, pair, preceded, separated_pair},
    IResult,
};

use std::collections::HashMap;
use std::rc::Rc;

pub enum TypeType {
    // user-declared type
    RecordDecl(HashMap<String, Type>),
    EnumDecl(Vec<String>),
    // built-ins
    Number,
    // This will have to have generics, but that comes later
    Function,
    Declaration,
}

pub struct Type {
    name: String,
    contents: TypeType,
    // if the generics are being bound, the latter
    // type is the only known part
    generics: Result<HashMap<String, Type>, Vec<String>>,
}

pub enum Value {
    Reference(Vec<String>, Type),
    Literal(Type),
}

pub enum Comparator {
    GT,
    LT,
    EQ,
    GTE,
    LTE,
    NEQ,
}

pub enum Matcher {
    And(Vec<Matcher>),
    Or(Vec<Matcher>),
    Not(Rc<Matcher>),
    Comparison(Value, Value, Comparator),
}

pub enum ProvisionBodyStmt {
    IfStmt(Matcher, Rc<ProvisionBodyStmt>, Rc<ProvisionBodyStmt>),
    ProvideStmt(Value),
}

pub struct Provision {
    name: String,
    args: HashMap<String, Type>,
    body: ProvisionBodyStmt,
    returns: Type,
}

fn whitespace(i: &str) -> IResult<&str, &str> {
    let spaces = " \t\r\n";
    take_while1(move |c| spaces.contains(c))(i)
}

fn parse_type_instantiation(input: &str) -> IResult<&str, Type> {
    map(
        separated_pair(
            preceded(whitespace, alnum),
            preceded(whitespace, tag("of")),
            separated_list(delimited(whitespace, tag("and"), whitespace), alnum),
        ),
        |(name, typs)| Type {
            name: String::from(name),
            contents: TypeType::Declaration,
            generics: Result::Err(typs.into_iter().map(String::from).collect()),
        },
    )(input)
}

fn parse_enum(input: &str) -> IResult<&str, TypeType> {
    map(
        separated_list(
            preceded(whitespace, alt((tag("or a"), tag("or an")))),
            alnum,
        ),
        |v| TypeType::EnumDecl(v.into_iter().map(String::from).collect()),
    )(input)
}

fn parse_record(input: &str) -> IResult<&str, TypeType> {
    map(
        separated_list(
            preceded(whitespace, alt((tag("and a"), tag("and an")))),
            separated_pair(
                delimited(whitespace, parse_type_instantiation, whitespace),
                tag("called"),
                preceded(whitespace, alnum),
            ),
        ),
        |v| {
            TypeType::RecordDecl(
                v.into_iter()
                    .map(|(typ, name)| (String::from(name), typ))
                    .collect::<HashMap<String, Type>>(),
            )
        },
    )(input)
}

fn parse_type_decl(input: &str) -> IResult<&str, Type> {
    map(
        pair(
            preceded(whitespace, parse_type_instantiation),
            alt((
                preceded(tag("is a"), parse_enum),
                preceded(tag("is an"), parse_enum),
                preceded(tag("has a"), parse_record),
                preceded(tag("has an"), parse_record),
            )),
        ),
        |(typ, recs)| Type {
            name: typ.name,
            contents: recs,
            // TODO: figure out how to reconcile the generics we know in recs with the generics we
            // forward-declare in the type.
            generics: typ.generics,
        },
    )(input)
}

fn parse_matcher(input: &str) -> IResult<&str, Matcher> {}

fn parse_provision(input: &str) -> IResult<&str, Provision> {
    preceded(
        alt((tag("Given a"), tag("Given an"))),
        separated_list(
            delimited(whitespace, tag("and"), whitespace),
            parse_type_instantiation,
        ),
    )
}
