use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::alphanumeric1 as alnum,
    combinator::map,
    multi::separated_list,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
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
    // Type is optional so that it can be left empty in parsing.
    Reference(Vec<Value>, Option<Type>),
    // Since the parsing is ambiguous, a Reference(x, Option::None)
    // may later be upgraded to an Invocation.
    Invocation(String, Vec<String>, Type),
    // Similar to the above, the Type is optional until we can solve for it.
    Literal(String, Option<Type>),
}

pub enum Comparator {
    GT,
    LT,
    EQ,
    GTE,
    LTE,
    NEQ,
}

// Note: the parser actually doesn't produce this type in full generality:
// just at most, an and containing ors containing nots containing comparisons.
// This is to avoid parenthesis in the programming language and sticking to a
// clear order of operations. Provisions (functions) can be used if an or absolutely
// must contain an and.
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
            separated_list(delimited(whitespace, tag("of"), whitespace), alnum),
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
            preceded(whitespace, alt((tag("with a"), tag("with an")))),
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
            // forward-declare in the type. (How to bind them here?)
            generics: typ.generics,
        },
    )(input)
}

// The Invocation grammar is actually ambiguous. Hence, it is after parsing that the value reference is
// converted into an invocation or a reference.
fn parse_value(input: &str) -> IResult<&str, Value> {
    alt((
        map(
            preceded(
                tag("the"),
                separated_list(
                    delimited(whitespace, alt((tag("then"), tag("of"))), whitespace),
                    parse_value,
                ),
            ),
            |vec_of_an| Value::Reference(vec_of_an, Option::None),
        ),
        map(delimited(whitespace, alnum, whitespace), |lit| {
            Value::Literal(lit.to_string(), Option::None)
        }),
    ))(input)
}

fn parse_matcher(input: &str) -> IResult<&str, Matcher> {
    fn base_matcher(input: &str) -> IResult<&str, Matcher> {
        map(
            tuple((
                delimited(whitespace, parse_value, whitespace),
                alt((
                    map(tag("is at least"), |_| Comparator::GTE),
                    map(tag("is bigger than"), |_| Comparator::GT),
                    map(tag("is at most"), |_| Comparator::LTE),
                    map(tag("is smaller than"), |_| Comparator::LT),
                    map(tag("is not"), |_| Comparator::NEQ),
                    map(tag("is"), |_| Comparator::EQ),
                )),
                delimited(whitespace, parse_value, whitespace),
            )),
            |(lhs, cmp, rhs)| Matcher::Comparison(lhs, rhs, cmp),
        )(input)
    }

    fn matcher_just_not(input: &str) -> IResult<&str, Matcher> {
        map(
            preceded(delimited(whitespace, tag("not"), whitespace), base_matcher),
            |bm| Matcher::Not(Rc::new(bm)),
        )(input)
    }
    fn matcher_no_and(input: &str) -> IResult<&str, Matcher> {
        map(
            separated_list(
                delimited(whitespace, tag("or"), whitespace),
                matcher_just_not,
            ),
            |ve| Matcher::Or(ve),
        )(input)
    }
    map(
        separated_list(
            delimited(whitespace, tag("and"), whitespace),
            matcher_no_and,
        ),
        |ve| Matcher::And(ve),
    )(input)
}

fn parse_provision(input: &str) -> IResult<&str, Provision> {
    fn get_args(input: &str) -> IResult<&str, HashMap<String, Type>> {
        map(
            preceded(
                alt((tag("Given a"), tag("Given an"))),
                separated_list(
                    delimited(whitespace, tag("then"), whitespace),
                    separated_pair(
                        alnum,
                        alt((tag("called a"), tag("called an"))),
                        parse_type_instantiation,
                    ),
                ),
            ),
            |v| v.into_iter().map(|(nm, typ)| (nm.to_string(), typ)).collect(),
        )(input)
    }

    fn parse_before_body(input: &str) -> IResult<&str, (HashMap<String, Type>, &str, Type)> {
        tuple((
            get_args,
            preceded(alt((tag("resolve a"), tag("resolve an"))), alnum),
            preceded(
                delimited(
                    whitespace,
                    alt((tag("to get a"), tag("to get an"))),
                    whitespace,
                ),
                parse_type_instantiation,
            ),
        ))(input)
    }

    fn parse_provision_body(input: &str) -> IResult<&str, ProvisionBodyStmt> {
        alt((
            map(
                preceded(
                    delimited(whitespace, tag("if"), whitespace),
                    separated_pair(
                        parse_matcher,
                        delimited(whitespace, tag("then"), whitespace),
                        separated_pair(
                            parse_provision_body,
                            delimited(whitespace, tag("else"), whitespace),
                            parse_provision_body,
                        ),
                    ),
                ),
                |(m, (t, e))| ProvisionBodyStmt::IfStmt(m, Rc::new(t), Rc::new(e)),
            ),
            map(
                preceded(
                    delimited(whitespace, tag("providing"), whitespace),
                    parse_value,
                ),
                |v| ProvisionBodyStmt::ProvideStmt(v),
            ),
        ))(input)
    }

    map(
        separated_pair(
            parse_before_body,
            delimited(whitespace, tag("by"), whitespace),
            parse_provision_body,
        ),
        |((args, name, ret), body)| Provision {
            name: name.to_string(),
            args: args,
            body: body,
            returns: ret,
        },
    )(input)
}
