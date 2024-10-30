use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;
// use std::io::{Read, Write,BufWriter};

use std::io::Read;
// use std::fs::OpenOptions;
use anyhow::Result;
// use csv::Writer;
// use encoding_rs::SHIFT_JIS;

// use super::add_parts::AddParts;
use crate::models::csv_database::PartRecord;

#[derive(Debug, Serialize, Deserialize)]
pub struct PartData {
    id: u32, // Add an optional id field
    name: String,
    model: String,
    manufacturer: String,
    tag: Option<String>,
    is_discontinued: bool,
    update_date: String,
    estimated_price: Option<f32>,
    url: Option<String>,
    remarks: Option<String>,
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct PartsMaster{
    inner: Vec<PartData>,
    path:String
}

impl PartData{
    #[allow(clippy::too_many_arguments,dead_code)]
    pub fn new(
        id: u32,
        name: String,
        model: String,
        manufacturer: String,
        tag: Option<String>,
        is_discontinued: bool,
        update_date: String,
        estimated_price: Option<f32>,
        url: Option<String>,
        remarks: Option<String>,
    )->Self{
        Self { id, name, model, manufacturer, tag, is_discontinued, update_date, estimated_price, url, remarks }
    }

    // pub fn csv_header() -> Vec<String>{
    //     vec![
    //         "id".to_string(),
    //         "name".to_string(),
    //         "model".to_string(),
    //         "manufacturer".to_string(),
    //         "tag".to_string(),
    //         "is_discontinued".to_string(),
    //         "update_date".to_string(),
    //         "estimated_price".to_string(),
    //         "url".to_string(),
    //         "remarks".to_string()
    //         ]
    // }
    // pub fn to_csv_record(&self) -> Vec<String> {
    //     vec![
    //         self.id.to_string(),
    //         self.name.clone(),
    //         self.model.clone(),
    //         self.manufacturer.clone(),
    //         self.tag.clone().unwrap_or_default(),
    //         self.is_discontinued.to_string(),
    //         self.update_date.clone(),
    //         self.estimated_price.unwrap_or_default().to_string(),
    //         self.url.clone().unwrap_or_default(),
    //         self.remarks.clone().unwrap_or_default(),
    //     ]
    // }
}

impl PartsMaster {
    pub fn load(path:&str)-> Result<Self>{
        if !std::path::Path::new(path).exists() {
            // ファイルが存在しない場合に作成
            File::create(path)?;
        }
        let mut master_file = File::open(path)?;
        let mut master_contents = String::new();
        master_file.read_to_string(&mut master_contents)?;
        
        let parts_master: Vec<PartData> = if master_contents.is_empty() {
            Vec::new()
        } else {
            serde_yaml::from_str(&master_contents)?
        };

        Ok(Self { inner: parts_master,path:path.to_string() })
    }

    pub fn create_csv_records(self)-> Result<Vec<PartRecord>>{
        let mut vec:Vec<PartRecord> =Vec::new();

        for data in self.inner{
            let manufacturer= data.manufacturer;
            let model = data.model;
            let name = data.name;
            let note = data.remarks.unwrap_or_default();
            let alternate_model = match data.is_discontinued {
                 true => "入力しろ".to_string(),
                 false => String::default()
            };
            let url  = data.url.unwrap_or_default();

            let record = PartRecord::new(manufacturer, model, name, note, alternate_model, url);
            vec.push(record);
        }

        Ok(vec)
    }

    // #[allow(dead_code)]
    // pub fn get_max_uuid(&self)->u32{
    //     let max_id = self.inner.iter().max_by_key(|part| part.id).map(|part| part.id);
    //     max_id.unwrap_or(0)
    // }

    // pub fn get_vec_model_and_max_uuid(&self)->(Vec<String>,u32){
    //     let mut max_id = 0;
    //     let vec_model:Vec<String> = self.inner.iter().map(|part| {
    //         if max_id < part.id { max_id = part.id }
    //         part.model.clone()
    //     }).collect();
    //     (vec_model,max_id)
    // }


    // #[allow(dead_code)]
    // pub fn write(self)->Result<()>{
    //     let updated_master_contents = serde_yaml::to_string(&self.inner)?;
    //     let mut master_file = File::open(&self.path)?;
    //     master_file.write_all(updated_master_contents.as_bytes())?;
    //     Ok(())
    // }

    // pub fn convert_to_csv_utf8(self,csv_path:&str)-> Result<()>{
    //     let mut wtr = Writer::from_path(csv_path)?;
    //     // wtr.write_record(["id", "name", "model", "manufacturer", "tag", "IsDiscontinued", "updateDate", "estimatedPrice", "url", "remarks"])?;
    //     wtr.write_record(PartData::csv_header())?;
    //     for item in self.inner {
    //         wtr.write_record(item.to_csv_record())?;
    //     }

    //     Ok(())
    // }

    // pub fn convert_to_csv_jis(self,csv_path:&str)-> Result<()>{
    //     let file = File::create(csv_path)?;
    //     let mut writer = BufWriter::new(file);

    //     let header = PartData::csv_header().join(",") + "\n";
    //     let (encoded_header, _, _) = SHIFT_JIS.encode(&header);
    //     writer.write_all(&encoded_header)?;

    //     for item in self.inner {
    //         let record = item.to_csv_record().join(",") + "\n";
    //         let (encoded_record, _, _) = SHIFT_JIS.encode(&record);
    //         writer.write_all(&encoded_record)?;
    //     }

    //     Ok(())
    // }
    // pub fn find_by_model(&self, model: &str) -> Option<&PartData> {
    //     self.inner.iter().find(|&part| part.model == model)
    // }
}

