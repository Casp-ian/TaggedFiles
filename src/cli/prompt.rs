use inquire::Select;

use crate::tags::types::StoredFile;

pub fn choose_file(options: Vec<StoredFile>) -> Result<StoredFile, String> {
    let answer = Select::<StoredFile>::new("Chooses file", options).prompt();

    if let Ok(x) = answer {
        return Ok(x);
    } else {
        return Err("No answer chosen".to_owned());
    }
}
