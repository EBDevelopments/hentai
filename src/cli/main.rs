use hentai::{Hentai, Result};
use std::env;

fn main() -> Result<()> {
    let mut path = env::current_exe()?;
    path.pop();
    path.push("sample.json");

    if let Ok(result) = Hentai::from_json(path) {
        println!("{:?}", result);
    }

    Ok(())
}