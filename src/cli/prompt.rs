use inquire::Select;

use crate::tags::db::File;

pub fn choose_file(options: Vec<File>) -> Result<File, String> {
    let answer = Select::<File>::new("Chooses file", options).prompt();

    if let Ok(x) = answer {
        return Ok(x);
    } else {
        return Err("No answer chosen".to_owned());
    }
}
