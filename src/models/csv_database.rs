use serde::{Deserialize,Serialize};
use anyhow::Result;

use serde_yaml;
use std::fs::File;
use std::io::{Read, Write,BufWriter};
use std::fs::OpenOptions;

use csv::{ReaderBuilder, WriterBuilder};
use encoding_rs_io::DecodeReaderBytesBuilder;
use csv::Writer;
use encoding_rs::SHIFT_JIS;


#[derive(Debug, Deserialize,Serialize, Clone)]
pub struct PartRecord {
    #[serde(rename = "メーカー")]
    manufacturer: String,
    #[serde(rename = "型式")]
    model: String,
    #[serde(rename = "名称")]
    name: String,
    #[serde(rename = "備考")]
    note: String,
    #[serde(rename = "代替製品")]
    alternate_model: String,
    #[serde(rename = "参照URL")]
    url: String
}

#[derive(Debug)]
pub struct PartsMaster{
    inner: Vec<PartRecord>,
    path:String
}

impl PartRecord{
    pub fn new(
        manufacturer: String,
        model: String,
        name: String,
        note: String,
        alternate_model: String,
        url: String
    )->Self{
        Self { 
            manufacturer,
            model,
            name,
            note,
            alternate_model,
            url }
    }

}
impl PartsMaster {
    // pub fn load(path:&str)-> Result<Self>{
    //     if !std::path::Path::new(path).exists() {
    //         // ファイルが存在しない場合に作成
    //         File::create(path)?;
    //     }

    //     let file = File::open(path)?;
    //     let transcoded = DecodeReaderBytesBuilder::new()
    //         .encoding(Some(SHIFT_JIS))
    //         .build(file);

    //     let mut rdr = ReaderBuilder::new()
    //         .has_headers(true)
    //         .from_reader(transcoded);

    //     let mut master_file = File::open(path)?;
    //     let mut master_contents = String::new();
    //     master_file.read_to_string(&mut master_contents)?;
        
    //     let parts_master: Vec<PartRecord> = if master_contents.is_empty() {
    //         Vec::new()
    //     } else {
    //         serde_yaml::from_str(&master_contents)?
    //     };

    //     Ok(Self { inner: parts_master,path:path.to_string() })
    // }

    #[allow(dead_code)]
    pub fn create_overwriting(overwriting_data:Vec<PartRecord>,path:&str)->Result<Self>{
        let mut wtr = WriterBuilder::new()
            .has_headers(true)
            .from_writer(vec![]);
        for record in &overwriting_data {
            wtr.serialize(record)?;
        }

        let data = wtr.into_inner()?;
        let binding = String::from_utf8(data).unwrap();
        let (encoded, _, _) = SHIFT_JIS.encode(&binding);
        // 構造体生成時にファイルを生成しているためopen
        let mut output_file = File::create(&path)?;
        output_file.write_all(&encoded)?;
        let path = path.to_string();
        Ok(
            Self { inner: overwriting_data, path }
        )
    }

    pub fn appending(&mut self, append_data:Vec<PartRecord>)-> Result<()>{
        Ok(())
    }

}