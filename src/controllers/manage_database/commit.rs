use anyhow::Result;

use crate::models::csv_database::{PartsDatabase,CommitRecords};
use crate::models::config::Config;

pub fn commit_stage_file_to_database(input_file_path:&str)->Result<()>{
    let config = Config::read();
    let file_path= config.get_database_path();
    let mut database= PartsDatabase::load(file_path.to_str().unwrap())?;

    let commit_records  = CommitRecords::load(input_file_path)?;

    database.commit_stage(commit_records)?;
    database.save_overwriting()?;

    Ok(())
}


pub fn verify_overwriting()->Result<()>{
    // csvのカラムは構造体の順番
    // 入れ替えても問題なくロードできる
    // セーブしたら入れ替えた順番になる
    let config = Config::read();
    let file_path= config.get_database_path();
    let database= PartsDatabase::load(file_path.to_str().unwrap())?;

    database.save_overwriting()?;

    Ok(())
}