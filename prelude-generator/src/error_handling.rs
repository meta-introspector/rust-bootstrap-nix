
use tokio::io::AsyncWriteExt;

pub async fn handle_pipeline_result(result: anyhow::Result<()>) -> anyhow::Result<()> {
    if let Err(ref e) = result {
        let mut stderr = tokio::io::stderr();
        stderr.write_all(format!("Pipeline failed: {:?}\n", e).as_bytes()).await?;
    } else {
        let mut stdout = tokio::io::stdout();
        stdout.write_all(b"Pipeline completed successfully.\n").await?;
    }
    result
}
