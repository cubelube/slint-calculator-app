use regex::Regex;
use lazy_static::lazy_static;

slint::include_modules!();

// lazy_static only compiles the regex once even if it's used in a loop
lazy_static! {
    static ref CORRECT_EXPRESSION: Regex = Regex::new(r"(\+|-|\*|\/)[0-9]+").unwrap(); // checks for + or - * or /, then at least one 1 digit number
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_add_to_text_area(move |existing_string, new_string|{

        let ui = ui_handle.unwrap();
        match new_string.to_string().as_str() {
            "C" => {
                let result = format!("");
                ui.set_textarea(result.into());
            }, // sets text area to blank
            "F" => {
                let result = format!("{}", factor(existing_string.to_string()).to_string()); 
                ui.set_textarea(result.into());
            }, // factors input
            "=" => {
                let result = format!("{}", get_calculation(existing_string.to_string()).to_string()); 
                ui.set_textarea(result.into());
                //ui.set_historyarea(historyresult.into());
            }, // calculates input
            _ => {
                let result = format!("{}{}", existing_string, new_string); 
                ui.set_textarea(result.into());
            } // adds input to end of text area
        }
    });

    ui.run()
}


fn get_calculation(input: String) -> String {
    let symbols = vec!["+", "-", "*", "/"];

    if CORRECT_EXPRESSION.is_match(&input) { // checks if input is valid expr using regex
        for symbol in symbols {
            match input.as_str().find(symbol) { // runs once it finds symbol in expr
                Some(_) => {
                    let n1: f64 = convert(input.to_string().split(symbol).nth(0).expect("FAILED"));
                    let n2: f64 = convert(input.to_string().split(symbol).nth(1).expect("FAILED"));
    
                    match symbol { // find correct symbol then return corresponding result
                        "+" => return (n1 + n2).to_string(),
                        "-" => return (n1 - n2).to_string(),
                        "*" => return (n1 * n2).to_string(),
                        "/" => return (n1 / n2).to_string(),
                        _ => return String::from("Failed (for some reason)") // the most amazing error handling on earth
                    };
                }
    
                _ => continue
            };
        }
    } else {
        return convert(&input).to_string(); // converts input to float (and converts percents) then return that as a string
    }

    return String::from("Failed (for some reason)") // another example of my wonderful error handling
}

fn convert(input: &str) -> f64 {
    match input.find("%") {
        Some(_) => {
            let output: f64 = input.split("%").nth(0).expect("FAILED").parse().expect("FAILED"); 
            return output / 100.0
        }, // if expr has %, return it as float / 100
        _ => {
            let output: f64 = input.parse().expect("FAILED"); 
            return output
        } // otherwise, return it as float
    };
}

fn factor(input: String) -> String {
    let mut answer = vec![];
    let mut answer_string = String::new();

    if input.find(".").is_some() { // if input is float, can't factor it
        return String::from("Only integers can be factored");
    } else if CORRECT_EXPRESSION.is_match(&input) { // if it's a math expr, calculate the expr then, then factor it, then return that
        answer_string.push_str(&get_calculation(input.clone()));
        answer_string.push_str(" (");
        let factor_result: String = factor(get_calculation(input));
        answer_string.push_str(&factor_result);
        answer_string.push_str(")");

        return answer_string;
    } else { // if it's just an integer, factor it and return that
        let input: i64 = input.parse().expect("FAILED");
        for i in 1..((input/2)+1) { // you have to add 1 so it checks all values 
            if input % i == 0 {
                answer.push(i);
            }
        }

        answer_string.push_str(answer[0].to_string().as_str());

        for num in 1..answer.len() {
            answer_string.push_str(", ");
            answer_string.push_str(answer[num].to_string().as_str());
        }

        return answer_string;
    }
}

