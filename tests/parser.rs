extern crate evalmath;
use evalmath::parse;
use evalmath::general::tokens::*;
use Term::{Oprand, Opratr, Bracts, Functs, Constn};
use Operator::*;
use Brackets::*;
use Function::*;
use Constant::*;


#[test]
fn parse_test_case()
{
    let expr1 = parse!("1+3.14");
    let expr2 = parse!("(log(5!))+pi/2^%++");
    let expr3 = parse!("54635.4354325-86554654.547547+765766578657656");
    let expr4 = parse!("ceil floor () []^e Ln      8.0001+7");
    let expr5 = parse!("arcsinh floor ceil phi[]()+0.9+5!!++arccotanh(pi*7)");


    assert_eq!(expr1.unwrap(),   [Oprand(1.0), Opratr(Add), Oprand(3.14)]);

    assert_eq!(expr2.unwrap(),   [Bracts(Oparantes), Functs(Log), Bracts(Oparantes),
                        Oprand(5.0), Functs(Fact), Bracts(Cparantes), 
                        Bracts(Cparantes), Opratr(Add), Constn(Pi), 
                        Opratr(Div), Oprand(2.0), Opratr(Pow), 
                        Opratr(Module), Opratr(Add), Opratr(Add)]);

    assert_eq!(expr3.unwrap(),   [Oprand(54635.4354325), Opratr(Sub), Oprand(86554654.547547),
                        Opratr(Add), Oprand(765766578657656.0)]);

    assert_eq!(expr4.unwrap(),   [Functs(Ceil), Functs(Floor), 
                        Bracts(Oparantes), Bracts(Cparantes), 
                        Bracts(Obracket), Bracts(Cbracket), 
                        Opratr(Pow), Constn(E), Functs(Ln),
                        Oprand(8.0001), Opratr(Add), Oprand(7.0)]);

    assert_eq!(expr5.unwrap(),   [Functs(Arcsinh), Functs(Floor), Functs(Ceil),
                        Constn(Phi), Bracts(Obracket), Bracts(Cbracket), 
                        Bracts(Oparantes), Bracts(Cparantes), Opratr(Add), 
                        Oprand(0.9), Opratr(Add), Oprand(5.0), Functs(Fact), 
                        Functs(Fact), Opratr(Add), Opratr(Add), 
                        Functs(Arccotanh), Bracts(Oparantes), Constn(Pi), 
                        Opratr(Mul), Oprand(7.0), Bracts(Cparantes)]);
                        
}

#[test]
fn parse_unary_test()
{
    let expr1 = parse!("-----9+--3+-2");
    let expr2 = parse!("-(-1+0)--(12-8)-8--21");
    let expr3 = parse!("(7/8*-1)-2*-9");
    let expr4 = parse!("12-(12-1)--(19+20)---(11*2-(11+54+(-9-1))");
    
    
    assert_eq!(expr1.unwrap(),   [Opratr(Neg), Opratr(Neg), Opratr(Neg), 
                        Opratr(Neg), Opratr(Neg), Oprand(9.0), 
                        Opratr(Add), Opratr(Neg), Opratr(Neg), 
                        Oprand(3.0), Opratr(Add), Opratr(Neg), 
                        Oprand(2.0)]);

    assert_eq!(expr2.unwrap(),   [Opratr(Neg), Bracts(Oparantes), Opratr(Neg), 
                        Oprand(1.0), Opratr(Add), Oprand(0.0), 
                        Bracts(Cparantes), Opratr(Sub), Opratr(Neg), 
                        Bracts(Oparantes), Oprand(12.0), Opratr(Sub), 
                        Oprand(8.0), Bracts(Cparantes), Opratr(Sub), 
                        Oprand(8.0), Opratr(Sub), Opratr(Neg), 
                        Oprand(21.0)]);

    assert_eq!(expr3.unwrap(),   [Bracts(Oparantes), Oprand(7.0), Opratr(Div), 
                        Oprand(8.0), Opratr(Mul), Opratr(Neg), 
                        Oprand(1.0), Bracts(Cparantes), Opratr(Sub), 
                        Oprand(2.0), Opratr(Mul), Opratr(Neg), Oprand(9.0)]);

    assert_eq!(expr4.unwrap(),   [Oprand(12.0), Opratr(Sub), Bracts(Oparantes), 
                        Oprand(12.0), Opratr(Sub), Oprand(1.0), 
                        Bracts(Cparantes), Opratr(Sub), Opratr(Neg), 
                        Bracts(Oparantes), Oprand(19.0), Opratr(Add), 
                        Oprand(20.0), Bracts(Cparantes), Opratr(Sub), 
                        Opratr(Neg), Opratr(Neg), Bracts(Oparantes), 
                        Oprand(11.0), Opratr(Mul), Oprand(2.0), 
                        Opratr(Sub), Bracts(Oparantes), Oprand(11.0), 
                        Opratr(Add), Oprand(54.0), Opratr(Add), 
                        Bracts(Oparantes), Opratr(Neg), Oprand(9.0), 
                        Opratr(Sub), Oprand(1.0), Bracts(Cparantes), 
                        Bracts(Cparantes)]);

}
