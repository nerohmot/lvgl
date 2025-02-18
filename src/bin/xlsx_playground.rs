use umya_spreadsheet::reader::xlsx;
use umya_spreadsheet::Spreadsheet;
use umya_spreadsheet::structs::Cell;
use umya_spreadsheet::helper::coordinate::CellCoordinates;
use indicatif::{ProgressBar, ProgressStyle};
use time::Instant;

use std::path::Path;
use polars::prelude::*;
use polars::io::parquet::write as parquet_write;
use std::collections::BTreeMap;
use std::collections::HashSet;

use polars_excel_writer::PolarsXlsxWriter;

fn kbo_nummers(path: &Path) -> Vec<String> {
    // Start timing
    let start = Instant::now();

    // Open the spreadsheet file
    let book: Spreadsheet = xlsx::read(&path).expect("Failed to open the spreadsheet");

    // Get the first sheet
    let sheet = book.get_sheet(&0).expect("Failed to get the first sheet");

    let duration = start.elapsed();
    println!("kbo_nummers initializing takes: {:?}", duration);


    // Find the column with "KBO" in the first header row
    let header_columns : BTreeMap<_, _> = sheet.get_collection_by_row_to_hashmap(&1)
        .into_iter()
        .collect();

    let mut kbo_column = None;
    for (column, cell) in header_columns.iter() {
        let value = cell.get_value().to_lowercase();
        if value.contains("kbo") {
            kbo_column = Some(column.clone());
            break;
        }
    }
    let kbo_column = kbo_column.expect("KBO column not found");

    // Extract data from the "KBO" column, skipping the first two header rows
    let kbo_data: Vec<String> = sheet.get_collection_by_column_to_hashmap(&kbo_column)
        .into_iter()
        .collect::<BTreeMap<_, _>>()
        .iter()
        .skip(2)
        .map(|(_, cell)| cell.get_value().to_string())
        .collect();

    let unique_entries: HashSet<_> = kbo_data.into_iter().collect();
    let result = unique_entries.into_iter().collect();

    // Finish timing
    let duration = start.elapsed();
    println!("Time taken for kbo_nummers: {:?}", duration);

    result
}

fn dmfa_df(path: &Path, progress: bool) -> DataFrame {
    // Start timing
    let start = Instant::now();

    // Open the spreadsheet file
    let book: Spreadsheet = xlsx::read(&path).expect("Failed to open the spreadsheet");

    // Get the first sheet
    let sheet = book.get_sheet(&0).expect("Failed to get the first sheet");

    let duration = start.elapsed();
    println!("dmfa_df initializing takes: {:?}", duration);


    // Find the columns with "INSZ", "WGC", "WNK", "LC", "LC_bedr", and "Kwart" in the first header row
    let header_columns: BTreeMap<_, _> = sheet.get_collection_by_row_to_hashmap(&1)
        .into_iter()
        .collect();
    let mut columns = BTreeMap::new();

    for (column, cell) in header_columns.iter() {
        let value = cell.get_value().to_lowercase();
        if value.contains("insz") {
            columns.insert("INSZ", column.clone());
        } else if value.contains("wgc") {
            columns.insert("WGC", column.clone());
        } else if value.contains("wnk") && !value.contains('_') {
            columns.insert("WNK", column.clone());
        } else if value.contains("lc_bedr") {
            columns.insert("LC_bedr", column.clone());
        } else if value.contains("lc") {
            columns.insert("LC", column.clone());
        } else if value.contains("kwart") {
            columns.insert("Kwart", column.clone());
        }
    }

    let required_columns = ["INSZ", "WGC", "WNK", "LC", "LC_bedr", "Kwart"];
    for &col in &required_columns {
        columns.get(col).expect(&format!("{} column not found", col));
        // println!("{} -> {}", col, columns.get(col).unwrap());
    }

    // println!("columns = {:?}", columns);

    let reference: BTreeMap<_, _> = sheet.get_collection_by_column_to_hashmap(columns.get("LC").unwrap())
        .into_iter()
        .collect();

    // Create a vector of u32 for non-empty cells in the reference
    let reference_rows: Vec<u32> = reference.iter()
        .skip(2)
        .filter_map(|(row, cell)| {
            if !cell.get_value().is_empty() {
                Some(*row)
            } else {
                None
            }
        })
        .collect();

    println!("{} Non-empty LC rows", reference_rows.len());

    let kwart_col = *columns.get("Kwart").unwrap();
    let mut kwart: Vec<String> = Vec::new();
    let rrn_col = *columns.get("INSZ").unwrap();
    let mut rrn: Vec<String> = Vec::new();
    let wgc_col = *columns.get("WGC").unwrap();
    let mut wgc: Vec<u32> = Vec::new(); 
    let wnk_col = *columns.get("WNK").unwrap();
    let mut wnk: Vec<u32> = Vec::new(); 
    let lc_col = *columns.get("LC").unwrap();
    let mut lc: Vec<u32> = Vec::new(); 
    let lc_bedr_col = *columns.get("LC_bedr").unwrap();
    let mut lc_bedr: Vec<f32> = Vec::new();

    // Create and configure the progress bar
    let pb = ProgressBar::new(reference_rows.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );

    for reference_row in reference_rows {
        let kwart_cell = sheet.get_cell(CellCoordinates::from((kwart_col, reference_row))).unwrap();
        let rrn_cell = sheet.get_cell(CellCoordinates::from((rrn_col, reference_row))).unwrap();
        let wgc_cell = sheet.get_cell(CellCoordinates::from((wgc_col, reference_row))).unwrap();
        let wnk_cell = sheet.get_cell(CellCoordinates::from((wnk_col, reference_row))).unwrap();
        let lc_cell = sheet.get_cell(CellCoordinates::from((lc_col, reference_row))).unwrap();
        let lc_bedr_cell = sheet.get_cell(CellCoordinates::from((lc_bedr_col, reference_row))).unwrap();

        kwart.push(kwart_cell.get_value().to_string());
        rrn.push(rrn_cell.get_value().to_string());
        wgc.push(wgc_cell.get_value().parse().unwrap());
        wnk.push(wnk_cell.get_value().parse().unwrap());
        lc.push(lc_cell.get_value().parse().unwrap());
        lc_bedr.push(lc_bedr_cell.get_value().parse().unwrap());

        // Increment the progress bar
        pb.inc(1);
    }

    // Finish the progress bar
    pb.finish_with_message("Processing complete");

    let mut data = Vec::new();
    data.push(Series::new("Kwart".into(), kwart).into());
    data.push(Series::new("INSZ".into(), rrn).into());
    data.push(Series::new("WGC".into(), wgc).into());
    data.push(Series::new("WNK".into(), wnk).into());
    data.push(Series::new("LC".into(), lc).into());
    data.push(Series::new("LC_bedr".into(), lc_bedr).into());

    // Finish timing
    let duration = start.elapsed();
    println!("Time taken for dmfa_df: {:?}", duration);

    // Create a DataFrame from the extracted data
    DataFrame::new(data).expect("Failed to create DataFrame")
}

fn main() {
    let path = Path::new("tests/fixtures/207527540-dmfa.xlsx");
    let kbo_nummers = kbo_nummers(&path);
    println!("{:?}", kbo_nummers);

    let df = dmfa_df(&path, true);
    println!("{:?}", df);

    // Create a new Excel writer.
    let mut xlsx_writer = PolarsXlsxWriter::new();

    // Write the dataframe to Excel.
    xlsx_writer.write_dataframe(&df).expect("Failed to write DataFrame to Excel");

    // Save the file to disk.
    xlsx_writer.save("tests/fixtures/dataframe.xlsx").expect("Failed to save the file");
    

    // // Save the DataFrame to a Parquet file
    // print!("Writing DataFrame to Parquet ... ");
    // let parquet_path = Path::new("output.parquet");
    // df.write_parquet(parquet_path, None).expect("Failed to write DataFrame to Parquet");
    // println!("done");
}


