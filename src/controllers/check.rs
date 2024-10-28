use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use csv::ReaderBuilder;
use encoding_rs::SHIFT_JIS;
use encoding_rs_io::DecodeReaderBytesBuilder;
use anyhow::Result;

const FILE_NAME:&str = "collection_parts_list.csv";


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Record {
    #[serde(rename = "図面")]
    drawing_name: String,
    #[serde(rename = "<記号>")]
    symbol: String,
    #[serde(rename = "名称")]
    name: String,
    #[serde(rename = "型式")]
    model: String,
    #[serde(rename = "メーカー")]
    manufacturer: String,
    #[serde(rename = "備考")]
    remarks: String
}

pub fn check_collection_csv()->Result<()>{
    // let mut rdr = ReaderBuilder::new().from_path(FILE_NAME)?;
    let file = File::open(FILE_NAME)?;
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(SHIFT_JIS))
        .build(file);

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(transcoded);

    let mut record_map: HashMap<String, Record> = HashMap::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        if let Some(existing_record) = record_map.get(&record.symbol) {
            if existing_record.model != record.model || existing_record.manufacturer != record.manufacturer  || existing_record.name != record.name{
                println!("重複エラー：図面={}, 記号={}", 
                          record.drawing_name, record.symbol);
            }
        } else {
            record_map.insert(record.symbol.clone(), record);
        }
    }
    println!("重複エラーのチェック完了");
    Ok(())
}