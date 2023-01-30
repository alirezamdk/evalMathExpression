use std::{collections::VecDeque, fmt::Error};
use crate::general::{tokens::*, mathfunc::math};


static mut ANGEL_MODE: AngelMode = AngelMode::Radians;

#[derive(Clone, Copy)]
pub enum AngelMode
{
    Degree,     // 0 - 360
    Radians,    // 0 - 2pi - default
    Gradians,   // 0 - 500
}


pub enum Calculatealgorithm
{
    DirectAlgebraic,     // Direct Algebraic Logic
    ShuntingYard,        // Shunting Yard algorithm
    ExpressionTree,      // Expression tree
}


pub fn round_to_decimal(num: NumsType) -> NumsType 
{
    let decimal_places = 14;
    let precision = 10.0_f64.powi(-decimal_places);
    (num * (1.0 / precision)).round() * precision
}


pub fn calculate(expr: Result<VecDeque<Term>, String>, calc_algorithm: Calculatealgorithm) -> Result<NumsType, String>
{
    match expr 
    {
        Ok(expr) => match calc_algorithm
        {
            Calculatealgorithm::DirectAlgebraic => todo!(),
            Calculatealgorithm::ShuntingYard => Ok(calc_postfix(&infix_to_postfix(&expr))),
            Calculatealgorithm::ExpressionTree => todo!(),
        },
        Err(e) => Err(e),
    }
}

// Shunting Yard
fn priority(term: Term) -> i8
{
    match term 
    {
        Term::Bracts(brackets) => 
        {
            match brackets 
            {
                Brackets::Oparantes |
                Brackets::Cparantes => -1,

                Brackets::Obracket  |
                Brackets::Cbracket => 4,

                _ => 0
            }
        },
        Term::Opratr(operator) => 
        {
            match operator 
            {
                Operator::Add |
                Operator::Sub => 1,

                Operator::Div |
                Operator::Mul => 2,

                Operator::Module |
                Operator::Pow => 3,

                Operator::Unknown => unreachable!(),
                Operator::Neg => 4,
            }
        },
        Term::Oprand(_) => unreachable!(),

        Term::Functs(_) => 5,
        Term::Constn(_) => 0,
    }
}

pub fn infix_to_postfix(expr: &VecDeque<Term>) -> VecDeque<Term>
{
    let mut postfix_stack:   VecDeque<Term> = VecDeque::new();
    let mut opr_stack:      VecDeque<Term> = VecDeque::new();
    
    for term in expr
    {
        match term 
        {
            Term::Oprand(num)                   => postfix_stack.push_back(Term::Oprand(*num)),
            Term::Constn(constn)                => postfix_stack.push_back(Term::Constn(*constn)),
            Term::Bracts(Brackets::Oparantes)   => opr_stack.push_back(*term),
            Term::Bracts(Brackets::Cparantes)   => 
            {
                while !(Some(Term::Bracts(Brackets::Oparantes)) == opr_stack.back().copied()
                    || Some(Term::Bracts(Brackets::Obracket)) == opr_stack.back().copied())
                {
                    let operator= opr_stack.pop_back();
                    if operator.is_none()
                        { break }
                    postfix_stack.push_back(operator.unwrap());
                }
                if !opr_stack.is_empty()
                    { opr_stack.pop_back().unwrap(); }
            },
            Term::Opratr(Operator::Neg)          => 
            {
                if opr_stack.is_empty()
                    { opr_stack.push_front(Term::Opratr(Operator::Neg)) }
                else 
                    { opr_stack.push_back(Term::Opratr(Operator::Neg)) }
            }
            _                                    => 
            {
                while !opr_stack.is_empty() 
                    && priority(*term) <= priority(opr_stack.back().copied().unwrap())
                {
                    let operator = opr_stack.pop_back().unwrap();
                    postfix_stack.push_back(operator);
                }
                opr_stack.push_back(*term);
            }
        }
        // println!("Tpos_stack: {:?}", postfix_stack);
        // println!("Zopr_stack: {:?}\n", opr_stack);
    }
    while !opr_stack.is_empty()
    {
        // println!("l_po_stack: {:?}", postfix_stack);
        let operator = opr_stack.pop_back().unwrap();
        postfix_stack.push_back(operator);
    }
    postfix_stack
}

pub fn infix_to_perfix(expr: &VecDeque<Term>) -> VecDeque<Term>
{
    let mut expr = expr.clone();
    let len = expr.len();
    let mut expr_stack_rev : VecDeque<Term> = VecDeque::new();
    for _ in 0..len
    {
        expr_stack_rev.push_front(expr.pop_front().unwrap());
    }
    let mut expr_stack_rev = infix_to_postfix(&expr_stack_rev);

    let mut perfix_stack : VecDeque<Term> = VecDeque::new();
    for _ in 0..len
    {
        if !expr_stack_rev.is_empty()
            { perfix_stack.push_front(expr_stack_rev.pop_front().unwrap()); }
    }
    perfix_stack
}

fn pop_2_back(res_stack: &mut VecDeque<f64>) -> (NumsType, NumsType)
{
    (pop_back(res_stack)
    ,pop_back(res_stack))
}

fn pop_back(res_stack: &mut VecDeque<f64>) -> NumsType
{
    res_stack.pop_back().expect("experssion problem")
}

pub fn calc_postfix(expr: &VecDeque<Term>) -> NumsType
{
    let mut res_stack: VecDeque<NumsType> = VecDeque::new();
    let mut term_num: usize = 0;

    while let Some(term) = expr.get(term_num)
    {
        match term
        {
            Term::Oprand(num) => res_stack.push_back(*num),
            Term::Opratr(operator) => 
            {
                let result = match operator 
                {
                    Operator::Add => { let (b, a) = pop_2_back(&mut res_stack); a + b }
                    Operator::Sub => { let (b, a) = pop_2_back(&mut res_stack); a - b }
                    Operator::Div => { let (b, a) = pop_2_back(&mut res_stack); a / b }
                    Operator::Mul => { let (b, a) = pop_2_back(&mut res_stack); a * b }
                    Operator::Pow => { let (b, a) = pop_2_back(&mut res_stack); math::power(a, b) }
                    Operator::Module => { let (b, a) = pop_2_back(&mut res_stack); a % b }
                    Operator::Neg => { let b = pop_back(&mut res_stack); -b},
                    Operator::Unknown => 0.0,
                };
                
                res_stack.push_back(result);
            },
            Term::Functs(func) => 
            {
                let result = match func
                {
                    Function::Fact => 
                    {
                        let b = pop_back(&mut res_stack);
                        math::fact(b)
                    }
                    Function::Sqrt => 
                    {
                        let b = pop_back(&mut res_stack);
                        math::sqrt(b)
                    }
                    Function::Root => 
                    {
                        let (b, a) = pop_2_back(&mut res_stack);
                        math::root(a, b)
                    }
                    Function::Pnr => 
                    {
                        let (b, a) = pop_2_back(&mut res_stack);
                        math::pnr(a, b)
                    }
                    Function::Cnr => 
                    {
                        let (b, a) = pop_2_back(&mut res_stack);
                        math::cnr(a, b)
                    }
                    Function::Log => 
                    {
                        let (b, a) = pop_2_back(&mut res_stack);
                        math::log(a, b)
                    }
                    Function::Ln => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::ln(a)
                    }
                    Function::Floor => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::floor(a)
                    }
                    Function::Ceil => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::ceil(a)
                    }
                    Function::Round => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::round(a)
                    }
                    Function::Abs => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::abs(a)
                    }
                    
                    Function::Sin => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::sin(a, unsafe { ANGEL_MODE })
                    },
                    Function::Cos => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::cos(a, unsafe { ANGEL_MODE })
                    },
                    Function::Tan => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::tan(a, unsafe { ANGEL_MODE })
                    },
                    Function::Cotan => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::cotan(a, unsafe { ANGEL_MODE })
                    },

                    Function::Arcsin =>
                    {
                        let a = pop_back(&mut res_stack);
                        math::arcsin(a, unsafe { ANGEL_MODE })
                    },
                    Function::Arccos => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::arccos(a, unsafe { ANGEL_MODE })
                    },
                    Function::Arctan => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::arctan(a, unsafe { ANGEL_MODE })
                    },
                    Function::Arccotan => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::arccotan(a, unsafe { ANGEL_MODE })
                    },

                    Function::Sinh => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::sinh(a, unsafe { ANGEL_MODE })
                    },
                    Function::Cosh => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::cosh(a, unsafe { ANGEL_MODE })
                    },
                    Function::Tanh => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::tanh(a, unsafe { ANGEL_MODE })
                    },
                    Function::Cotanh => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::cotanh(a, unsafe { ANGEL_MODE })
                    },
                    Function::Arcsinh => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::arcsinh(a, unsafe { ANGEL_MODE })
                    },
                    Function::Arccosh => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::arccosh(a, unsafe { ANGEL_MODE })
                    },
                    Function::Arctanh => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::arctanh(a, unsafe { ANGEL_MODE })
                    },
                    Function::Arccotanh => 
                    {
                        let a = pop_back(&mut res_stack);
                        math::arccotanh(a, unsafe { ANGEL_MODE })
                    },

                    Function::Max =>
                    {
                        math::max(expr, &mut term_num, &mut res_stack)
                    }
                    Function::Min =>
                    {
                        math::min(expr, &mut term_num, &mut res_stack)
                    }
                    Function::Avg =>
                    {
                        math::avg(expr, &mut term_num, &mut res_stack)
                    }
                    Function::Var => 
                    {
                        math::var(expr, &mut term_num, &mut res_stack)
                    }
                    Function::Dev =>
                    {
                        math::dev(expr, &mut term_num, &mut res_stack)
                    }

                    Function::Bin => todo!(),
                    Function::Oct => todo!(),
                    Function::Dec => todo!(),
                    Function::Hex => todo!(),
                    Function::Tobase => todo!(),
                };
                res_stack.push_back(result);
            },
            Term::Constn(constant) => 
            {   
                let constant = match constant 
                {
                    Constant::Pi => std::f64::consts::PI,
                    Constant::E => std::f64::consts::E,
                    Constant::Phi => 1.61,
                };

                res_stack.push_back(constant);
            },
            Term::Bracts(_) => {},
        }

        term_num += 1;
        // println!("term:   {:?}", term);
        // println!("stack:  {:?}\n", res_stack);
    }
    *res_stack.front().unwrap()
}


// Expression tree
fn infix_to_tree()
{}

fn calc_tree()
{}

// Direct Algebraic Logic
fn infix_add_parantes()
{}

fn calc_infix()
{}

#[macro_export]
macro_rules! calculate
{
    ($expression: expr) => 
    {
        crate::calculate::calculate(
            crate::evalmath::parse::parser::parse(&$expression.to_ascii_lowercase()), 
            $crate::calculate::Calculatealgorithm::ShuntingYard)
    };
    ($expression: expr, $algorithm: ident) => 
    {
        crate::calculate::calculate(
            crate::evalmath::parse::parser::parse(&$expression.to_ascii_lowercase()), 
            $algorithm)
    };
    ($expression: expr, $algorithm: ident, $degmod: ident, $base: expr) => 
    {
        crate::calculate::calculate(
            crate::evalmath::parse::parser::parse(&$expression.to_ascii_lowercase()), 
            $algorithm)
    };
}


