use super::repos::{Repo, RepoRequest};
pub use crate::graphql::repo_basic_info::repo_basic_info_query as basic_info_response;
use crate::utils::http::{request, HttpMethod};
use crate::{graphql::repo_basic_info::RepoBasicInfoQuery, utils::graphql::query_graphql};
use anyhow::Result;
use async_trait::async_trait;
use graphql_client::GraphQLQuery as _;

const BASE_URL: &str = "https://api.github.com/repos";

pub type BasicInfoResponse = basic_info_response::ResponseData;

#[async_trait]
pub trait BasicInfo {
    async fn get_basic_info(&self) -> Result<BasicInfoResponse>;
    async fn get_raw_readme(&self) -> Result<String>;
}

impl From<&Repo<'_>> for basic_info_response::Variables {
    fn from(repo: &Repo<'_>) -> Self {
        basic_info_response::Variables {
            name: repo.repo_name.to_owned(),
            owner: repo.repo_owner.to_owned(),
        }
    }
}

#[async_trait]
impl BasicInfo for RepoRequest<'_> {
    async fn get_basic_info(&self) -> Result<BasicInfoResponse> {
        let RepoRequest(repo, auth_token) = self;
        let graphql_query = RepoBasicInfoQuery::build_query(repo.into());
        let resp = query_graphql(graphql_query, &auth_token).await?;
        resp.data
            .ok_or_else(|| anyhow::anyhow!("Couldn't find repository basic information!"))
    }

    async fn get_raw_readme(&self) -> Result<String> {
        let RepoRequest(repo, auth_token) = self;
        let resp = request(
            &format!("{}/{}/readme", BASE_URL, repo),
            HttpMethod::GET,
            auth_token,
        )
        .header("Accept", "application/vnd.github.VERSION.raw")
        .call()
        .await?;
        resp.body().await
    }
}
