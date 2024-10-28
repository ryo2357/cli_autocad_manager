use anyhow::Result;
use glob::glob;
use calamine::{Reader, open_workbook, Xlsx};
use std::fs::File;
use std::io::{BufWriter, Write};
use encoding_rs::SHIFT_JIS;

const INPUT_HEADER:&str = "<記号>,名称,型式,メーカー,備考";
const OUTPUT_HEADER:&str = "図面,<記号>,名称,型式,メーカー,備考";
const SEARCH_PATH_PATTERN:&str = "./**/*.xlsx";
const CREATE_FILE_NAME:&str = "collection_parts_list.csv";


pub fn collect_xlsx_parts_list()->Result<()>{
    let file = File::create(CREATE_FILE_NAME)?;
    let mut bfw = BufWriter::new(file);
    
    // HEADERの記入
    let record= String::new() + OUTPUT_HEADER + "\n";
    let (encoded, _, _) = SHIFT_JIS.encode(&record);
    let _ = bfw.write(&encoded)?;

    for entry in glob(SEARCH_PATH_PATTERN)? {
        match entry {
            Ok(path) => {
                println!("{}",path.display());

                let file_name = path.to_str().unwrap();
                // ファイル名に `~$` が含まれている場合はスキップ
                if file_name.contains("~$") {
                    continue;
                }
            
                let folder_name= path.parent().unwrap().to_str().unwrap();
                
                let mut xl = open_workbook::<Xlsx<_>, _>(&path)?;
                if let Ok(r) = xl.worksheet_range("集計用") {
                    for row in r.rows() {
                        let mut record = Vec::new();
                        for cell in row.iter() {
                            record.push(cell.to_string().replace("_x000D_", ""));
                        }
                        let record = record.join(",");
                        if record == INPUT_HEADER {
                            continue;
                        }
                        let record= String::new() + folder_name + ","+ &record + "\n";
                        let (encoded, _, _) = SHIFT_JIS.encode(&record);
                        let _ = bfw.write(&encoded)?;
                    }
                }
            },
            Err(e) => println!("{:?}",e),
        }
    }
    Ok(())
}