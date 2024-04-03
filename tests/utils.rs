use cndevwebapp::configuration::get_configuration;
use cndevwebapp::utils::{FileSystemCLient, RepoCLient};

#[tokio::test]
async fn test_list_filesystem_items() {
    let config = get_configuration().expect("Failed to read configuration.");

    let path = format!("{}/posts", &config.content_path.clone());

    match FileSystemCLient::list_files(&path).await {
        Ok(repoclient) => {
            for item in repoclient {
                println!("{:?}", item.path)
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}

#[tokio::test]
async fn test_get_filesystem_item() {
    let config = get_configuration().expect("Failed to read configuration.");

    let file = format!("{}/services/service_01.json", &config.content_path.clone());

    let content = FileSystemCLient::get_file(&file).await;
    println!("\n{}\n", content);
}

#[tokio::test]
#[ignore]
async fn test_list_repo_items() {
    let config = get_configuration().expect("Failed to read configuration.");

    let url = format!(
        "https://api.github.com/repos/{}/contents/posts",
        &config.repo.clone()
    );

    match RepoCLient::list_files(&url).await {
        Ok(repoclient) => {
            for item in repoclient {
                println!("{:?}", item.name)
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
