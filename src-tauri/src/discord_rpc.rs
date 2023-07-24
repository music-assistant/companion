use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use serde_json;
use std::time::{SystemTime, UNIX_EPOCH};
use tungstenite::connect;
use url::Url;

// Discord client id for MASS application
const CLIENT_ID: &str = "1107294634507518023";

// Struct for storing data about songs.
struct Song {
    name: String,
    artist: String,
    artist_image: String,
    album: String,
    album_image: String,
    end: i64,
    started: i64,
    preview_url: String,
}

// Function for running the Discord rich presence
pub fn start_rpc(mass_ws: String) {
    // Create the discord rpc client
    let mut client: DiscordIpcClient =
        DiscordIpcClient::new(CLIENT_ID).expect("Coulnd't create the discord client!");

    // Connect to the discord rich presence socket
    client
        .connect()
        .expect("Failure while connecting to discord rpc socket");

    // Connect to MASS socket
    let (mut socket, _response) = connect(Url::parse(mass_ws.as_str()).unwrap())
        .expect("Can't connect to mass socket.. Is it running? Is the port 8095 open?");

    // Continuously update the thing
    loop {
        // Read the websocket
        let msg: tungstenite::Message = socket.read_message().expect("msg");

        // Parse the response to text
        let msg_text: &str = msg
            .to_text()
            .expect("Coulnd't convert response to text")
            .clone();
        // Parse to json. Sometimes fails there wrapped in match thing
        let msg_json: serde_json::Value = match serde_json::from_str(&msg_text) {
            Ok(msg_json) => {
                msg_json
            }
            Err(_) => continue
        };
        
        //let msg_json: serde_json::Value =
        //    serde_json::from_str(&msg_text).expect("Response JSON was not valid");

        // If it isnt a queue update we ignore it
        if !(msg_json["event"] == "queue_updated") {
            continue;
        }

        // Get the basic paths so to say
        let current_item: serde_json::Value = msg_json["data"]["current_item"].clone();
        let media_item: serde_json::Value = current_item["media_item"].clone();
        let metadata: serde_json::Value = media_item["metadata"].clone();

        // If no track is playing clear discord actitivity
        if current_item.to_string() == "null" {
            client.clear_activity().expect("Couldnt clear activity");
            continue;
        }

        // Create the current song struct
        let current_song: Song = Song {
            name: media_item["name"].clone().to_string().replace('"', ""),
            album: media_item["album"]["name"]
                .clone()
                .to_string()
                .replace('"', ""),
            album_image: metadata["images"][0]["path"]
                .clone()
                .to_string()
                .replace('"', ""),
            artist: media_item["artists"][0]["name"]
                .clone()
                .to_string()
                .replace('"', ""),
            preview_url: metadata["preview"].clone().to_string().replace('"', ""),
            artist_image: media_item["artists"][0]["metadata"]["images"][0]["path"]
                .clone()
                .to_string()
                .replace('"', ""),
            started: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("whoops")
                .as_millis() as i64,
            end: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("whoops")
                .as_millis() as i64
                + (&media_item["duration"].as_i64().unwrap() * 1000),
        };

        // The assets of the activity
        let assets: activity::Assets<'_> = activity::Assets::new()
            .small_image(&current_song.artist_image.as_str())
            .small_text(&current_song.artist.as_str())
            .large_image(&current_song.album_image.as_str())
            .large_text(&current_song.album.as_str());

        // The timestamps of the activity
        let timestamps: activity::Timestamps = activity::Timestamps::new()
            .start(current_song.started)
            .end(current_song.end);

        // The buttons of the activity
        let buttons: Vec<activity::Button<'_>> = vec![
            activity::Button::new("What's Mass??", "https://github.com/music-assistant/server"),
            activity::Button::new("Preview song!", &current_song.preview_url),
        ];

        // Construct the final payload
        let payload: activity::Activity<'_> = activity::Activity::new()
            .state(&current_song.artist)
            .details(&current_song.name)
            .assets(assets)
            .buttons(buttons)
            .timestamps(timestamps);

        // Set the activity
        client
            .set_activity(payload)
            .expect("Failure updating status");
    }
}
