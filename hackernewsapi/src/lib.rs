use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

const BASE_URL: &str = "https://hacker-news.firebaseio.com/v0";

#[derive(Debug)]
pub struct HackerNewsResponse {
    top: Option<Vec<u64>>,
    new: Option<Vec<u64>>,
    best: Option<Vec<u64>>,
    ask: Option<Vec<u64>>,
    show: Option<Vec<u64>>,
    job: Option<Vec<u64>>,
    stories: Option<Vec<HashMap<u64, Story>>>,
}

impl HackerNewsResponse {
    pub async fn debub_print_story(
        &self,
        story_type: StoryType,
        amount: u8,
    ) -> Result<(), HackerNewsApiError> {
        // create a vec to store ids we want to work with
        let amount = amount as usize;
        let mut ids: Vec<u64> = Vec::new();
        match story_type {
            StoryType::Top => {
                println!("Story Type: Top");
                ids = if let Some(story) = &self.top {
                    story.iter().cloned().take(amount).collect()
                } else {
                    Vec::new()
                }
            }
            StoryType::New => {
                println!("Story Type: New");
                ids = if let Some(story) = &self.new {
                    story.iter().cloned().take(amount).collect()
                } else {
                    Vec::new()
                }
            }
            StoryType::Best => {
                println!("Story Type: Best");
                ids = if let Some(story) = &self.best {
                    story.iter().cloned().take(amount).collect()
                } else {
                    Vec::new()
                }
            }
            StoryType::Ask => {
                println!("Story Type: Ask");
                ids = if let Some(story) = &self.ask {
                    story.iter().cloned().take(amount).collect()
                } else {
                    Vec::new()
                }
            }
            StoryType::Job => {
                println!("Story Type: Job");
                ids = if let Some(story) = &self.job {
                    story.iter().cloned().take(amount).collect()
                } else {
                    Vec::new()
                }
            }
            StoryType::Show => {
                println!("Story Type: Show");
                ids = if let Some(story) = &self.show {
                    story.iter().cloned().take(amount).collect()
                } else {
                    Vec::new()
                }
            }
            _ => {}
        }

        for (index, id) in ids.iter().enumerate() {
            let story = self.fetch_story(*id).await?;
            println!("Story #{} - id: {} - {}", index, id, story.title);
        }

        Ok(())
    }

    pub async fn debug_print_stories(&self, amount: u8) -> Result<(), HackerNewsApiError> {
        self.debub_print_story(StoryType::Show, amount).await?;
        self.debub_print_story(StoryType::Job, amount).await?;
        self.debub_print_story(StoryType::Best, amount).await?;
        self.debub_print_story(StoryType::Top, amount).await?;
        self.debub_print_story(StoryType::New, amount).await?;
        self.debub_print_story(StoryType::Ask, amount).await?;

        Ok(())
    }

    async fn fetch_story(&self, id: u64) -> Result<Story, HackerNewsApiError> {
        let url = format!("{}/item/{}.json?", BASE_URL, id);

        let story = reqwest::get(url)
            .await?
            .json::<Story>()
            .await
            .map_err(|e| HackerNewsApiError::AsyncRequestFailed(e))?;

        Ok(story)
    }
}

impl Default for HackerNewsResponse {
    fn default() -> Self {
        Self {
            top: None,
            new: None,
            best: None,
            ask: None,
            show: None,
            job: None,
            stories: None,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum HackerNewsApiError {
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

pub struct HackerNewsAPI {
    endpoint: StoryType,
}

impl HackerNewsAPI {
    /// Creates a new [`HackerNewsAPI`].
    pub fn new() -> Self {
        Self {
            endpoint: StoryType::Top,
        }
    }

    async fn fetch_stories(&mut self) -> Result<Vec<u64>, HackerNewsApiError> {
        let url = self.prepare_url(None);

        let stories = reqwest::get(url)
            .await?
            .json::<Vec<u64>>()
            .await
            .map_err(|e| HackerNewsApiError::AsyncRequestFailed(e))?;

        Ok(stories)
    }

    pub async fn collect_all_stories(&mut self) -> Result<HackerNewsResponse, HackerNewsApiError> {
        let mut response = HackerNewsResponse::default();
        let endpoints = [
            StoryType::Top,
            StoryType::New,
            StoryType::Ask,
            StoryType::Job,
            StoryType::Best,
            StoryType::Show,
        ];

        for i in endpoints {
            match i {
                StoryType::Top => {
                    self.endpoint = StoryType::Top;
                    response.top = Some(self.fetch_stories().await?);
                }
                StoryType::New => {
                    self.endpoint = StoryType::New;
                    response.new = Some(self.fetch_stories().await?);
                }
                StoryType::Ask => {
                    self.endpoint = StoryType::Ask;
                    response.ask = Some(self.fetch_stories().await?);
                }
                StoryType::Job => {
                    self.endpoint = StoryType::Job;
                    response.job = Some(self.fetch_stories().await?);
                }
                StoryType::Best => {
                    self.endpoint = StoryType::Best;
                    response.best = Some(self.fetch_stories().await?);
                }
                StoryType::Show => {
                    self.endpoint = StoryType::Show;
                    response.show = Some(self.fetch_stories().await?);
                }
                _ => {}
            }
        }

        Ok(response)
    }

    fn prepare_url(&self, id: Option<u64>) -> String {
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
}

impl Default for HackerNewsAPI {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug)]
pub enum StoryType {
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

fn map_response_err(code: Option<String>) -> HackerNewsApiError {
    if let Some(code) = code {
        HackerNewsApiError::BadRequest("Unknown Error...")
    } else {
        HackerNewsApiError::BadRequest("Uknown Error...")
    }
}
