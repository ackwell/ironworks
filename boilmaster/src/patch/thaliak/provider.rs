use anyhow::Result;
use graphql_client::{GraphQLQuery, Response};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
	endpoint: String,
}

#[derive(GraphQLQuery)]
#[graphql(
	schema_path = "src/patch/thaliak/schema.2022-08-14.json",
	query_path = "src/patch/thaliak/query.graphql",
	response_derives = "Debug"
)]
pub struct RepositoryQuery;

pub struct Provider {
	config: Config,
	client: reqwest::Client,
}

impl Provider {
	pub fn new(config: Config) -> Self {
		Self {
			config,
			client: reqwest::Client::new(),
		}
	}

	// TODO: how does versioning fall in on this? patch _lists_ technically sit above the concept of versions, but SE has a habit of "deprecating" patch files, which means that a patch list is only ever a point-in-time snapshot. Given that, I'm tempted to say that short term patch list should just represent "latest", and when i get around to actually building versioning, a version should "snapshot" the patch list at the time it's created (or configured or whatever) for reproducibility in the repository data cache.
	pub async fn patches(&self, repository: String) -> Result<()> {
		let query = RepositoryQuery::build_query(repository_query::Variables { repository });

		let response = self
			.client
			.post(&self.config.endpoint)
			.json(&query)
			.send()
			.await?
			.json::<Response<repository_query::ResponseData>>()
			.await?;

		println!("repository: {response:#?}");

		Ok(())
	}
}
