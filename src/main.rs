use std::error::Error;

use hackernewsapi::{HackerNewsAPI, StoryType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut hn_api = HackerNewsAPI::new();
    let response = hn_api.collect_all_stories().await?;

    response.debub_print_story(StoryType::Show).await?;
    Ok(())
}
