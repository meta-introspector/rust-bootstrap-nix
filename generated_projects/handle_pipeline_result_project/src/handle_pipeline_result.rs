use std::collections::HashSet;
use split_expanded_lib::{DeclarationItem};

pub async fn handle_pipeline_result (result : anyhow :: Result < () > ,) -> anyhow :: Result < () > { if let Err (ref e) = result { let mut stderr = tokio :: io :: stderr () ; stderr . write_all (:: alloc :: __export :: must_use ({ :: alloc :: fmt :: format (format_args ! ("Pipeline failed: {0:?}\n" , e) ,) }) . as_bytes () ,) . await ? ; } else { let mut stdout = tokio :: io :: stdout () ; stdout . write_all (b"Pipeline completed successfully.\n") . await ? ; } result }