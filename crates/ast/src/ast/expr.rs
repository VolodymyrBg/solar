use super::{Lit, SubDenomination, Ty};
use std::fmt;
use sulk_interface::{Ident, Span};

/// A list of named arguments: `{a: "1", b: 2}`.
pub type NamedArgList = Vec<NamedArg>;

/// An expression.
///
/// Reference: <https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.expression>
#[derive(Clone, Debug)]
pub struct Expr {
    pub span: Span,
    pub kind: ExprKind,
}

impl Expr {
    /// Creates a new expression from an identifier.
    pub fn from_ident(ident: Ident) -> Self {
        Self { span: ident.span, kind: ExprKind::Ident(ident) }
    }

    /// Creates a new expression from a type.
    pub fn from_ty(ty: Ty) -> Self {
        Self { span: ty.span, kind: ExprKind::Type(ty) }
    }
}

/// A kind of expression.
#[derive(Clone, Debug)]
pub enum ExprKind {
    /// An array literal expression: `[a, b, c, d]`.
    Array(Vec<Box<Expr>>),

    /// An assignment: `a = b`, `a += b`.
    Assign(Box<Expr>, Option<BinOp>, Box<Expr>),

    /// A binary operation: `a + b`, `a >> b`.
    Binary(Box<Expr>, BinOp, Box<Expr>),

    /// A function call expression: `foo(42)` or `foo({ bar: 42 })`.
    Call(Box<Expr>, CallArgs),

    /// Function call options: `foo.bar{ value: 1, gas: 2 }`.
    CallOptions(Box<Expr>, NamedArgList),

    /// A unary `delete` expression: `delete vector`.
    Delete(Box<Expr>),

    /// An identifier: `foo`.
    Ident(Ident),

    /// A square bracketed indexing expression: `vector[index]`, `slice[l:r]`.
    Index(Box<Expr>, IndexKind),

    /// A literal: `hex"1234"`, `5.6 ether`.
    Lit(Lit, Option<SubDenomination>),

    /// Access of a named member: `obj.k`.
    Member(Box<Expr>, Ident),

    /// A `new` expression: `new Contract`.
    New(Ty),

    /// A `payable` expression: `payable(address(0x...))`.
    Payable(CallArgs),

    /// A ternary (AKA conditional) expression: `foo ? bar : baz`.
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),

    /// A tuple expression: `(a,,, b, c, d)`.
    Tuple(Vec<Option<Box<Expr>>>),

    /// A `type()` expression: `type(uint256)`
    TypeCall(Ty),

    /// An elementary type name: `uint256`.
    Type(Ty),

    /// A unary operation: `!x`, `-x`, `x++`.
    Unary(UnOp, Box<Expr>),
}

/// A binary operation: `a + b`, `a += b`.
#[derive(Clone, Debug)]
pub struct BinOp {
    pub span: Span,
    pub kind: BinOpKind,
}

/// A kind of binary operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BinOpKind {
    /// `<`
    Lt,
    /// `<=`
    Le,
    /// `>`
    Gt,
    /// `>=`
    Ge,
    /// `==`
    Eq,
    /// `!=`
    Ne,
    /// `||`
    Or,
    /// `&&`
    And,

    /// `>>`
    Shr,
    /// `<<`
    Shl,
    /// `>>>`
    Sar,
    /// `&`
    BitAnd,
    /// `|`
    BitOr,
    /// `^`
    BitXor,

    /// `+`
    Add,
    /// `-`
    Sub,
    /// `**`
    Pow,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `%`
    Rem,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.kind.to_str())
    }
}

impl BinOpKind {
    /// Returns the string representation of the operator.
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
            Self::Eq => "==",
            Self::Ne => "!=",
            Self::Or => "||",
            Self::And => "&&",
            Self::Sar => ">>>",
            Self::Shr => ">>",
            Self::Shl => "<<",
            Self::BitAnd => "&",
            Self::BitOr => "|",
            Self::BitXor => "^",
            Self::Add => "+",
            Self::Sub => "-",
            Self::Pow => "**",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Rem => "%",
        }
    }

    /// Returns `true` if the operator is able to be used in an assignment.
    pub const fn assignable(self) -> bool {
        // https://docs.soliditylang.org/en/latest/grammar.html#a4.SolidityParser.expression
        match self {
            Self::BitOr
            | Self::BitXor
            | Self::BitAnd
            | Self::Shl
            | Self::Shr
            | Self::Sar
            | Self::Add
            | Self::Sub
            | Self::Mul
            | Self::Div
            | Self::Rem => true,

            Self::Lt
            | Self::Le
            | Self::Gt
            | Self::Ge
            | Self::Eq
            | Self::Ne
            | Self::Or
            | Self::And
            | Self::Pow => false,
        }
    }
}

/// A unary operation: `!x`, `-x`, `x++`.
#[derive(Clone, Debug)]
pub struct UnOp {
    pub span: Span,
    pub kind: UnOpKind,
}

/// A kind of unary operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnOpKind {
    /// `++x`
    PreInc,
    /// `--x`
    PreDec,
    /// `!`
    Not,
    /// `-`
    Neg,
    /// `~`
    BitNot,

    /// `x++`
    PostInc,
    /// `x--`
    PostDec,
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.kind.to_str())
    }
}

impl UnOpKind {
    /// Returns the string representation of the operator.
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::PreInc => "++",
            Self::PreDec => "--",
            Self::Not => "!",
            Self::Neg => "-",
            Self::BitNot => "~",
            Self::PostInc => "++",
            Self::PostDec => "--",
        }
    }

    /// Returns `true` if the operator is a prefix operator.
    pub const fn is_prefix(self) -> bool {
        match self {
            Self::PreInc | Self::PreDec | Self::Not | Self::Neg | Self::BitNot => true,
            Self::PostInc | Self::PostDec => false,
        }
    }

    /// Returns `true` if the operator is a postfix operator.
    pub const fn is_postfix(self) -> bool {
        !self.is_prefix()
    }
}

/// A list of function call arguments.
#[derive(Clone, Debug)]
pub enum CallArgs {
    /// A list of unnamed arguments: `(1, 2, 3)`.
    Unnamed(Vec<Box<Expr>>),

    /// A list of named arguments: `({x: 1, y: 2, z: 3})`.
    Named(NamedArgList),
}

impl Default for CallArgs {
    #[inline]
    fn default() -> Self {
        Self::Unnamed(Vec::new())
    }
}

impl CallArgs {
    /// Creates a new empty list of unnamed arguments.
    #[inline]
    pub fn empty() -> Self {
        Self::Unnamed(Vec::new())
    }
}

/// A named argument: `name: value`.
#[derive(Clone, Debug)]
pub struct NamedArg {
    pub name: Ident,
    pub value: Box<Expr>,
}

/// A kind of square bracketed indexing expression: `vector[index]`, `slice[l:r]`.
#[derive(Clone, Debug)]
pub enum IndexKind {
    /// A single index: `vector[index]`.
    Index(Option<Box<Expr>>),

    /// A slice: `slice[l:r]`.
    Range(Option<Box<Expr>>, Option<Box<Expr>>),
}
