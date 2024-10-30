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
pub struct PartsDatabase{
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
impl PartsDatabase {
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

        let mut vec:Vec<PartRecord>=Vec::new();
        for result in rdr.deserialize() {
            let record: PartRecord = result?;
            vec.push(record);
        }

        Ok(Self { inner: vec,path:path.to_string() })
    }

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
        let mut output_file = File::create(path)?;
        output_file.write_all(&encoded)?;
        let path = path.to_string();
        Ok(
            Self { inner: overwriting_data, path }
        )
    }

    pub fn create_stage_file_from_records(&self,records:Vec<PartRecord>)-> CommitRecords{
        let mut commit_records:Vec<CommitRecord>=Vec::new();

        for record in records{
            let database_record = self.search_from_model(&record.model);
            if let Some(ref t) = database_record {
                if record.name==t.name 
                && record.manufacturer==t.manufacturer
                // 備考はデータベース優先
                // && record.note ==t.note
                {
                continue
                }
            }
            let commit_record = CommitRecord::new(record, database_record);
            commit_records.push(commit_record);
        }

        CommitRecords::new(commit_records)

    }

    fn search_from_model(&self,model:&str)-> Option<PartRecord>{
        self.inner.iter().find(|&record| record.model == model).cloned()
    }

    pub fn commit_stage(&mut self, stage_data:CommitRecords)-> Result<()>{
        // inner の更新、ファイルへの書き込みは行わない
        for commit_record in stage_data.inner{
            match commit_record.database_record {
                Some(ref commit_database_record )=>{
                    let mut overwriting_record = commit_record.new_record;
                    if overwriting_record.note == String::default(){
                        overwriting_record.note = commit_database_record.note.clone();
                    }
                    if overwriting_record.note == String::default(){
                        overwriting_record.url = commit_database_record.url.clone();
                    }
                    if overwriting_record.alternate_model == String::default(){
                        overwriting_record.alternate_model = commit_database_record.alternate_model.clone();
                    }
                    for part in self.inner.iter_mut() {
                        if part.model == overwriting_record.model {
                            *part = overwriting_record.clone();
                        }
                    }
                },
                None=>{
                    self.inner.push(commit_record.new_record);
                }
            }
        }

        Ok(())
    }
    pub fn save_overwriting(&self)->Result<()>{
        if !std::path::Path::new(&self.path).exists() {
            // ファイルが存在しない場合に作成
            File::create(&self.path)?;
        }
        let mut output_file = File::open(&self.path)?;
        let mut wtr = WriterBuilder::new()
            .has_headers(true)
            .from_writer(vec![]);
        for record in &self.inner {
            wtr.serialize(record)?;
        }

        let data = wtr.into_inner()?;
        let binding = String::from_utf8(data).unwrap();
        let (encoded, _, _) = SHIFT_JIS.encode(&binding);
        // 構造体生成時にファイルを生成しているためopen
        output_file.write_all(&encoded)?;
        Ok(())
    }
}
#[derive(Debug, Deserialize,Serialize)]
pub struct CommitRecord{
    new_record:PartRecord,
    database_record:Option<PartRecord>
}
impl CommitRecord{
    pub fn new(
        new_record:PartRecord,
        database_record:Option<PartRecord>
    )->Self{
        Self{new_record,database_record}
    }
}

#[derive(Debug, Deserialize,Serialize)]
pub struct CommitRecords{
    inner:Vec<CommitRecord>,
}

impl CommitRecords{
    pub fn new(
        inner:Vec<CommitRecord>
    )->Self{
        Self{inner}
    }

    pub fn load(path:&str)->Result<Self>{
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let records: Vec<CommitRecord> = if contents.is_empty() {
            Vec::new()
        } else {
            serde_yaml::from_str(&contents)?
        };
        
        Ok(Self { inner:records})
        
    }
    pub fn save(self,path:&str)->Result<()>{
        let contents = serde_yaml::to_string(&self.inner)?;
        let mut save_file = File::create(path)?;
        save_file.write_all(contents.as_bytes())?;
        Ok(())
    }

    pub fn call(&self){
        println!("alternate_model: {}",self.inner.first().unwrap().new_record.alternate_model)
    }
}