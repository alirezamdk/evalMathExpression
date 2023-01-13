use core::num;
use std::{collections::VecDeque};
use crate::general::{terms::*, mathfunc::math};


trait Calculate 
{
    
}


struct Postfixx;
impl Calculate for Postfixx
{

}


impl dyn Calculate 
{
    pub fn calculate(expr: &VecDeque<Term>, calc_algorithem: CalculateAlgorithem) -> NumsType
    {
        match calc_algorithem
        {
            CalculateAlgorithem::Infix => todo!(),
            CalculateAlgorithem::Postfix => calc_postfix(&infix_to_postfix(expr)),
            CalculateAlgorithem::Tree => todo!(),
            CalculateAlgorithem::W3 => todo!(),
        }
    }
}

pub enum CalculateAlgorithem
{
    Infix,
    Postfix,
    Tree,
    W3,
}

pub fn calculate(expr: &VecDeque<Term>, calc_algorithem: CalculateAlgorithem) -> NumsType
{
    match calc_algorithem
    {
        CalculateAlgorithem::Infix => todo!(),
        CalculateAlgorithem::Postfix => calc_postfix(&infix_to_postfix(expr)),
        CalculateAlgorithem::Tree => todo!(),
        CalculateAlgorithem::W3 => todo!(),
    }
}

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


pub fn postfix_to_infix()
{
    
}

pub fn infix_to_postfix(expr: &VecDeque<Term>) -> VecDeque<Term>
{
    let mut postfix_stack:   VecDeque<Term> = VecDeque::new();
    let mut opr_stack:      VecDeque<Term> = VecDeque::new();
    
    for term in expr
    {
        match term 
        {
            Term::Oprand(num)               => postfix_stack.push_back(Term::Oprand(*num)),
            Term::Constn(constn)       => postfix_stack.push_back(Term::Constn(*constn)),
            Term::Bracts(Brackets::Oparantes)     => opr_stack.push_back(*term),
            Term::Bracts(Brackets::Cparantes)     => 
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
            Term::Opratr(Operator::Neg)           => 
            {
                if opr_stack.is_empty()
                    { opr_stack.push_front(Term::Opratr(Operator::Neg)) }
                else 
                    { opr_stack.push_back(Term::Opratr(Operator::Neg)) }
            }
            _                                      => 
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

pub fn to_perfix(expr: &VecDeque<Term>) -> VecDeque<Term>
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

fn to_tree()
{}

fn calc_tree()
{}

fn to_parantes()
{}

fn calc_infix()
{}

fn calc_w3()
{}

fn calc_perfix(expr: &VecDeque<Term>) -> NumsType
{
    todo!()
}

fn pop_2_back(res_stack: &mut VecDeque<f64>) -> (NumsType, NumsType)
{
    // let  = 0.0;
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
                    Operator::Neg => { let b = res_stack.pop_back().expect("experssion problem"); -b},
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
                        let b = res_stack.pop_back().expect("experssion problem");
                        math::fact(b)
                    }
                    Function::Sqrt => 
                    {
                        let b = res_stack.pop_back().expect("experssion problem");
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
                    Function::Log10 => 
                    {
                        let a = res_stack.pop_back().expect("experssion problem");
                        math::log(10.0, a)
                    }
                    Function::Ln => 
                    {
                        let a = res_stack.pop_back().expect("experssion problem");
                        math::ln(a)
                    }
                    Function::Floor => 
                    {
                        let a = res_stack.pop_back().expect("experssion problem");
                        math::floor(a)
                    }
                    Function::Ceil => 
                    {
                        let a = res_stack.pop_back().expect("experssion problem");
                        math::ceil(a)
                    }
                    Function::Round => 
                    {
                        let a = res_stack.pop_back().expect("experssion problem");
                        math::round(a)
                    }
                    Function::Abs => 
                    {
                        let a = res_stack.pop_back().expect("experssion problem");
                        math::abs(a)
                    }
                    
                    Function::Sin => todo!(),
                    Function::Cos => todo!(),
                    Function::Tan => todo!(),
                    Function::Cotan => todo!(),

                    Function::Arcsin => todo!(),
                    Function::Arccos => todo!(),
                    Function::Arctan => todo!(),
                    Function::Arccotan => todo!(),

                    Function::Sinh => todo!(),
                    Function::Cosh => todo!(),
                    Function::Tanh => todo!(),
                    Function::Cotanh => todo!(),

                    Function::Arcsinh => todo!(),
                    Function::Arccosh => todo!(),
                    Function::Arctanh => todo!(),
                    Function::Arccotanh => todo!(),

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
        println!("term:   {:?}", term);
        println!("stack:  {:?}\n", res_stack);
    }
    *res_stack.front().unwrap()
}

#[cfg(test)]
mod postfix
{
    use std::collections::VecDeque;
    use Term::{Oprand, Opratr, Bracts, Functs, Constn};
    use Operator::*;
    use Brackets::*;
    use Constant::*;
    use Function::*;
    use crate::parse;

    use super::{*, CalculateAlgorithem::*};

    #[test]
    fn test_infix_to_postfix()
    {
        // 1+1
        let expr1: VecDeque<Term> 
        = VecDeque::from(   [Term::Oprand(1.0), Opratr(Add), Oprand(1.0)]);

        // 12-(33+44)
        let expr2: VecDeque<Term> 
        = VecDeque::from(   [Oprand(12.0), Opratr(Sub), Bracts(Oparantes), 
                            Oprand(33.0), Opratr(Add), Oprand(44.0), Bracts(Cparantes)]);

        // 1+2-3
        let expr3: VecDeque<Term> 
        = VecDeque::from(   [Oprand(1.0), Opratr(Add), Oprand(2.0),
                            Opratr(Sub), Oprand(3.0)]);
        // 1/2*3*5
        let expr4: VecDeque<Term> 
        = VecDeque::from(   [Oprand(1.0), Opratr(Div), Oprand(2.0),
                            Opratr(Mul), Oprand(3.0), Opratr(Mul), Oprand(5.0)]);

        // 4+7*9/3+6
        let expr5: VecDeque<Term> 
        = VecDeque::from(   [Oprand(4.0), Opratr(Add), Oprand(7.0), 
                            Opratr(Mul), Oprand(9.0), Opratr(Div),
                            Oprand(3.0), Opratr(Add), Oprand(6.0)]);

        // 1+^2-2*7^3/4-5
        let expr6: VecDeque<Term> 
        = VecDeque::from(   [Oprand(1.0), Opratr(Add), Opratr(Pow), 
                            Oprand(2.0), Opratr(Sub), Oprand(2.0), 
                            Opratr(Mul), Oprand(7.0), Opratr(Pow), 
                            Oprand(3.0), Opratr(Div), Oprand(4.0), 
                            Opratr(Sub), Oprand(5.0)]);

        // ((((8-0)/(5+5*6))^5+7+(((2-3)*4)/4^(2-0)^2)-4)+2*4)^3
        let expr7: VecDeque<Term> 
        = VecDeque::from(   [Bracts(Oparantes), Bracts(Oparantes), Bracts(Oparantes), 
                            Bracts(Oparantes), Oprand(8.0), Opratr(Sub), Oprand(0.0), 
                            Bracts(Cparantes), Opratr(Div), Bracts(Oparantes), 
                            Oprand(5.0), Opratr(Add), Oprand(5.0), Opratr(Mul), 
                            Oprand(6.0), Bracts(Cparantes), Bracts(Cparantes), 
                            Opratr(Pow), Oprand(5.0), Opratr(Add), Oprand(7.0), 
                            Opratr(Add), Bracts(Oparantes), Bracts(Oparantes), 
                            Bracts(Oparantes), Oprand(2.0), Opratr(Sub), Oprand(3.0), 
                            Bracts(Cparantes), Opratr(Mul), Oprand(4.0), 
                            Bracts(Cparantes), Opratr(Div), Oprand(4.0), 
                            Opratr(Pow), Bracts(Oparantes), Oprand(2.0), 
                            Opratr(Sub), Oprand(0.0), Bracts(Cparantes), 
                            Opratr(Pow), Oprand(2.0), Bracts(Cparantes), 
                            Opratr(Sub), Oprand(4.0), Bracts(Cparantes), 
                            Opratr(Add), Oprand(2.0), Opratr(Mul), Oprand(4.0), 
                            Bracts(Cparantes), Opratr(Pow), Oprand(3.0)]);



        assert_eq!(infix_to_postfix(&expr1),  [Oprand(1.0), Oprand(1.0), Opratr(Add)]);

        assert_eq!(infix_to_postfix(&expr2),  [Oprand(12.0), Oprand(33.0), Oprand(44.0), 
                                            Opratr(Add), Opratr(Sub)]);

        assert_eq!(infix_to_postfix(&expr3),  [Oprand(1.0), Oprand(2.0), Opratr(Add),
                                            Oprand(3.0), Opratr(Sub)]);

        assert_eq!(infix_to_postfix(&expr4),  [Oprand(1.0), Oprand(2.0), Opratr(Div),
                                            Oprand(3.0), Opratr(Mul), Oprand(5.0), Opratr(Mul)]);

        assert_eq!(infix_to_postfix(&expr5),  [Oprand(4.0), Oprand(7.0), Oprand(9.0),
                                            Opratr(Mul), Oprand(3.0), Opratr(Div),
                                            Opratr(Add), Oprand(6.0), Opratr(Add)]);

        assert_eq!(infix_to_postfix(&expr6),  [Oprand(1.0), Oprand(2.0), Opratr(Pow), 
                                            Opratr(Add), Oprand(2.0), Oprand(7.0), 
                                            Oprand(3.0), Opratr(Pow), Opratr(Mul), 
                                            Oprand(4.0), Opratr(Div), Opratr(Sub), 
                                            Oprand(5.0), Opratr(Sub)]);

        assert_eq!(infix_to_postfix(&expr7),  [Oprand(8.0), Oprand(0.0), Opratr(Sub), 
                                            Oprand(5.0), Oprand(5.0), Oprand(6.0), 
                                            Opratr(Mul), Opratr(Add), Opratr(Div), 
                                            Oprand(5.0), Opratr(Pow), Oprand(7.0), 
                                            Opratr(Add), Oprand(2.0), Oprand(3.0), 
                                            Opratr(Sub), Oprand(4.0), Opratr(Mul),
                                            Oprand(4.0), Oprand(2.0), Oprand(0.0), 
                                            Opratr(Sub), Opratr(Pow), Oprand(2.0),
                                            Opratr(Pow), Opratr(Div), Opratr(Add), 
                                            Oprand(4.0), Opratr(Sub), Oprand(2.0), 
                                            Oprand(4.0), Opratr(Mul), Opratr(Add), 
                                            Oprand(3.0), Opratr(Pow)]);
    }

    #[test]
    fn test_calculate_postfix()
    {
        // ((((8-0)/(5+5*6))^5+7+(((2-3)*4)/4^(2-0)^2)-4)+2*4)^3
        let expr1 : VecDeque<Term> 
        = VecDeque::from(   [Oprand(8.0), Oprand(0.0), Opratr(Sub), Oprand(5.0), 
                            Oprand(5.0), Oprand(6.0), Opratr(Mul), Opratr(Add), 
                            Opratr(Div), Oprand(5.0), Opratr(Pow), Oprand(7.0), 
                            Opratr(Add), Oprand(2.0), Oprand(3.0), Opratr(Sub), 
                            Oprand(4.0), Opratr(Mul), Oprand(4.0), Oprand(2.0), 
                            Oprand(0.0), Opratr(Sub), Opratr(Pow), Oprand(2.0), 
                            Opratr(Pow), Opratr(Div), Opratr(Add), Oprand(4.0), 
                            Opratr(Sub), Oprand(2.0), Oprand(4.0), Opratr(Mul), 
                            Opratr(Add), Oprand(3.0), Opratr(Pow)]);
        
        let expr2 : VecDeque<Term> 
        = VecDeque::from(   [Oprand(5.0), Functs(Fact), Oprand(5.0), 
                            Oprand(2.0), Opratr(Add), Oprand(3.0), 
                            Opratr(Add), Functs(Sqrt), Opratr(Add), 
                            Constn(Pi), Opratr(Add)]);

        let expr3 : VecDeque<Term> 
        = VecDeque::from(   [Oprand(2.0), Constn(E), Opratr(Pow), 
                            Oprand(0.0), Functs(Fact), Opratr(Pow), 
                            Oprand(5.0), Opratr(Add)]);

        // 7-4+2*4/9+3-3
        let expr4 : VecDeque<Term> 
        = VecDeque::from(   [Oprand(7.0), Oprand(4.0), Opratr(Sub), 
                            Oprand(2.0), Oprand(4.0), Opratr(Mul), 
                            Oprand(9.0), Opratr(Div), Opratr(Add), 
                            Oprand(3.0), Opratr(Add), Oprand(3.0), 
                            Opratr(Sub)]);

        assert_eq!(calc_postfix(&expr1), 1325.5620206713477_f64);
        assert_eq!(calc_postfix(&expr2), 126.30387031375818_f64);
        assert_eq!(calc_postfix(&expr3), 11.58088599101792_f64);
        assert!((calc_postfix(&expr4) - 3.88888 < 0.00001));
    }

    #[test]
    fn test_calculate_postfix_static_func()
    {
        let expr1 = &parse!("max(-12,2)");
        let expr2 = &parse!("max(1+2, 2)");
        let expr3 = &parse!("max(12,2)+2");
        let expr4 = &parse!("3+max(12,2)");
        let expr5 = &parse!("max(max(4,6,14), avg(3,4,5), min(2,99))");
        let expr6 = &parse!("(1+(2+(3+(max(1,2,3,4))))");
        let expr7 = &parse!("-max(12,2)");
        let expr8 = &parse!("1+max(5!,((((8-0)/(5+5*6))^5+7+(((2-3)*4)/4^(2-0)^2)-4)+2*4)^3,9^9,0)");

        assert_eq!(calculate(expr1, Postfix), 2.0);
        assert_eq!(calculate(expr2, Postfix), 3.0);
        assert_eq!(calculate(expr3, Postfix), 14.0);
        assert_eq!(calculate(expr4, Postfix), 15.0);
        assert_eq!(calculate(expr5, Postfix), 14.0);
        assert_eq!(calculate(expr6, Postfix), 10.0);
        assert_eq!(calculate(expr7, Postfix), -12.0);
        assert_eq!(calculate(expr8, Postfix), 387420490.0);
    }

    #[test]
    fn test_calculate_postfix_math_func()
    {
        
    }
}
