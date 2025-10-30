use anyhow::Context;
use std::fs::File;
use std::path::PathBuf;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

pub async fn analyze_hf_dataset_asts(dataset_path: &PathBuf) -> anyhow::Result<pipeline_traits::AstStatistics> {
    let parsing_phase_path = dataset_path.join("semantic/parsing-phase/data.parquet");

    let (mut reader, mut stats) = tokio::task::spawn_blocking(move || -> anyhow::Result<_> {
        let file = File::open(&parsing_phase_path)
            .context(format!("Failed to open Parquet file: {}", parsing_phase_path.display()))?;

        let reader_builder = ParquetRecordBatchReaderBuilder::try_new(file)
            .context("Failed to create Parquet reader builder")?;

        let reader = reader_builder.build().context("Failed to build Parquet reader")?;

        let stats = pipeline_traits::AstStatistics::default();

        Ok((reader, stats))
    }).await.context("Blocking task for Parquet reader initialization failed")??;

    while let Some(record_batch) = reader.next().transpose()? {
        let element_type_column = record_batch.column_by_name("element_type")
            .context("element_type column not found in Parquet file")?;
        let line_column = record_batch.column_by_name("line")
            .context("line column not found in Parquet file")?;
        let column_column = record_batch.column_by_name("column")
            .context("column column not found in Parquet file")?;
        let processing_time_ms_column = record_batch.column_by_name("processing_time_ms")
            .context("processing_time_ms column not found in Parquet file")?;
        let rust_version_column = record_batch.column_by_name("rust_version")
            .context("rust_version column not found in Parquet file")?;
        let analyzer_version_column = record_batch.column_by_name("analyzer_version")
            .context("analyzer_version column not found in Parquet file")?;
        let source_snippet_column = record_batch.column_by_name("source_snippet")
            .context("source_snippet column not found in Parquet file")?;

        let element_types = element_type_column.as_any().downcast_ref::<arrow::array::StringArray>()
            .context("element_type column is not a StringArray")?;
        let lines = line_column.as_any().downcast_ref::<arrow::array::UInt32Array>()
            .context("line column is not a UInt32Array")?;
        let columns = column_column.as_any().downcast_ref::<arrow::array::UInt32Array>()
            .context("column column is not a UInt32Array")?;
        let processing_times = processing_time_ms_column.as_any().downcast_ref::<arrow::array::UInt64Array>()
            .context("processing_time_ms column is not a UInt64Array")?;
        let rust_versions = rust_version_column.as_any().downcast_ref::<arrow::array::StringArray>()
            .context("rust_version column is not a StringArray")?;
        let analyzer_versions = analyzer_version_column.as_any().downcast_ref::<arrow::array::StringArray>()
            .context("analyzer_version column is not a StringArray")?;
        let source_snippets = source_snippet_column.as_any().downcast_ref::<arrow::array::StringArray>()
            .context("source_snippet column is not a StringArray")?;

        for i in 0..record_batch.num_rows() {
            if let Some(element_type) = element_types.value(i).to_string().into() {
                *stats.node_type_counts.entry(element_type.clone()).or_insert(0) += 1;

                // Line stats
                let line = lines.value(i) as usize;
                let (min, max, sum, count) = stats.line_stats.entry(element_type.clone()).or_insert((usize::MAX, 0, 0, 0));
                *min = (*min).min(line);
                *max = (*max).max(line);
                *sum += line;
                *count += 1;

                // Column stats
                let column = columns.value(i) as usize;
                let (min, max, sum, count) = stats.column_stats.entry(element_type.clone()).or_insert((usize::MAX, 0, 0, 0));
                *min = (*min).min(column);
                *max = (*max).max(column);
                *sum += column;
                *count += 1;

                // Processing time stats
                let processing_time = processing_times.value(i);
                let (min, max, sum, count) = stats.processing_time_stats.entry(element_type.clone()).or_insert((u64::MAX, 0, 0, 0));
                *min = (*min).min(processing_time);
                *max = (*max).max(processing_time);
                *sum += processing_time;
                *count += 1;

                // Rust version counts
                if let Some(rust_version) = rust_versions.value(i).to_string().into() {
                    *stats.rust_version_counts.entry(rust_version).or_insert(0) += 1;
                }

                // Analyzer version counts
                if let Some(analyzer_version) = analyzer_versions.value(i).to_string().into() {
                    *stats.analyzer_version_counts.entry(analyzer_version).or_insert(0) += 1;
                }

                // Snippet length stats
                let snippet_length = source_snippets.value(i).len();
                let (min, max, sum, count) = stats.snippet_length_stats.entry(element_type.clone()).or_insert((usize::MAX, 0, 0, 0));
                *min = (*min).min(snippet_length);
                *max = (*max).max(snippet_length);
                *sum += snippet_length;
                *count += 1;
            }
        }
    }

    Ok(stats)
}