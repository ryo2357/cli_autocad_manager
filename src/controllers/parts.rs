use anyhow::Result;

use crate::controllers::{
    collect::collect_xlsx_parts_list,
    check::check_not_unusual_collection_csv,
    aggregate::aggregate_collection_csv,
};


pub fn aggregate_parts_table_from_xlsx()->Result<()>{
    println!("部品表の集計処理の開始");

    println!("--------------------");
    println!("xlsxの集計処理開始");
    match collect_xlsx_parts_list() {
        Ok(()) =>{
            println!("xlsxの集計処理完了")
        },
        Err(r) =>{
            anyhow::bail!("collectにてエラー: {}",r);
        }        
    }

    println!("--------------------");
    println!("収集したxlsxの内容確認");
    match check_not_unusual_collection_csv() {
        Ok(true) =>{
            println!("内容確認完了")
        },
        Ok(false) =>{
            anyhow::bail!("集計データに不具合あり")
        },
        Err(r) =>{
            anyhow::bail!("checkにてエラー: {}",r);
        }        
    }

    println!("--------------------");
    println!("集計csvの生成");
    match aggregate_collection_csv() {
        Ok(()) =>{
            println!("集計csvの生成完了")
        },
        Err(r) =>{
            anyhow::bail!("aggregateにてエラー: {}",r);
        }        
    }
    println!("--------------------");
    println!("パーツの集計処理の完了");


    Ok(())
}