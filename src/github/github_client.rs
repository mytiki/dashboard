use async_trait::async_trait;
use aws_sdk_s3::{operation::{get_object::GetObjectError, list_objects_v2::ListObjectsV2Error}, Client as S3Client};

#[async_trait]
pub trait CreateIssue {
    async fn create_issue(&self, bearer_token: &str) -> Result<Vec<u8>, GetObjectError>;
}

impl CreateIssue for github_client {
    async fn create_issue(&self, bearer_token: &str) -> Result<Vec<u8>, GetObjectError>;
}

#[async_trait]
pub trait ListRepositories {
    async fn list_repositories(&self, bearer_token: &str, owner: &str, repo: &str) -> Result<Vec<u8>, GetObjectError>;
}

impl ListRepositories for github_client {
    async fn list_repositories(&self, bearer_token: &str, owner: &str, repo: &str) -> Result<Vec<u8>, GetObjectError>;
}