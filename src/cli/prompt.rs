use inquire::Select;

pub fn choose_file(options: Vec<String>) -> Result<String, String> {
    let answer = Select::new("Chooses file", options).prompt();

    if let Ok(x) = answer {
        return Ok(x);
    } else {
        return Err("No answer chosen".to_owned());
    }
}
