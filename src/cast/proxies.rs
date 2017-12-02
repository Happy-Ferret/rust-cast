/// Proxy classes for the `connection` channel.
pub mod connection {
    #[derive(Serialize, Debug)]
    pub struct ConnectionRequest {
        #[serde(rename = "type")] pub typ: String,
        #[serde(rename = "userAgent")] pub user_agent: String,
    }
}

/// Proxy classes for the `heartbeat` channel.
pub mod heartbeat {
    #[derive(Serialize, Debug)]
    pub struct HeartBeatRequest {
        #[serde(rename = "type")] pub typ: String,
    }
}

/// Proxy classes for the `media` channel.
pub mod media {
    #[derive(Serialize, Debug)]
    pub struct GetStatusRequest {
        #[serde(rename = "requestId")] pub request_id: i32,

        #[serde(rename = "type")] pub typ: String,

        #[serde(rename = "mediaSessionId", skip_serializing_if = "Option::is_none")]
        pub media_session_id: Option<i32>,
    }

    #[derive(Serialize, Debug)]
    pub struct MediaRequest {
        #[serde(rename = "requestId")] pub request_id: i32,

        #[serde(rename = "sessionId")] pub session_id: String,

        #[serde(rename = "type")] pub typ: String,

        pub media: Media,

        #[serde(rename = "currentTime")] pub current_time: f64,

        #[serde(rename = "customData")] pub custom_data: CustomData,

        pub autoplay: bool,
    }

    #[derive(Serialize, Debug)]
    pub struct PlaybackGenericRequest {
        #[serde(rename = "requestId")] pub request_id: i32,

        #[serde(rename = "mediaSessionId")] pub media_session_id: i32,

        #[serde(rename = "type")] pub typ: String,

        #[serde(rename = "customData")] pub custom_data: CustomData,
    }

    #[derive(Serialize, Debug)]
    pub struct PlaybackSeekRequest {
        #[serde(rename = "requestId")] pub request_id: i32,

        #[serde(rename = "mediaSessionId")] pub media_session_id: i32,

        #[serde(rename = "type")] pub typ: String,

        #[serde(rename = "resumeState")] pub resume_state: Option<String>,

        #[serde(rename = "currentTime")] pub current_time: Option<f32>,

        #[serde(rename = "customData")] pub custom_data: CustomData,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Media {
        #[serde(rename = "contentId")] pub content_id: String,
        #[serde(rename = "streamType", default)] pub stream_type: String,
        #[serde(rename = "contentType")] pub content_type: String,
        #[serde(skip_serializing_if = "Option::is_none")] pub duration: Option<f32>,
    }

    #[derive(Serialize, Debug)]
    pub struct CustomData {
        #[serde(skip_serializing)] private: (),
    }

    impl CustomData {
        pub fn new() -> CustomData {
            CustomData { private: () }
        }
    }

    #[derive(Deserialize, Debug)]
    pub struct Status {
        #[serde(rename = "mediaSessionId")] pub media_session_id: i32,
        #[serde(default)] pub media: Option<Media>,
        #[serde(rename = "playbackRate")] pub playback_rate: f32,
        #[serde(rename = "playerState")] pub player_state: String,
        #[serde(rename = "idleReason")] pub idle_reason: Option<String>,
        #[serde(rename = "currentTime")] pub current_time: Option<f32>,
        #[serde(rename = "supportedMediaCommands")] pub supported_media_commands: u8,
    }

    #[derive(Deserialize, Debug)]
    pub struct StatusReply {
        #[serde(rename = "requestId", default)] pub request_id: i32,

        #[serde(rename = "type")] pub typ: String,

        pub status: Vec<Status>,
    }

    #[derive(Deserialize, Debug)]
    pub struct LoadCancelledReply {
        #[serde(rename = "requestId")] pub request_id: i32,

        #[serde(rename = "type")] typ: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct LoadFailedReply {
        #[serde(rename = "requestId")] pub request_id: i32,

        #[serde(rename = "type")] typ: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct InvalidPlayerStateReply {
        #[serde(rename = "requestId")] pub request_id: i32,

        #[serde(rename = "type")] typ: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct InvalidRequestReply {
        #[serde(rename = "requestId")] pub request_id: i32,

        #[serde(rename = "type")] pub typ: String,

        pub reason: Option<String>,
    }
}

/// Proxy classes for the `receiver` channel.
pub mod receiver {
    use std::borrow::Cow;

    #[derive(Serialize, Debug)]
    pub struct AppLaunchRequest {
        #[serde(rename = "requestId")] pub request_id: i32,

        #[serde(rename = "type")] pub typ: String,

        #[serde(rename = "appId")] pub app_id: String,
    }

    #[derive(Serialize, Debug)]
    pub struct AppStopRequest<'a> {
        #[serde(rename = "requestId")] pub request_id: i32,

        #[serde(rename = "type")] pub typ: String,

        #[serde(rename = "sessionId")] pub session_id: Cow<'a, str>,
    }

    #[derive(Serialize, Debug)]
    pub struct GetStatusRequest {
        #[serde(rename = "requestId")] pub request_id: i32,

        #[serde(rename = "type")] pub typ: String,
    }

    #[derive(Serialize, Debug)]
    pub struct SetVolumeRequest {
        #[serde(rename = "requestId")] pub request_id: i32,

        #[serde(rename = "type")] pub typ: String,

        pub volume: Volume,
    }

    #[derive(Deserialize, Debug)]
    pub struct StatusReply {
        #[serde(rename = "requestId")] pub request_id: i32,

        #[serde(rename = "type")] pub typ: String,

        pub status: Status,
    }

    #[derive(Deserialize, Debug)]
    pub struct Status {
        #[serde(default)] pub applications: Vec<Application>,

        #[serde(rename = "isActiveInput", default)] pub is_active_input: bool,

        #[serde(rename = "isStandBy", default)] pub is_stand_by: bool,

        /// Volume parameters of the currently active cast device.
        pub volume: Volume,
    }

    #[derive(Deserialize, Debug)]
    pub struct Application {
        #[serde(rename = "appId")] pub app_id: String,

        #[serde(rename = "sessionId")] pub session_id: String,

        #[serde(rename = "transportId", default)] pub transport_id: String,

        #[serde(default)] pub namespaces: Vec<AppNamespace>,

        #[serde(rename = "displayName")] pub display_name: String,

        #[serde(rename = "statusText")] pub status_text: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct AppNamespace {
        pub name: String,
    }

    /// Structure that describes possible cast device volume options.
    #[derive(Deserialize, Serialize, Debug)]
    pub struct Volume {
        /// Volume level.
        pub level: Option<f32>,
        /// Mute/unmute state.
        pub muted: Option<bool>,
    }

    #[derive(Deserialize, Debug)]
    pub struct LaunchErrorReply {
        #[serde(rename = "requestId")] pub request_id: i32,

        #[serde(rename = "type")] pub typ: String,

        pub reason: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    pub struct InvalidRequestReply {
        #[serde(rename = "requestId")] pub request_id: i32,

        #[serde(rename = "type")] pub typ: String,

        pub reason: Option<String>,
    }
}