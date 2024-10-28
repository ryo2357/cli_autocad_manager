use serde::{Deserialize,Serialize};
use std::collections::HashMap;
use std::fs::File;
use csv::{ReaderBuilder, WriterBuilder};
use encoding_rs::SHIFT_JIS;
use encoding_rs_io::DecodeReaderBytesBuilder;
use anyhow::Result;
use std::io::Write;
use std::collections::HashSet;

const INPUT_FILE_NAME:&str = "collection_parts_list.csv";
const OUTPUT_FILE_NAME:&str = "aggregate_parts_list.csv";

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct InputRecord {
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

#[derive(Debug, Serialize, Clone)]
struct MediumRecord {
    symbol: String,
    name: String,
    model: String,
    manufacturer: String,
}
#[derive(Debug, Serialize, Clone)]
struct OutputRecord {
    #[serde(rename = "メーカー")]
    manufacturer: String,
    #[serde(rename = "型式")]
    model: String,
    #[serde(rename = "名称")]
    name: String,
    #[serde(rename = "数量")]
    count: u32
}

pub fn aggregate_collection_csv()->Result<()>{
    let medium_vec = get_medium_records()?;
    let output_vec = get_output_records(medium_vec);

    write_output_records_to_csv(output_vec)?;        
    Ok(())
}

fn get_medium_records() -> Result<Vec<MediumRecord>>{
    let file = File::open(INPUT_FILE_NAME)?;
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(SHIFT_JIS))
        .build(file);

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(transcoded);

    let mut vec:Vec<MediumRecord>=Vec::new();
    let mut seen_symbols: HashSet<String> = HashSet::new();
    
    for result in rdr.deserialize() {
        let record: InputRecord = result?;
        if seen_symbols.insert(record.symbol.clone()) {
            vec.push(
                MediumRecord {
                    symbol: record.symbol,
                    name: record.name,
                    model: record.model,
                    manufacturer: record.manufacturer,
                }
            )
        }
    }

    Ok(vec)
}


fn get_output_records(medium_records: Vec<MediumRecord>) -> Vec<OutputRecord> {
    let mut record_map: HashMap<String, OutputRecord> = HashMap::new();

    for record in medium_records {
        if &record.model == "-"{
            continue;
        }
        let model = record.model.clone();

        if let Some(existing_record) = record_map.get_mut(&model) {
            existing_record.count += 1;
        } else {
            record_map.insert(model, OutputRecord {
                name: record.name.clone(),
                model: record.model.clone(),
                manufacturer: record.manufacturer.clone(),
                count: 1,
            });
        }
    }

    record_map.into_values().collect()
}

fn write_output_records_to_csv(records: Vec<OutputRecord>) -> Result<()> {
    let mut wtr = WriterBuilder::new()
        .has_headers(true)
        .from_writer(vec![]);

    for record in &records {
        wtr.serialize(record)?;
    }

    let data = wtr.into_inner()?;
    let binding = String::from_utf8(data).unwrap();
    let (encoded, _, _) = SHIFT_JIS.encode(&binding);
    let mut output_file = File::create(OUTPUT_FILE_NAME)?;
    output_file.write_all(&encoded)?;

    Ok(())
}