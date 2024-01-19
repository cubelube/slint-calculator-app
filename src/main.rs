use slint::SharedString;
use regex::Regex;
use lazy_static::lazy_static;

slint::include_modules!();

lazy_static! {
    static ref FIND_SYMBOL: Regex = Regex::new(r"(\+|-|\*|\/)").unwrap();
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_add_to_text_area(move |existing_string, new_string|{

        let ui = ui_handle.unwrap();
        match new_string.to_string().as_str() {
            "C" => {let result = format!(""); ui.set_textarea(result.into());},
            "=" => {let result = format!("{}", get_calculation(&existing_string).to_string()); ui.set_textarea(result.into());},
            _ => {let result = format!("{}{}", existing_string, new_string); ui.set_textarea(result.into());}
        }
    });

    ui.run()
}

fn get_calculation(input: &SharedString) -> String {
    let symbols = vec!["+", "-", "*", "/"];

    if FIND_SYMBOL.is_match(input) {
        for symbol in symbols {
            match input.as_str().find(symbol) {
                Some(_) => {
                    let n1: f64 = convert(input.to_string().split(symbol).nth(0).expect("FAILED"));
                    let n2: f64 = convert(input.to_string().split(symbol).nth(1).expect("FAILED"));
    
                    match symbol {
                        "+" => return (n1 + n2).to_string(),
                        "-" => return (n1 - n2).to_string(),
                        "*" => return (n1 * n2).to_string(),
                        "/" => return (n1 / n2).to_string(),
                        _ => return String::from("Failed (for some reason)")
                    };
                }
    
                _ => continue
            };
        }
    } else {
        return convert(input).to_string();
    }

    return String::from("Failed (for some reason)")
}

fn convert(input: &str) -> f64 {
    match input.find("%") {
        Some(_) => {let output: f64 = input.split("%").nth(0).expect("FAILED").parse().expect("FAILED"); return output / 100.0},
        _ => {let output: f64 = input.parse().expect("FAILED"); return output}
    };
}
