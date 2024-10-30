use anyhow::Result;

use crate::models::yaml_parts_master;
use crate::models::csv_database::PartsMaster;

pub fn convert_yaml_to_csv(parts_master_path:&str,csv_path:&str)->Result<()>{
    let load_data = yaml_parts_master::PartsMaster::load(parts_master_path)?;
    let data = load_data.create_csv_records()?;
    let _save_data= PartsMaster::create_overwriting(data, csv_path)?;

    Ok(())
}