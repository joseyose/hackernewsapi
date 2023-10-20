use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

const BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

#[derive(thiserror::Error, Debug)]
enum HackerNewsApiError {
    #[error("Failed Fetching Story")]
    RequestFailed(#[from] ureq::Error),
    #[error("Request failed: {0}")]
    BadRequest(&'static str),
    #[error("Async Request Failed")]
    AsyncRequestFailed(#[from] reqwest::Error),
}

#[derive(Debug, Serialize, Deserialize)]
struct Story {
    by: String,
    descendants: Option<u32>,
    id: u32,
    kids: Option<Vec<u32>>,
    score: u32,
    title: String,
    #[serde(rename = "type")]
    story_type: String,
    url: Option<String>,
}

struct HackerNewsAPI {
    stories: Vec<u64>,
    endpoint: StoryType,
}

impl HackerNewsAPI {
    fn new() -> Self {
        Self {
            stories: vec![],
            endpoint: StoryType::Top,
        }
    }

    fn fetch(&mut self) -> Result<(), HackerNewsApiError> {
        let url = self.prepare_url(None);
        let req = ureq::get(&url);
        let response: Vec<i64> = req.call().unwrap().into_json().unwrap();

        Ok(())
    }

    async fn fetch_stories(&mut self) -> Result<(Vec<u64>), HackerNewsApiError> {
        let url = self.prepare_url(None);

        let stories = reqwest::get(url)
            .await?
            .json::<Vec<u64>>()
            .await
            .map_err(|e| HackerNewsApiError::AsyncRequestFailed(e))?;

        // self.stories = stories;
        // for s in stories.iter().take(20) {
        //     let story_url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", s);
        //     let response = reqwest::get(story_url).await?.json::<Story>().await?;
        //     println!("{:?}\n", response.title);
        // }
        //
        Ok(stories)
    }

    fn prepare_url(&self, id: Option<u32>) -> String {
        let mut url = Url::parse(BASE_URL).unwrap();

        match self.endpoint {
            StoryType::Item => {
                if let Some(item) = &id {
                    url.path_segments_mut()
                        .unwrap()
                        .push(&self.endpoint.to_string());
                    let id = format!("{}.json", item);
                    url.path_segments_mut().unwrap().push(&id);
                }
            }
            _ => {
                let id = format!("{}.json", &self.endpoint.to_string());
                url.path_segments_mut().unwrap().push(&id);
            }
        }

        url.to_string()
    }

    // fn print_story(&mut self) {
    //     for i in self.stories.iter().take(2) {
    //         self.endpoint = StoryType::Item;
    //         let url = self.prepare_url(Some(*i as u32));
    //         let req = ureq::get(&url);
    //         let response: Story = req.call().unwrap().into_json().unwrap();
    //         // println!("{:?}", response);
    //         self.stories_a.push(response);
    //     }
    // }
}

impl Default for HackerNewsAPI {
    fn default() -> Self {
        Self::new()
    }
}
enum StoryType {
    Top,
    New,
    Best,
    Ask,
    Show,
    Job,
    Item,
}

impl ToString for StoryType {
    fn to_string(&self) -> String {
        match self {
            Self::Top => "topstories".to_string(),
            Self::New => "newstories".to_string(),
            Self::Best => "beststories".to_string(),
            Self::Ask => "askstories".to_string(),
            Self::Show => "showstories".to_string(),
            Self::Job => "jobstories".to_string(),
            Self::Item => "item".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct TopStories {
    stories: Vec<i64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let top_stories = "https://hacker-news.firebaseio.com/v0/topstories.json";

    // let response = reqwest::get(top_stories).await?.text().await?;

    let top_stories_vec = reqwest::get(top_stories).await?.json::<Vec<u32>>().await?;

    for s in top_stories_vec.iter().take(20) {
        let story_url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", s);
        let response = reqwest::get(story_url).await?.json::<Story>().await?;
        println!("{:?}\n", response.title);
    }

    Ok(())
}

fn map_response_err(code: Option<String>) -> HackerNewsApiError {
    if let Some(code) = code {
        HackerNewsApiError::BadRequest("Unknown Error...")
    } else {
        HackerNewsApiError::BadRequest("Uknown Error...")
    }
}
