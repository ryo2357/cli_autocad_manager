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

use crate::models::csv_database::PartRecord;



#[derive(Debug, Serialize,Deserialize, Clone)]
pub struct AggregateRecord {
    #[serde(rename = "メーカー")]
    manufacturer: String,
    #[serde(rename = "型式")]
    model: String,
    #[serde(rename = "名称")]
    name: String,
    #[serde(rename = "数量")]
    count: u32,
    #[serde(rename = "備考")]
    note: String
}

#[derive(Debug)]
pub struct AggregateRecords{
    inner : Vec<AggregateRecord>,
    path:String
}

impl AggregateRecords {
    pub fn load(path:&str)-> Result<Self>{
        if !std::path::Path::new(path).exists() {
            // ファイルが存在しない場合に作成
            File::create(path)?;
        }

        let file = File::open(path)?;
        let transcoded = DecodeReaderBytesBuilder::new()
            .encoding(Some(SHIFT_JIS))
            .build(file);

        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(transcoded);

        let mut vec:Vec<AggregateRecord>=Vec::new();
        for result in rdr.deserialize() {
            let record: AggregateRecord = result?;
            vec.push(record);
        }

        Ok(Self { inner: vec,path:path.to_string() })
    }
    pub fn convert(self)->Vec<PartRecord>{
        let mut vec:Vec<PartRecord> =Vec::new();
        for record in self.inner{
            let convert_record = PartRecord::new(
                record.manufacturer,
                record.model,
                record.name,
                record.note,
                String::default(),
                String::default(),
            );
            vec.push(convert_record);
        }
        vec
    }
}

