pub type NumsType = f64;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Term
{
    Oprand(NumsType),
    Opratr(Operator),
    Functs(Function),
    Constn(Constant),
    Bracts(Brackets),
    // Varabl(char),
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Brackets
{
    Oparantes,
    Cparantes,
    Obracket,
    Cbracket,
    Oangel,
    Cangel,
    Colon,
}


#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Constant
{
    Pi,     // 3.14
    E,      // 2.7182
    Phi,    // 1.6180  
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Function
{
    Fact,       // fact(n)      factoriel
    Sqrt,       // sqrt(n)      
    Root,       // root(n,m)    Ex. root(3,8) = 2

    Pnr,        // p(n, r)
    Cnr,        // c(n, r)

    Log,        // log[n](m)
    Ln,         // ln(n)

    Floor,      // floor(n)
    Ceil,       // ceil(n)
    Round,      // round(n)

    Abs,        // abs(n)

    Sin,        // sin
    Cos,
    Tan,
    Cotan,

    Arcsin,
    Arccos,
    Arctan,
    Arccotan,

    Sinh,
    Cosh,
    Tanh,
    Cotanh,

    Arcsinh,
    Arccosh,
    Arctanh,
    Arccotanh,

    Max,
    Min,
    Avg,
    Var,
    Dev,

    Bin,
    Oct,
    Dec,
    Hex,
    Tobase,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Operator
{
    Add,        // +
    Sub,        // -
    Div,        // /
    Mul,        // *

    Pow,        // ^
    Module,     // %

    Neg,        // Unary negative

    Unknown,
}

impl Default for Term
{
    fn default() -> Self 
    { 
        Term::Opratr(Operator::Unknown)
    }
}