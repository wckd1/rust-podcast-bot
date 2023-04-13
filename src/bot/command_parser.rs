use url::Url;
use teloxide::utils::command::ParseError;

use crate::models::youtube_item::YouTubeItem;

//
// Supported links:
//
// Video:
// /watch?v={id}
//
// Channel:
// /c/{id}
// /channel/{id}
// /@id
//
// Playlist:
// /watch?v={video_id}&list={id}
// /playlist?list={id}
//
pub fn parse_youtube_item(input: String) -> Result<(String, YouTubeItem), ParseError> { // TODO: Get rid of id
    let mut sub = YouTubeItem::default();
    let input_url: String;

    // Check if contains optional filters
    if let Some((url, filter)) = input.split_once(" ") {
        input_url = url.to_string();
        sub.filter = Some(filter.to_string());
    } else {
        input_url = input.clone();
    }

    // Check if first argument is valid URL
    let url = Url::parse(&input_url)
        .or_else(|_| Err(ParseError::UnknownCommand("Invalid source url provided".to_string())))?;

    // Check if YouTube url is provided
    if let (Some(full_host), Some(path_split)) = (url.host_str(), url.path_segments()) {
        let host = full_host.replace("www.", "");
        if host != "youtube.com" {
            return Err(ParseError::UnknownCommand("Only YouTube links are supported".to_string()));
        }

        // Parse type
        let path = path_split.clone().collect::<Vec<&str>>()[0].to_string();

        // Is playlist
        if let Some(list_pair) = url.query_pairs().find(|pair| pair.0 == "list") {
            if path == "watch" || path == "playlist" {
                sub.id = list_pair.1.to_string();
                sub.url = format!("https://www.youtube.com/playlist?list={}", sub.id);
                sub.filter = None;

                return Ok((sub.id.clone(), sub));
            } else {
                return Err(ParseError::UnknownCommand("Invalid playlist link".to_string()));
            }
        }

        // Is video
        if let Some(video_pair) = url.query_pairs().find(|pair| pair.0 == "v") {
            if path == "watch" {
                sub.id = video_pair.1.to_string();
                sub.url = input_url.clone();
                sub.is_video = true;
                sub.filter = None;

                return Ok((sub.id.clone(), sub));
            } else {
                return Err(ParseError::UnknownCommand("Invalid video link".to_string()));
            }
        }
        
        // Is channel
        let mut channel_id: Option<String> = None;
        
        if path == "c" || path == "channel" {
            channel_id = Some(path_split.clone().collect::<Vec<&str>>()[1].to_string());
        } else if path.starts_with("@") { // TODO: Add test for this case
            channel_id = Some(path);
        }

        if let Some(id) = channel_id {
            if let Some(filter) = &sub.filter {
                sub.id = format!("{}_{}", id, filter);
            } else {
                sub.id = id;
            }

            sub.url = input_url.clone();

            return Ok((sub.id.clone(), sub));
        }
    }
    
    Err(ParseError::UnknownCommand("Incorrect source url provided".to_string()))
}

#[test]
fn test_video_subscription_parse() {
    let url = "https://www.youtube.com/watch?v=video_id".to_string();
    match parse_youtube_item(url.clone()) {
        Ok(res) => {
            assert_eq!(res.0, "video_id");
            assert_eq!(res.1.id, res.0);
            assert_eq!(res.1.url, url);
            assert_eq!(res.1.is_video, true);
            assert_eq!(res.1.filter, None);
        },
        Err(e) => panic!("Single video parse failed: {}", e),
    }
}

#[test]
fn test_invalid_video_subscription_parse() {
    let url = "https://www.youtube.com/foo?v=video_id".to_string();
    assert!(parse_youtube_item(url.clone()).is_err());
}

#[test]
fn test_playlist_subscription_parse() {
    let playlist_url = "https://www.youtube.com/playlist?list=list_id".to_string();
    match parse_youtube_item(playlist_url.clone()) {
        Ok(res) => {
            assert_eq!(res.0, "list_id");
            assert_eq!(res.1.id, res.0);
            assert_eq!(res.1.url, playlist_url);
            assert_eq!(res.1.is_video, false);
            assert_eq!(res.1.filter, None);
        },
        Err(e) => panic!("Playlist url parse failed: {}", e),
    }

    let watch_url = "https://www.youtube.com/watch?v=video_id&list=list_id".to_string();
    match parse_youtube_item(watch_url.clone()) {
        Ok(res) => {
            assert_eq!(res.0, "list_id");
            assert_eq!(res.1.id, res.0);
            assert_eq!(res.1.url, "https://www.youtube.com/playlist?list=list_id");
            assert_eq!(res.1.is_video, false);
            assert_eq!(res.1.filter, None);
        },
        Err(e) => panic!("Watch url parse failed: {}", e),
    }
}

#[test]
fn test_invalid_playlist_subscription_parse() {
    let url = "https://www.youtube.com/foo?list=list_id".to_string();
    assert!(parse_youtube_item(url.clone()).is_err());
}

#[test]
fn test_channel_subscription_parse() {
    let short_url = "https://www.youtube.com/c/channel_id".to_string();
    match parse_youtube_item(short_url.clone()) {
        Ok(res) => {
            assert_eq!(res.0, "channel_id");
            assert_eq!(res.1.id, res.0);
            assert_eq!(res.1.url, short_url);
            assert_eq!(res.1.is_video, false);
            assert_eq!(res.1.filter, None);
        },
        Err(e) => panic!("Short url parse failed: {}", e),
    }

    let full_url = "https://www.youtube.com/channel/channel_id".to_string();
    match parse_youtube_item(full_url.clone()) {
        Ok(res) => {
            assert_eq!(res.0, "channel_id");
            assert_eq!(res.1.id, res.0);
            assert_eq!(res.1.url, full_url);
            assert_eq!(res.1.is_video, false);
            assert_eq!(res.1.filter, None);
        },
        Err(e) => panic!("Full url parse failed: {}", e),
    }

    let filter_url = "https://www.youtube.com/channel/channel_id".to_string();
    let filter = "Filter".to_string();
    let built_url = filter_url.clone() + " " + &filter.clone();
    match parse_youtube_item(built_url){
        Ok(res) => {
            assert_eq!(res.0, "channel_id_Filter");
            assert_eq!(res.1.id, res.0);
            assert_eq!(res.1.url, filter_url);
            assert_eq!(res.1.is_video, false);
            assert_eq!(res.1.filter, Some("Filter".to_string()));
        },
        Err(e) => panic!("Full url parse failed: {}", e),
    }
}

#[test]
fn test_not_url_subscription_parse() {
    let url = "just text".to_string();
    assert!(parse_youtube_item(url.clone()).is_err());
}

#[test]
fn test_not_youtube_subscription_parse() {
    let url = "https://www.google.com".to_string();
    assert!(parse_youtube_item(url.clone()).is_err());
}
