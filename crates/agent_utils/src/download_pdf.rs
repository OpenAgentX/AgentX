use anyhow::Result;
use futures::StreamExt;
use reqwest::Client;
use std::{fs, io::Write};

pub async fn download_pdf(pdf_url: &str, out_path: &str) -> Result<()> {

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36".parse()?); // 替换成您的用户代理

    let client = Client::builder().default_headers(headers).build()?;

    let response = client.get(pdf_url).send().await?;

    if !response.status().is_success() {
        eprintln!("Failed to download PDF: {:?}", response.status());
        return Ok(());
    }

    let out_path = if out_path.ends_with(".pdf") {
        out_path.to_string()
    } else {
        format!("{}.pdf", out_path)
    };
    let mut file = fs::File::create(out_path)?;
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)?;
    }

    Ok(())
}
