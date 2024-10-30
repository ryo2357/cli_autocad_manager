use anyhow::Result;
use std::process::Command;

use crate::models::config::Config;


pub fn show_database_in_excel()->Result<()>{
    let config = Config::read();
    let file_path= config.get_database_path();
    
    println!("{:?}をエクセルで開く",file_path);
    Command::new("cmd")
        .args(["/C", "start", "excel", file_path.to_str().unwrap()])
        .spawn()?;

    Ok(())
}