use std::{fs, io::Result, path::Path};

const PATH: &str = "todo.json";

pub fn load() -> Result<Vec<super::Task>> {
    if !Path::new(PATH).exists() {
        fs::write(PATH, "[]")?;
    }

    let text = fs::read_to_string(PATH)?;
    Ok(serde_json::from_str(&text)?)
}

pub fn save(todos: &[super::Task]) -> Result<()> {
    let text = serde_json::to_string_pretty(todos)?;
    fs::write(PATH, text)
}