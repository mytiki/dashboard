
pub mod github_client {

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};

use serde_json::json;


pub async fn create_issue(bearer_token: String,  owner: String, repo: String, title: String, body: String, assignees: Option<Vec<String>>, labels: String, milestone: Option<Vec<String>>) -> Result<String, reqwest::Error> {
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
    "title": title,
    "body": body,
    "assignees": 
      assignees
    ,
    "milestone": milestone,
    "labels": 
      labels
    });
    
  let get_url = format!("https://api.github.com/repos/{}/{}/issues", owner, repo);
 

  let client = reqwest::Client::new();
  let resp = client
    .post(get_url)
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

  pub async fn get_issues(bearer_token: String, owner: String, repo: String) -> 
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


  pub async fn create_issue_comment(bearer_token: String, owner: String, repo: String, issue_number: String, comment: String) -> 
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
      "body": comment,
      });
  
    let post_url = format!("https://api.github.com/repos/{owner}/{repo}/issues/{issue_number}/comments");
    let client = reqwest::Client::new();
    let resp = client
      .post(&post_url)
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
  pub async fn get_issue_comment(bearer_token: String, owner: String, repo: String, issue_number: String) -> 
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
      });
  
    let get_url = format!("https://api.github.com/repos/{owner}/{repo}/issues/{issue_number}/comments");
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

