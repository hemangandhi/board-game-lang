#[macro_use]
extern crate nom;

pub struct TypeName {
    name: [u8],
    generic_args: Vec<[u8]>,
}

pub enum FieldReference {
    Recursive([u8], FieldReference),
    Base([u8]),
}

pub enum ValueReference {
    Field(FieldReference),
    Value([u8]),
}

pub enum MatcherStmt {
    And(MatcherStmt, MatcherStmt),
    Or(MatcherStmt, MatcherStmt),
    Not(MatcherStmt),
    AtLeast(ValueReference, ValueReference),
    AtMost(ValueReference, ValueReference),
}

pub enum BodyStmt {
    IfStmt(MatcherStmt, BodyStmt, BodyStmt),
    ProvisionStmt(BodyStmt),
    InstantiationStmt(TypeName, Vec<(FieldReference, ValueReference)>),
}

pub enum BoardGameStatement {
    RecordDecl(Vec<([u8], TypeName)>),
    EnumDecl(Vec<[u8]>),
    ProviderDecl([u8], Vec<([u8], TypeName)>, BodyStmt),
}

pub fn parse_game(in_stream: [u8]) -> BoardGameSpec {}
