use regex::Regex;

fn main() {

    //regex and   // (\d+) \s?\+ \s? (\d+) Regular expressions
    let re_multiply = Regex::new(r"(\d+) \s*\*\s* (\d+)").unwrap();
    let re_divide = Regex::new(r"(\d+) \s* / \s* (\d+)").unwrap();
    let re_add = Regex::new(r"(\d+) \s*\+\s* (\d+)").unwrap();
    let re_rest = Regex::new(r"(\d+) \s*\-\s* (\d+)").unwrap();

    
    //Get user data
    println!("Please introduce your expression");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();


    //multiply
    loop {
        let caps = re_multiply.captures(expression.as_str());

        if caps.is_none() {
            break;
      }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let _left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let _right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    
        let multiply = _left_value * _right_value;

        expression = expression.replace(cap_expression, &multiply.to_string());
    }

    /*
    Division
    */
    loop {
        let caps = re_divide.captures(expression.as_str());

        if caps.is_none() {
            break;
      }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let _left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let _right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    
        let division = _left_value / _right_value;

        expression = expression.replace(cap_expression, &division.to_string());
    }

    // addition
    loop {
        let caps = re_add.captures(expression.as_str());

        if caps.is_none() {
            break;
      }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let _left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let _right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    
        let addition = _left_value + _right_value;

        expression = expression.replace(cap_expression, &addition.to_string());
    }

    // rest
    loop {
        let caps = re_rest.captures(expression.as_str());

        if caps.is_none() {
            break;
      }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let _left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let _right_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
    
        let rest = _left_value - _right_value;

        expression = expression.replace(cap_expression, &rest.to_string());
    }

    //show results
    println!("result: {}", expression);

}
