use std::collections::VecDeque;
extern crate evalmath;
use evalmath::{parse, calculate::*};
use evalmath::general::tokens::*;
use evalmath::calculate::Calculatealgorithm::ShuntingYard;
use Term::{Oprand, Opratr, Bracts, Functs, Constn};
use Operator::*;
use Brackets::*;
use Function::*;
use Constant::*;


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

    // 1+2-2*7^3/4-5
    let expr6: VecDeque<Term> 
    = VecDeque::from(   [Oprand(1.0), Opratr(Add), Oprand(2.0), 
                        Opratr(Sub), Oprand(2.0), Opratr(Mul), 
                        Oprand(7.0), Opratr(Pow), Oprand(3.0), 
                        Opratr(Div), Oprand(4.0), Opratr(Sub), 
                        Oprand(5.0)]);

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

    assert_eq!(infix_to_postfix(&expr6),    [Oprand(1.0), Oprand(2.0), Opratr(Add), 
                                            Oprand(2.0), Oprand(7.0), Oprand(3.0), 
                                            Opratr(Pow), Opratr(Mul), Oprand(4.0), 
                                            Opratr(Div), Opratr(Sub), Oprand(5.0), 
                                            Opratr(Sub)]);

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
    let expr1 = parse!("max(-12,2)");
    let expr2 = parse!("max(1+2, 2)");
    let expr3 = parse!("max(12,2)+2");
    let expr4 = parse!("3+max(12,2)");
    let expr5 = parse!("max(max(4,6,14), avg(3,4,5), min(2,99))");
    let expr6 = parse!("(1+(2+(3+(max(1,2,3,4))))");
    let expr7 = parse!("-max(12,2)");
    let expr8 = parse!("1+max(5!,((((8-0)/(5+5*6))^5+7+(((2-3)*4)/4^(2-0)^2)-4)+2*4)^3,9^9,0)");

    assert_eq!(calculate(expr1, ShuntingYard).unwrap(), 2.0);
    assert_eq!(calculate(expr2, ShuntingYard).unwrap(), 3.0);
    assert_eq!(calculate(expr3, ShuntingYard).unwrap(), 14.0);
    assert_eq!(calculate(expr4, ShuntingYard).unwrap(), 15.0);
    assert_eq!(calculate(expr5, ShuntingYard).unwrap(), 14.0);
    assert_eq!(calculate(expr6, ShuntingYard).unwrap(), 10.0);
    assert_eq!(calculate(expr7, ShuntingYard).unwrap(), -12.0);
    assert_eq!(calculate(expr8, ShuntingYard).unwrap(), 387420490.0);
}

#[test]
fn test_calculate_postfix_math_func()
{
    
}