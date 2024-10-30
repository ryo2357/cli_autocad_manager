use anyhow::Result;

use crate::models::csv_database::PartsDatabase;
use crate::models::aggregate_csv::AggregateRecords;
use crate::models::config::Config;

pub fn create_stage_file(input_file_path:&str,output_path:&str)->Result<()>{
    let config = Config::read();
    let file_path= config.get_database_path();
    let database= PartsDatabase::load(file_path.to_str().unwrap())?;

    let aggregate_records  = AggregateRecords::load(input_file_path)?;
    let new_records = aggregate_records.convert();

    let stage_records= database.create_stage_file_from_records(new_records);
    stage_records.save(output_path)?;


    Ok(())
}


// 検証用コード
// pub fn verify_save()->Result<()>{
//     let commit_records = sample_records();
//     commit_records.call();
//     let _ = commit_records.save("test.yaml");
//     Ok(())
// }

// pub fn verify_load()->Result<()>{
//     let commit_records = CommitRecords::load("test.yaml")?;
//     commit_records.call();
//     println!("load file:{:?}",commit_records);
//     Ok(())
// }

// fn sample_records()->CommitRecords{
//     let mut records:Vec<CommitRecord>=Vec::new();
//     let record= CommitRecord::new(sample_record("ddddd"),None);
//     records.push(record);
//     let record= CommitRecord::new(sample_record("ccccc"),Some(sample_record("ffff")));
//     records.push(record);
//     CommitRecords::new(records)
// }

// fn sample_record(model:&str)-> PartRecord{
//     PartRecord::new(
//         "ミスミ".to_string(),
//             model.to_string(),
//             "スイッチ".to_string(),
            
//             "".to_string(),
//             String::default(),
//             "".to_string(),)
// }