//! Module for reading Hugging Face dataset Parquet files and reconstructing Rust AST.

use std::path::Path;
use anyhow::{Result, Context};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use arrow::record_batch::RecordBatch;
use arrow::array::*;
use arrow::datatypes::DataType;
use std::fs::File;
use std::collections::HashMap;

// Placeholder for AST reconstruction logic
#[allow(dead_code)]
pub async fn reconstruct_ast_from_hf_dataset(dataset_dir: &Path) -> Result<()> {
    println!("Attempting to reconstruct AST from dataset at: {:?}", dataset_dir);

    let parsing_phase_path = dataset_dir.join("semantic").join("parsing-phase").join("data.parquet");

    if !parsing_phase_path.exists() {
        anyhow::bail!("Parsing phase Parquet file not found at: {:?}", parsing_phase_path);
    }

    println!("Reading parsing phase data from: {:?}", parsing_phase_path);

    let file = File::open(&parsing_phase_path)
        .context(format!("Failed to open Parquet file: {:?}", parsing_phase_path))?;

    let builder = ParquetRecordBatchReaderBuilder::try_new(file)
        .context("Failed to create Parquet reader builder")?;

    let mut reader = builder.build()
        .context("Failed to build Parquet reader")?;

    // Read all record batches
    let mut all_records: Vec<HashMap<String, String>> = Vec::new();
    while let Some(batch_result) = reader.next() {
        let batch = batch_result.context("Failed to read record batch")?;
        let records = extract_records_from_batch(&batch)?;
        all_records.extend(records);
    }

    println!("Successfully read {} records from parsing phase.", all_records.len());

    // TODO: Implement AST reconstruction logic here using `all_records`
    // This will involve iterating through the records and building `syn` AST nodes.

    Ok(())
}

#[allow(dead_code)]
fn extract_records_from_batch(batch: &RecordBatch) -> Result<Vec<HashMap<String, String>>> {
    let schema = batch.schema();
    let num_rows = batch.num_rows();
    let mut records = Vec::new();

    for row_idx in 0..num_rows {
        let mut record = HashMap::new();
        for (col_idx, field) in schema.fields().iter().enumerate() {
            let column = batch.column(col_idx);
            let value = extract_value_at_index(column, row_idx)?;
            record.insert(field.name().clone(), value);
        }
        records.push(record);
    }

    Ok(records)
}

#[allow(dead_code)]
fn extract_value_at_index(array: &dyn Array, index: usize) -> Result<String> {
    if array.is_null(index) {
        return Ok("null".to_string());
    }

    match array.data_type() {
        DataType::Utf8 => {
            let string_array = array.as_any().downcast_ref::<StringArray>()
                .context("Failed to downcast to StringArray")?;
            Ok(string_array.value(index).to_string())
        }
        DataType::UInt32 => {
            let uint32_array = array.as_any().downcast_ref::<UInt32Array>()
                .context("Failed to downcast to UInt32Array")?;
            Ok(uint32_array.value(index).to_string())
        }
        DataType::Int64 => {
            let int64_array = array.as_any().downcast_ref::<Int64Array>()
                .context("Failed to downcast to Int64Array")?;
            Ok(int64_array.value(index).to_string())
        }
        DataType::Boolean => {
            let bool_array = array.as_any().downcast_ref::<BooleanArray>()
                .context("Failed to downcast to BooleanArray")?;
            Ok(bool_array.value(index).to_string())
        }
        DataType::List(_) => {
            Ok("[list]".to_string()) // Simplified for lists
        }
        _ => {
            Ok(format!("[{}]", array.data_type()))
        }
    }
}
