use napi::{
    threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
    JsFunction
};
use napi_derive::napi;
use tokio::runtime::Runtime;
use std::{thread, time::Duration};
use crate::MediaInfo;

/// The type of event that occurred
#[napi]
pub enum EventType {
    MediaChange,
    PlaybackChange,
    VolumeChange,
    MuteChange,
}

/// The data for an event
#[napi(object)]
pub struct EventData {
    pub event_type: EventType,
    pub media_info: Option<MediaInfo>,
    pub volume: Option<f64>,
    pub muted: Option<bool>,
}

/// The data for a subscription
#[napi(object)]
pub struct Subscription {
    #[napi(ts_type = "() => void")]
    pub stop: JsFunction,
}

/// Subscribes to events
#[napi]
pub fn subscribe_to_events(
    callback: ThreadsafeFunction<EventData, ErrorStrategy::Fatal>,
    #[napi(ts_arg_type = "() => void")] stop_callback: JsFunction,
) -> Subscription {
    thread::spawn(move || {
        let mut last_media_info: Option<MediaInfo> = None;
        let mut last_volume: Option<f64> = None;
        let mut last_mute: Option<bool> = None;
        
        let rt = Runtime::new().expect("Failed to create Tokio runtime");

        loop {
            // Check for media changes
            if let Some(current_info) = crate::get_media_info() {
                match &last_media_info {
                    Some(last_info) => {
                        if last_info.title != current_info.title 
                            || last_info.artist != current_info.artist 
                            || last_info.album != current_info.album {
                            let _ = callback.call(
                                EventData {
                                    event_type: EventType::MediaChange,
                                    media_info: Some(current_info.clone()),
                                    volume: None,
                                    muted: None,
                                },
                                ThreadsafeFunctionCallMode::NonBlocking
                            );
                        }
                        if last_info.playback_status != current_info.playback_status {
                            let _ = callback.call(
                                EventData {
                                    event_type: EventType::PlaybackChange,
                                    media_info: Some(current_info.clone()),
                                    volume: None,
                                    muted: None,
                                },
                                ThreadsafeFunctionCallMode::NonBlocking
                            );
                        }
                    }
                    None => {
                        let _ = callback.call(
                            EventData {
                                event_type: EventType::MediaChange,
                                media_info: Some(current_info.clone()),
                                volume: None,
                                muted: None,
                            },
                            ThreadsafeFunctionCallMode::NonBlocking
                        );
                    }
                }
                last_media_info = Some(current_info);
            }

            // Check for volume changes
            if let Some(current_volume) = rt.block_on(crate::get_system_volume()) {
                if last_volume != Some(current_volume) {
                    let _ = callback.call(
                        EventData {
                            event_type: EventType::VolumeChange,
                            media_info: None,
                            volume: Some(current_volume),
                            muted: None,
                        },
                        ThreadsafeFunctionCallMode::NonBlocking
                    );
                    last_volume = Some(current_volume);
                }
            }

            if let Some(current_mute) = rt.block_on(crate::get_system_mute()) {
                if last_mute != Some(current_mute) {
                    let _ = callback.call(
                        EventData {
                            event_type: EventType::MuteChange,
                            media_info: None,
                            volume: None,
                            muted: Some(current_mute),
                        },
                        ThreadsafeFunctionCallMode::NonBlocking
                    );
                    last_mute = Some(current_mute);
                }
            }

            thread::sleep(Duration::from_millis(500));
        }
    });

    Subscription {
        stop: stop_callback
    }
}
  

/// Custom error codes that can be thrown
#[napi(string_enum)]
pub enum ErrorStatus {
    #[napi(value = "ERR_STATUS_CANTDSMTGH")]
    CantDoSomething,
    #[napi(value = "ERR_STATUS_INVALID")]
    InvalidCondition,
}