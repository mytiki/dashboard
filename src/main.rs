mod s3;
mod github;

use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{Client, Error};
use github::github_client;
use rocket::futures::FutureExt;


/// Lists your DynamoDB tables in the default Region or us-east-1 if a default Region isn't set.
#[tokio::main]
async fn main() -> Result<(), Error> { 

    let token = ""; 
    let owner = "";
    let repo = "";

    let results = s3::getFileList("/".to_owned());
    match github_client::create_issue(token.to_owned(), owner.to_owned(), repo.to_owned()).await {
        Ok(result) => println!("Resultado: {}", result),
        Err(e) => println!("Erro ao criar a issue: {}", e),
    };
    // match github_client::list_repositories(token.to_owned(), owner.to_owned(), repo.to_owned()).await {
    //     Ok(result) => println!("Resultado: {}", result),
    //     Err(e) => println!("Erro ao listar a issues: {}", e),
    // };

    Ok(())
}