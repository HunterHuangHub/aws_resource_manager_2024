use aws_sdk_s3::{Client, Error};
use aws_config::{load_defaults, BehaviorVersion};
use std::io;
use anyhow::Result;

use std::error::Error as StdError;
use std::fmt;


#[tokio::main]
async fn main() -> Result<(), Error> {
    // Load AWS configuration with explicit behavior version
    let config = load_defaults(BehaviorVersion::latest()).await;

    // Create an S3 client
    let client = Client::new(&config);

    // Example: List all buckets
    println!("Listing all S3 buckets...");
    match client.list_buckets().send().await {
        Ok(output) => {
            if let Some(buckets) = output.buckets {
                for bucket in buckets {
                    println!("Bucket: {}", bucket.name.as_deref().unwrap_or_default());
                }
            } else {
                println!("No buckets found.");
            }
        }
        Err(e) => {
            println!("Error listing buckets: {}", e);
        }
    }

    Ok(())
}

// --- --- --- --- --- --- --- --- ---
// --- --- --- Rust Notes: --- --- ---
// --- --- --- --- --- --- --- --- ---

// Notes for main():

// 1. if let Some(buckets) is the same as: 
// match output.buckets {
//     Some(buckets) => { /* Use buckets */ }
//     None => { /* Do nothing or handle None */ }
// }

// 2. println!("Bucket: {}", bucket.name.as_deref().unwrap_or_default());
// .as_deref(): Converts an Option<&String> into an Option<&str>
// .unwrap_or_default(): Returns the value inside the Option if it exists, or a default value.
// Option<&String>: value is a ref. to a String inside the struct.
// Option<&str>: ref. to a string slice (&str)— flexible representation.
// &str: ref. to the content of a string & can point to substrings/str literals. 
// Converting to &str lets you work with the value w/o needing a full Str.

// --- --- --- --- --- --- --- --- ---
// --- --- --- --- --- --- --- --- ---
// --- --- --- --- --- --- --- --- ---

async fn upload_file(client: &Client, bucket: &str, file_path: &str) -> Result<()> {
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;

    // Read the file content
    let mut file = File::open(file_path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    // Extract file name from the path
    let file_name = std::path::Path::new(file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?;

    // Upload the file
    client
        .put_object()
        .bucket(bucket)
        .key(file_name)
        .body(buffer.into())
        .send()
        .await?;

    println!("File '{}' uploaded to bucket '{}'", file_name, bucket);
    Ok(())
}


// Depreciated main()
// Version with load_from_env()

// use aws_sdk_s3::{Client, Error};
// use aws_config;

// #[tokio::main]
// async fn main() -> Result<(), Error> { // Async: next statement gets executed without waiting for the previous one.
//     // Load AWS configuration from the environment
//     let config = aws_config::load_from_env().await;

//     // Create an S3 client
//     let client = Client::new(&config);

//     // Example: List all buckets
//     println!("Listing all S3 buckets...");
//     match client.list_buckets().send().await {
//         Ok(output) => {
//             if let Some(buckets) = output.buckets {
//                 for bucket in buckets {
//                     println!("Bucket: {}", bucket.name.as_deref().unwrap_or_default());
//                 }
//             } else {
//                 println!("No buckets found.");
//             }
//         }
//         Err(e) => {
//             println!("Error listing buckets: {}", e);
//         }
//     }

//     Ok(())
// }

// Version with load_defaults()