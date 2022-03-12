pub mod tiktok {
    use serde::{Serialize, Deserialize};
    use reqwest::header::CONNECTION;
    use std::path::Path;
    use ms_converter::get_max_possible_duration_long;
    use url::Url;
    use chrono::prelude::*;
    use std::env;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PlayAddr {
        pub width : i16,
        pub height : i16,
        pub url_list : Vec<String>
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Video {
        pub duration : i64,
        pub play_addr : PlayAddr,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Author {
        pub unique_id : String
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct AwemeDetail {
        pub author : Author,
        pub aweme_id : String,
        pub desc : String,
        pub create_time : i64,
        pub video : Video
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Response {
        pub aweme_detail: AwemeDetail
    }
    impl Response {
        pub async fn new(video_url : &str) -> Result<Response, Box<dyn std::error::Error>> {
            let id = get_id(video_url).await?;
            let url = format!("http://api2.musical.ly/aweme/v1/aweme/detail/?aweme_id={}", id);

            let user = env::var("USER").unwrap();
            let password = env::var("PASSWORD").unwrap();
            let ip = env::var("IP").unwrap();
            let port = env::var("PORT").unwrap();

            let proxy_url = format!("http://{}:{}@{}:{}", user, password, ip, port);

            let proxy = reqwest::Proxy::http(proxy_url)?;

            let client = reqwest::Client::builder()
                .proxy(proxy)
                .build()?;

            let resp = client.get(&url).send()
                .await?;

            let detail : Response;
            if let Ok(state) = resp.json().await {
                detail = state;
            } else {
                let res = reqwest::get(url).await?;
                detail = res.json().await?;
            }

            Ok(detail)
        }
        pub fn get_video_url(&self) -> Option<String> {
            let first = self.aweme_detail.video.play_addr.url_list.get(0);
            if let Some(state) = first {
                Some(state.to_owned())
            } else {
                None
            }
        }
        pub async fn download_video(&self) -> Result<(), Box<dyn std::error::Error>> {
            let video_url = self.get_video_url().unwrap();
            let response = reqwest::get(video_url).await?;
            let file_name = format!("{}.mp4", self.aweme_detail.aweme_id);
            let mut file = std::fs::File::create(file_name)?;
            let mut content = std::io::Cursor::new(response.bytes().await?);
            std::io::copy(&mut content, &mut file)?;
            Ok(())
        }
        pub async fn delete_video(&self) -> Result<(), Box<dyn std::error::Error>> {
            let file_name = self.get_file_name();
            std::fs::remove_file(file_name)?;
            Ok(())
        }
        pub fn get_description(&self) -> String {
            self.aweme_detail.desc.to_owned()
        }
        pub fn get_duration(&self) -> String {
            let duration = get_max_possible_duration_long(self.aweme_detail.video.duration as i64).unwrap();
            format!("~{}", duration)
        }
        pub fn get_file_name(&self) -> String {
            format!("{}.mp4", self.aweme_detail.aweme_id)
        }
        pub fn get_date_created(&self) -> String {
            let naive = NaiveDateTime::from_timestamp(self.aweme_detail.create_time, 0);
            
            let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
            datetime.to_string()
        } 
    }
        
    async fn get_id(uri : &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut url = Url::parse(uri)?;
        url.set_query(None);
        url.set_scheme("http").ok();
    
        let resp = reqwest::Client::new().get(url)
            .header(CONNECTION, "keep-alive").send().await?;
    
        let mut url = resp.url().to_owned();
        url.set_query(None);
        url.set_scheme("http").ok();
        let path = Path::new(url.as_str());
        let path = path.file_stem().unwrap().to_owned();
        Ok(path.into_string().unwrap())
    }
}