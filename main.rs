use slint::SharedString;
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_add_to_text_area(move |string1, string2|{
        let ui = ui_handle.unwrap();
        if string2 == "C" {
            let result = format!("");
            ui.set_textarea(result.into());
        } else if string2 == "=" {
            
            let result = calculate(&string1).to_string();
            let result2 = format!("{}", result);
            ui.set_textarea(result2.into());
        } else {
            let result = format!("{}{}", string1, string2);
            ui.set_textarea(result.into());
        }
    });

    ui.run()
}

fn calculate(input: &SharedString) -> String {

    if input.find("+").is_some() {
        let number1: f32 = input.as_str().split("+").nth(0).expect("FAILED").parse().expect("FAILED");
        let number2: f32 = input.as_str().split("+").nth(1).expect("FAILED").parse().expect("FAILED");

        return (number1 + number2).to_string();
    } else if input.find("-").is_some() {
        let number1: f32 = input.as_str().split("-").nth(0).expect("FAILED").parse().expect("FAILED");
        let number2: f32 = input.as_str().split("-").nth(1).expect("FAILED").parse().expect("FAILED");

        return (number1 - number2).to_string();
    } else if input.find("*").is_some() {
        let number1: f32 = input.as_str().split("*").nth(0).expect("FAILED").parse().expect("FAILED");
        let number2: f32 = input.as_str().split("*").nth(1).expect("FAILED").parse().expect("FAILED");

        return (number1 * number2).to_string();
    } else if input.find("/").is_some() {
        let number1: f32 = input.as_str().split("/").nth(0).expect("FAILED").parse().expect("FAILED");
        let number2: f32 = input.as_str().split("/").nth(1).expect("FAILED").parse().expect("FAILED");

        return (number1 / number2).to_string();
    } else {
        return String::from("Failed (for some reason)");
    }
}
