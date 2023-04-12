#[path = "module_abc.rs"]
mod module_hello;
fn func_ex_div_some(x: i32, y: i32) -> Option<i32> {
    let ans = if y == 0 { None } else { Some(x / y) };
    ans
}

fn func_ex_div_result(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("div by zero")
    } else {
        Ok(x / y)
    }
}

fn func_ex_print_some<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(x) = ans {
        println!("{}", x)
    } else {
        println!("None")
    }
}

fn func_ex_print_some_match<T: std::fmt::Display>(ans: Option<T>) {
    match ans {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}

fn func_ex_print_result<T: std::fmt::Display, E: std::fmt::Display>(ans: Result<T, E>) {
    match ans {
        Ok(res) => println!("{}", res),
        Err(str) => println!("{}", str),
    }
}

fn main() {
    func_ex_print_some(func_ex_div_some(10, 5));
    func_ex_print_some(func_ex_div_some(10, 0));
    func_ex_print_some_match(func_ex_div_some(10, 5));
    func_ex_print_some_match(func_ex_div_some(10, 0));
    func_ex_print_result(func_ex_div_result(10, 5));
    func_ex_print_result(func_ex_div_result(10, 0));
    module_hello::print_hello();
}

#[test]
fn test_ex_div_some() {
    assert_eq!(func_ex_div_some(10, 5), Some(2));
    assert_eq!(func_ex_div_some(10, 0), None);
}

#[test]
fn test_ex_div_result() {
    assert_eq!(func_ex_div_result(10, 5), Ok(2));
    assert_eq!(func_ex_div_result(10, 0), Err("div by zero"));
}

#[test]
fn test_ex_print_some() {
    func_ex_print_some(func_ex_div_some(10, 5));
    func_ex_print_some(func_ex_div_some(10, 0));
}

#[test]
fn test_ex_print_some_match() {
    func_ex_print_some_match(func_ex_div_some(10, 5));
    func_ex_print_some_match(func_ex_div_some(10, 0));
}

#[test]
fn test_ex_print_result() {
    func_ex_print_result(func_ex_div_result(10, 5));
    func_ex_print_result(func_ex_div_result(10, 0));
}
