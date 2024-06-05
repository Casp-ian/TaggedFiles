use inquire::Select;

pub fn choose_file(options: Vec<String>) -> Result<String, ()> {
    let answer = Select::new("Chooses file", options).prompt();

    if let Ok(x) = answer {
        return Ok(x);
    } else {
        return Err(());
    }
}
