use crate::eval::{executor, types::Expr};

pub fn process(id: String, args: Vec<Box<Expr>>) -> f64 {
    let mut nums = Vec::new();
    for arg in args.iter() {
        nums.push(executor::calculate((**arg).clone()))
    }

    match id.as_str() {
        "sqrt" => nums[0].sqrt(),
        "ln" => nums[0].ln(),
        "root" => nums[0].powf(1.0 / nums[1]),
        "log" => nums[1].log(nums[0]),
        "cbrt" => nums[0].powf(1.0 / 3.0),

        "sin" => nums[0].sin(),
        "cos" => nums[0].cos(),
        "tan" => nums[0].tan(),
        "cot" => 1.0 / nums[0].sin(),
        "sec" => 1.0 / nums[0].cos(),
        "csc" => 1.0 / nums[0].tan(),

        "asin" => nums[0].asin(),
        "acos" => nums[0].acos(),
        "atan" => nums[0].atan(),
        "acot" => 1.0 / nums[0].atan(),
        "asec" => 1.0 / nums[0].acos(),
        "acsc" => 1.0 / nums[0].asin(),

        "sinh" => nums[0].sinh(),
        "cosh" => nums[0].cosh(),
        "tanh" => nums[0].tanh(),
        "coth" => 1.0 / nums[0].tanh(),
        "sech" => 1.0 / nums[0].cosh(),
        "csch" => 1.0 / nums[0].sinh(),

        "asinh" => nums[0].asinh(),
        "acosh" => nums[0].acosh(),
        "atanh" => nums[0].atanh(),
        "acoth" => 1.0 / nums[0].atanh(),
        "asech" => 1.0 / nums[0].acosh(),
        "acsch" => 1.0 / nums[0].asinh(),
        _ => unreachable!(),
    }
}
