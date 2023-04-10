# YouTube Podcasts Feed

Rust implementation of my [tg-youtube-podcasts-bot](https://github.com/wckd1/tg-youtube-podcasts-bot)

This is self-hosted service for extract audio from YouTube videos and build rss feed that can be added to Podcast app.

It allows to subscribe to YouTube channels/playlists using Telegram bot and automatically get updates to feed.
Also, single item can be added without subscription.

The service uses [yt-dlp](https://github.com/yt-dlp/yt-dlp) to pull videos and [ffmpeg](https://www.ffmpeg.org/) for audio extraction.

## Bot commands

### `add`
Add subscription or single video to feed.

Add single item
```
/add https://youtube.com/watch?v={id}
```

Subscribe to channel
```
/add https://youtube.com/c/{id}
/add https://youtube.com/channel/{id}
```

Subscribe to playlist
```
/add https://youtube.com/watch?v={video_id}&list={id}
/add https://youtube.com/playlist?list={id}
```

Filter string can be added to subscription to get only specified updates
```
/add https://youtube.com/c/{id} {some title entry}
```

### `remove`
Removes specified subscription. Same syntax as fo adding subscription.

## API
Servise expose only one endpoint for adding feed to Podcasts app.

- `GET /rss/{key}` - returns generated rss xml with configured limit

## Configuration
Add .env file following the example of [.env.example](https://github.com/wckd1/rust-podcast-bot/blob/main/.env.example)

- `BOT_TOKEN` - token for Telegram bot to communicate with;
- `UPDATE_INTERVAL` - iterval for feed update in minutes (min 10 mins are recommended);
