#[macro_use]
extern crate nom;

use nom::error::VerboseError;
use nom::{
    branch::{alt, switch},
    bytes::{tag, take_while},
    character::complete::alphanumeric1 as alnum,
    combinator::map,
    multi::separated_list,
    sequence::{delimited, pair, preceeded, separated_pair, terminated},
    Err, IResult,
};

use std::collections::HashMap;

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
    RecordField(String, Value),
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
    And(Matcher, Matcher),
    Or(Matcher, Matcher),
    Not(Matcher),
    Comparison(Value, Value, Comparator),
}

fn whitespace(i: &[u8]) -> IResult<&[u8], &[u8]> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

fn parse_type_instantiation(input: &[u8]) -> IResult<&[u8], Type> {
    map(
        separated_pair(
            preceeded(whitespace, alnum),
            preceeded(whitespace, "of"),
            separated_list(delimited(whitespace, tag("and"), whitespace), alnum),
        ),
        |name, typs| Type {
            name: name,
            contents: TypeType::Declaration,
            generics: Result::Err(typs),
        },
    )
}

fn parse_enum(input: &[u8]) -> IResult<&[u8], TypeType> {
    map(
        separated_list(preceeded(whitespace, alt(tag("or a"), tag("or an"))), alnum),
        |v| TypeType::EnumDecl(v),
    )
}

fn parse_record(input: &[u8]) -> IResult<&[u8], TypeType> {
    map(
        separated_list(
            preceeded(whitespace, alt(tag("and a"), tag("and an"))),
            separated_pair(
                delimited(whitespace, parse_type_instantiation, whitespace),
                tag("called"),
                preceeded(whitespace, alnum),
            ),
        ),
        |v| {
            TypeType::RecordDecl(
                v.into_iter()
                    .map(|(typ, name)| (name, typ))
                    .collect::<HashMap<String, Type>>(),
            )
        },
    )
}

fn parse_type_decl(input: &[u8]) -> IResult<&[u8], Type> {
    map(
        pair(
            preceeded(whitespace, parse_type_instantiation),
            switch!(alt(tag("is a"), tag("has a"), tag("is an"), tag("has an")),
            "is a" => call!(parse_enum),
            "is an" => call!(parse_enum),
            "has a" => call!(parse_record),
            "has an" => call!(parse_record),
            ),
        ),
        |(typ, recs)| Type {
            name: typ.name,
            contents: recs,
            // TODO: figure out how to reconcile the generics we know in recs with the generics we
            // forward-declare in the type.
            generics: typ.generics,
        },
    )
}
