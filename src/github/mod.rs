
pub mod github_client {

use std::env;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};

use serde_json::json;


pub async fn create_issue(bearer_token: String) -> Result<String, reqwest::Error> {
  dotenv::dotenv().ok();

  let auth_token: String = format!("Bearer {}", bearer_token);

  let mut headers = HeaderMap::new();
  headers.insert(
      AUTHORIZATION,
      HeaderValue::from_str(&auth_token).expect("Invalid header value"),
  );

  headers.insert(
      reqwest::header::HeaderName::from_static("notion-version"), // header name needs to be lowercase
      HeaderValue::from_static("2022-06-28"),
  );
  let body = json!({
    "query": "Medium",
    "filter":  {
        "value": "database",
        "property": "object"
        }
    });

  let client = reqwest::Client::new();
  let resp = client
    .post("https://api.github.com/repos/OWNER/REPO/issues")
    .headers(headers)
    .json(&body)
    .send()
    .await?; 

    if resp.status().is_success() {
      let resp_body = resp.text().await?;
      Ok(resp_body)
    } else {
        let error_message = format!("Failed to create issue: {}", resp.status());
        println!("{}", error_message);
        Err(resp.error_for_status().unwrap_err())    }

  }

  pub async fn list_repositories(bearer_token: String, owner: String, repo: String) -> 
  Result<String, reqwest::Error> {
    dotenv::dotenv().ok();

    let auth_token: String = format!("Bearer {}", bearer_token);
  
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&auth_token).expect("Invalid header value"),
    );
    headers.insert(
      ACCEPT,
      HeaderValue::from_str("application/vnd.github+json").unwrap() // header name needs to be lowercase
    );

    headers.insert(
      USER_AGENT, 
      HeaderValue::from_str("rust web-api-client demo").unwrap()
    );
  
    headers.insert(
      HeaderName::from_lowercase("x-github-api-version".as_bytes()).unwrap(), // header name needs to be lowercase
      HeaderValue::from_static("2022-11-28"),
    );
  
    let body = json!({
      "query": "Medium",
      "filter":  {
          "value": "database",
          "property": "object"
          }
      });
  
    let get_url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);
    let client = reqwest::Client::new();
    let resp = client
      .get(&get_url)
      .headers(headers)
      .json(&body)
      .send()
      .await?; 
    
    if resp.status().is_success() {
      let resp_body = resp.text().await?;
      println!("{}", resp_body);
      Ok(resp_body)
    } else {
        let error_message = format!("Failed to list issue: {}", resp.status());
        println!("{}", error_message);
        Err(resp.error_for_status().unwrap_err())
    }
  }
}

