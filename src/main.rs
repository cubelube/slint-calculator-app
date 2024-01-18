use slint::SharedString;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_add_to_text_area(move |string1, string2|{

        let ui = ui_handle.unwrap();
        match string2.to_string().as_str() {
            "C" => {let result = format!(""); ui.set_textarea(result.into());},
            "=" => {let result = format!("{}", get_calculation(&string1).to_string()); ui.set_textarea(result.into());},
            _ => {let result = format!("{}{}", string1, string2); ui.set_textarea(result.into());}
        }
    });

    ui.run()
}

fn get_calculation(input: &SharedString) -> f64 {
    let symbols = vec!["+", "-", "*", "/"];

    for symbol in symbols {
        match input.as_str().find(symbol) {
            Some(_) => {
                let n1: f64 = input.split(symbol).nth(0).expect("FAILED").parse().expect("FAILED");
                let n2: f64 = input.split(symbol).nth(1).expect("FAILED").parse().expect("FAILED");

                match symbol {
                    "+" => return n1 + n2,
                    "-" => return n1 - n2,
                    "*" => return n1 * n2,
                    "/" => return n1 / n2,
                    _ => return 1.0
                };
            }

            _ => continue
        };
    }

    return 1.0;
}
