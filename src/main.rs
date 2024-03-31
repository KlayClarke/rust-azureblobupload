use std::{fs::File, io::Read};


use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::stream::StreamExt;
use native_dialog::FileDialog;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    dotenv().ok();

    let file = FileDialog::new()
        .set_location("~")
        .add_filter("Image", &["jpg"])
        .show_open_single_file()
        .unwrap();

    let file_path = file.unwrap();

    let mut file = File::open(file_path).unwrap();
    let mut file_contents = Vec::new();
    file.read_to_end(&mut file_contents).unwrap();

    let account = std::env::var("ACCOUNT").expect("Failed to retrieve ACCOUNT environment variable!");
    let key = std::env::var("KEY").expect("Failed to retrieve KEY environment variable!");
    let container_name = std::env::var("CONTAINER_NAME").expect("Failed to retrieve CONTAINER_NAME environment variable!");
    let blob_name = std::env::var("BLOB_NAME").expect("Failed to retrieve BLOB_NAME environment variable!");

    let storage_credentials = StorageCredentials::access_key(account.clone(), key);
    let blob_client = ClientBuilder::new(account, storage_credentials).blob_client(container_name, blob_name);

    blob_client.put_block_blob(file_contents).content_type("image/jpg").await?;

    let mut result: Vec<u8> = vec![];

    let mut stream = blob_client.get().into_stream();
    while let Some(value) = stream.next().await {
        let mut body = value?.data;

        while let Some(value) = body.next().await {
            let value = value?;
            result.extend(&value);
        }
    }

    println!("{:?}", result);

    Ok(())
}
