use reqwest::header;
use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::Value;
use std::io;
use tokio::fs;

pub struct Helpers {}

impl Helpers {
    pub fn get_last_number_of_items<T: Clone>(list: Vec<T>, number_of_items: usize) -> Vec<T> {
        let len = list.len();
        // If the vector has less than 4 elements, return all elements
        if len >= number_of_items {
            // Slice the vector to get the last 4 elements
            list[len - &number_of_items..].to_vec()
        } else {
            // If the vector has less than 4 elements, return the whole vector
            list
        }
    }

    pub fn get_first_number_of_words(text: &str, number: usize) -> String {
        let mut words = text.split_whitespace();
        let mut first_words = String::new();

        for _ in 0..number {
            if let Some(word) = words.next() {
                first_words.push_str(word);
                first_words.push(' ');
            } else {
                break;
            }
        }

        first_words.trim().to_string()
    }
}

#[derive(Deserialize, Debug, Clone, Default, Eq, Ord, PartialEq, PartialOrd)]
#[allow(dead_code)]
pub struct FileSystemResponse {
    pub name: String,
    pub path: String,
}

pub struct FileSystemCLient {}

impl FileSystemCLient {
    pub async fn get_file(file_path: &str) -> Value {
        // Read the file contents
        match fs::read_to_string(file_path).await {
            Ok(contents) => {
                // Parse the JSON content
                match serde_json::from_str::<Value>(&contents) {
                    Ok(json) => json,
                    Err(e) => {
                        eprintln!("Failed to parse JSON: {}", e);
                        Value::default()
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read file: {}", e);
                Value::default()
            }
        }
    }

    pub async fn list_files(folder_path: &str) -> Result<Vec<FileSystemResponse>, io::Error> {
        let mut files = Vec::new();

        // Open the directory
        let mut dir = match fs::read_dir(folder_path).await {
            Ok(dir) => dir,
            Err(err) => return Err(io::Error::new(io::ErrorKind::NotFound, err)), // Handle folder not found error
        };

        // Iterate over the entries in the directory
        while let Some(entry) = dir.next_entry().await? {
            let file_type = entry.file_type().await?;
            let file_name = entry.file_name();

            if file_type.is_file() {
                // If it's a file, add it to the list of files
                let file_info = FileSystemResponse {
                    name: file_name.clone().into_string().unwrap(), // Convert OsString to String
                    path: format!(
                        "{}/{}",
                        folder_path.to_string(),
                        file_name.clone().into_string().unwrap()
                    ),
                    // Add more file info here as needed
                };
                files.push(file_info);
            }
        }

        files.sort_by(|a, b| a.cmp(b));
        Ok(files)
    }
}

// GITHUB CLIENT NOT USED IN CURRENT APP //////////////////////////////////
///////////////////////////////////////////////////////////////////////////
#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct GitHubResponse {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub size: u64,
    pub url: String,
    pub html_url: String,
    pub git_url: String,
    pub download_url: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[allow(dead_code)]
pub struct RepoCLient {}

#[allow(dead_code)]
impl RepoCLient {
    pub async fn get_file(file_url: &str) -> Value {
        // Create a reqwest client
        let client = Client::new();

        // Send the request and handle errors
        match client.get(file_url).send().await {
            Ok(response) => {
                // Check if the request was successful
                if response.status().is_success() {
                    // Deserialize the JSON response
                    match response.json::<Value>().await {
                        Ok(json) => json,
                        Err(e) => {
                            eprintln!("Failed to parse JSON: {}", e);
                            Value::default()
                        }
                    }
                } else {
                    eprintln!("Request failed with status code: {}", response.status());
                    Value::default()
                }
            }
            Err(e) => {
                eprintln!("Failed to send request: {}", e);
                Value::default()
            }
        }
    }

    pub async fn list_files(folder_url: &str) -> Result<Vec<GitHubResponse>, Error> {
        let client = reqwest::Client::new();

        // Create request builder with authorization header
        let resp = client
            .get(folder_url)
            .header(header::USER_AGENT, "pbweb")
            //.header(header::AUTHORIZATION, format!("token {}", token))
            .send()
            .await?;

        // Check if the response was successful (status code 2xx)
        if !resp.status().is_success() {
            if resp.status() == reqwest::StatusCode::FORBIDDEN {
                println!(
                    "GitHub API request failed with status code: {}",
                    resp.status(),
                );
            }
        }

        // Try to parse the response as JSON
        let json_body: Vec<GitHubResponse> = resp.json().await?;
        Ok(json_body)
    }
}
