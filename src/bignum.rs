use std::ops::{Add, Sub, Mul, Div};
use std::fmt;

// it will be complete and use in library. it meybe change 
#[derive(Clone, PartialEq)]
pub struct BigNum
{
    digits: Vec<u32>,
    negative: bool,
    decimal_point: usize
}

impl BigNum 
{
    pub fn new(n: &str) -> Self
    {
        let mut num = n.to_string();
        let mut negative = false;
        if num.starts_with("-")
        {
            negative = true;
            num.remove(0);
        }
        let mut digits = vec![];
        let mut decimal_point = num.len();
        for ch in num.chars()
        {
            if ch == '.'
            {
                decimal_point = digits.len();
            } else 
            {
                digits.push(ch.to_digit(10).unwrap());
            }
        }
        Self { digits, negative, decimal_point }
    }

    // Other methods for operations like add, substract, multiply and divide
    pub fn addition(&self, other: &BigNum) -> BigNum 
    {
        todo!()
    }

    fn subtract(&self, other: &BigNum) -> BigNum {
        todo!()
    }

}

impl fmt::Display for BigNum 
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        let mut res = String::new();
        if self.negative 
        {
            res.push('-');
        }
        for i in 0..self.digits.len() 
        {
            if i == self.decimal_point 
            {
                res.push('.');
            }
            res.push_str(&self.digits[i].to_string());
        }
        write!(f, "{}", res)
    }
}


impl Add for BigNum 
{
    type Output = Self;

    fn add(self, other: BigNum) -> Self
    {
        self.addition(&other)
    }
}

impl Sub for BigNum 
{
    type Output = Self;

    fn sub(self, other: Self) -> Self 
    {
        self.subtract(&other)
    }
}

// impl Mul for BigNum 
// {
//     type Output = BigNum;

//     fn mul(self, other: BigNum) -> BigNum 
//     {
//         self.multiply(&other)
//     }
// }


// impl Div for BigNum 
// {
//     type Output = BigNum;

//     fn div(self, other: BigNum) -> BigNum 
//     {
//         self.divide(&other)
//     }
// }



