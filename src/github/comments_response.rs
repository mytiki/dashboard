// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::comments_response;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: comments_response = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

pub type CommentsResponse = Vec<CommentsResponseElement>;

#[derive(Serialize, Deserialize)]
pub struct CommentsResponseElement {
    url: String,
    html_url: String,
    issue_url: String,
    id: i64,
    node_id: String,
    user: User,
    created_at: String,
    updated_at: String,
    author_association: String,
    body: String,
    reactions: Reactions,
    performed_via_github_app: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Reactions {
    url: String,
    total_count: i64,
    #[serde(rename = "+1")]
    the_1: i64,
    #[serde(rename = "-1")]
    reactions_1: i64,
    laugh: i64,
    hooray: i64,
    confused: i64,
    heart: i64,
    rocket: i64,
    eyes: i64,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    user_type: String,
    site_admin: bool,
}
