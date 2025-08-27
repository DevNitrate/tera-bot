use serde_json::Value;

pub mod ping;
pub mod getchapter;

#[derive(Debug)]
pub struct Chapter {
    pub number: i64,
    pub title: String
}

impl Chapter {
    pub fn new(number: i64, title: &str) -> Self {
        Chapter {
            number,
            title: title.to_string()
        }
    }

    pub async fn get_latest() -> Option<Chapter> {
        let response = reqwest::get("https://read1piece.org/data/chapters.json").await;

        if let Ok(res) = response {
            let json_content: Value = res.json().await.unwrap();
            let id = json_content[0]["id"].as_i64().unwrap();
            let title = json_content[0]["title"].to_string();

            let chapter = Chapter::new(id, title.trim_matches('"'));
            Some(chapter)
        } else {
            None
        }
    }

    pub async fn get(number: i64) -> Option<Chapter> {
        let response = reqwest::get("https://read1piece.org/data/chapters.json").await;

        if let Ok(res) = response {
            let json_content: Value = res.json().await.unwrap();

            if let Some(array) = json_content.as_array() {
                if let Some(item) = array.iter().find(|item| item["id"] == number) {
                    let id = item["id"].as_i64().unwrap();
                    let title = item["title"].to_string();

                    let chapter = Chapter::new(id, title.trim_matches('"'));
                    Some(chapter)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn _to_big_header(&self) -> String {
        format!("# chapter number: {}\n# title: {}", self.number, self.title)
    }

    pub fn to_medium_header(&self) -> String {
        format!("## chapter number: {}\n## title: {}", self.number, self.title)
    }
}