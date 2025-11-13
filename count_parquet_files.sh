#!/usr/bin/env bash

PARQUET_DIR="./generated/hf_dataset_output/"

if [ -d "$PARQUET_DIR" ]; then
  echo "Counting .parquet files in $PARQUET_DIR..."
  find "$PARQUET_DIR" -name "*.parquet" | wc -l
else
  echo "Directory $PARQUET_DIR does not exist."
fi
