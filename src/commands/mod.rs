use serde_json::Value;

pub mod ping;
pub mod getchapter;
pub mod colorspread;
pub mod setquestion;
pub mod qotd;
pub mod getpfp;

#[derive(Debug)]
pub struct Chapter {
    pub number: i64,
    pub title: String,
    pub pages: u64
}

impl Chapter {
    pub fn new(number: i64, title: &str, pages: u64) -> Self {
        Chapter {
            number,
            title: title.to_string(),
            pages
        }
    }

    pub async fn get_latest() -> Option<Chapter> {
        let response = reqwest::get("https://read1piece.org/data/chapters.json").await;

        if let Ok(res) = response {
            let json_content: Value = res.json().await.unwrap();
            let id = json_content[0]["id"].as_i64().unwrap();
            let title = json_content[0]["title"].to_string();
            let pages = json_content[0]["pages"].as_u64().unwrap();

            let chapter = Chapter::new(id, title.trim_matches('"'), pages);
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
                    let pages = item["pages"].as_u64().unwrap();

                    let chapter = Chapter::new(id, title.trim_matches('"'), pages);
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
        format!("# chapter number: {}\n# title: {}\n# pages: {}", self.number, self.title, self.pages)
    }

    pub fn to_medium_header(&self) -> String {
        format!("## chapter number: {}\n## title: {}\n## pages: {}", self.number, self.title, self.pages)
    }
}