
pub mod math
{
    use std::{collections::VecDeque};
    use crate::general::tokens::*;

    pub fn power(a: NumsType, b: NumsType) -> NumsType
    {
        NumsType::powf(a, b)
    }

    pub fn fact(a: NumsType) -> NumsType
    {
        if a == 0.0 || a == 1.0 { return 1.0 }
        fact( a - 1.0 ) * a
    }

    pub fn sqrt(a: NumsType) -> NumsType
    {
        NumsType::sqrt(a)
    }

    pub fn root(a: NumsType, b: NumsType) -> NumsType
    {
        NumsType::powf(b, 1.0 / a)
    }

    pub fn pnr(a: NumsType, b: NumsType) -> NumsType
    {
        fact(a) / fact(a - b)
    }

    pub fn cnr(a: NumsType, b: NumsType) -> NumsType
    {
        fact(a) / (fact(b)*fact(a - b))
    }

    pub fn log(a: NumsType, b: NumsType) -> NumsType
    {
        NumsType::log(a, b)
    }

    pub fn ln(a: NumsType) -> NumsType
    {
        NumsType::ln(a)
    }

    pub fn floor(a: NumsType) -> NumsType
    {
        NumsType::floor(a)
    }

    pub fn ceil(a: NumsType) -> NumsType
    {
        NumsType::ceil(a)
    }

    pub fn round(a: NumsType) -> NumsType
    {
        NumsType::round(a)
    }

    pub fn abs(a: NumsType) -> NumsType
    {
        NumsType::abs(a)
    }

    pub fn max(expr: &VecDeque<Term>, term_num: &mut usize, res_stack: &mut  VecDeque<NumsType>) -> NumsType
    {
        let nums = get_nums(expr, term_num, res_stack);
        nums.into_iter()
            .reduce(NumsType::max)
            .unwrap()
    }

    pub fn min(expr: &VecDeque<Term>, term_num: &mut usize, res_stack: &mut  VecDeque<NumsType>) -> NumsType
    {
        let nums = get_nums(expr, term_num, res_stack);
        nums.into_iter()
            .reduce(NumsType::min)
            .unwrap()
    }

    

    pub fn avg(expr: &VecDeque<Term>, term_num: &mut usize, res_stack: &mut  VecDeque<NumsType>) -> NumsType
    {
        let nums = get_nums(expr, term_num, res_stack);
        _avg(&nums)
    }


    // Standard deviation
    pub fn dev(expr: &VecDeque<Term>, term_num: &mut usize, res_stack: &mut  VecDeque<NumsType>) -> NumsType
    {
        let nums = get_nums(expr, term_num, res_stack);
        _dev(nums)
    }

    // Variance
    pub fn var(expr: &VecDeque<Term>, term_num: &mut usize, res_stack: &mut  VecDeque<NumsType>)-> NumsType
    {
        let nums = get_nums(expr, term_num, res_stack);
        sqrt(_dev(nums))
    }

    fn _avg(nums: &Vec<NumsType>) -> NumsType
    {
        let len = nums.len() as NumsType;
        nums.iter()
            .sum::<NumsType>() / len 
    }
    
    fn _dev(nums: Vec<NumsType>) -> NumsType
    {
        let len = nums.len() as NumsType;
        let avg = _avg(&nums);
        let mut sum: NumsType = 0.0;
        
        for a in nums.iter()
        {
            sum += power(a - avg, 2.0);
        }

        sum / len
    }


    fn count_op(expr: &VecDeque<Term>, term_num: &mut usize) -> usize
    {
        let mut count_op: usize = 0;
        // skip max function
        *term_num += 1;

        while let Some(term) = expr.get(*term_num)
        {
            println!("term in count_op: {:?}", term);
            match term 
            {
                Term::Oprand(_) |
                Term::Constn(_) |
                Term::Opratr(Operator::Neg) => break,
                
                Term::Functs(_) |
                Term::Bracts(Brackets::Colon) |
                Term::Opratr(_) => count_op += 1,
                
                Term::Bracts(_) => break,
            }
            *term_num += 1
        }

        *term_num -= count_op + 1;
        count_op
    }

    fn get_nums(expr: &VecDeque<Term>, term_num: &mut usize, res_stack: &mut  VecDeque<NumsType>) -> Vec<NumsType>
    {
        let mut nums: Vec<NumsType> = Vec::new();

        let count_op: usize = count_op(expr, term_num);
        
        println!("numer: {}", count_op);

        let res_stack_len = res_stack.len();
        if res_stack_len > 0
        {
            for _ in count_op..res_stack_len
            {
                let num = res_stack.pop_back().expect("experssion problem");
                nums.push(num);
            }
        }
        nums
    }
}