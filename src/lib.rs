#![deny(clippy::all)]

mod audio_controller;
mod session;

pub use audio_controller::{AudioController, CoinitMode};
pub use session::{Session, ApplicationSession, EndPointSession};

use napi_derive::napi;
use windows::{
    Media::Control::{
        GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionManager,
        GlobalSystemMediaTransportControlsSessionPlaybackStatus,
    },
    Storage::Streams::{Buffer, IRandomAccessStreamReference, InputStreamOptions, IBuffer},
    core::{Interface, GUID},
    Win32::{
        Media::Audio::{
            eRender, eConsole, IMMDeviceEnumerator, MMDeviceEnumerator,
            Endpoints::IAudioEndpointVolume,
        },
        System::Com::{
            CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED,
        }
    },
};

/// Information about the currently playing media
#[napi(object)]
pub struct MediaInfo {
    /// The title of the media
    pub title: Option<String>,
    /// The artist performing the media
    pub artist: Option<String>,
    /// The album containing the media
    pub album: Option<String>,
    /// The artist of the album
    pub album_artist: Option<String>,
    /// The current playback status (playing, paused, etc)
    pub playback_status: String,
    /// Whether the media has an associated thumbnail image
    pub has_thumbnail: bool,
}

fn request_session_manager() -> Option<GlobalSystemMediaTransportControlsSessionManager> {
    let async_op = GlobalSystemMediaTransportControlsSessionManager::RequestAsync().ok()?;
    async_op.get().ok()
}

/// Gets information about the currently playing media
#[napi]
pub fn get_media_info() -> Option<MediaInfo> {
    let manager = request_session_manager()?;
    let session = manager.GetCurrentSession().ok()?;
    
    let async_props = session.TryGetMediaPropertiesAsync().ok()?;
    let media_props = async_props.get().ok()?;

    let playback_info = session.GetPlaybackInfo().ok()?;
    let status = playback_info.PlaybackStatus().ok()?;
    let status_str = match status {
        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Closed => "closed",
        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Opened => "opened",
        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Changing => "changing",
        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Stopped => "stopped",
        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Playing => "playing",
        GlobalSystemMediaTransportControlsSessionPlaybackStatus::Paused => "paused",
        _ => "unknown",
    };

    let thumbnail: Option<IRandomAccessStreamReference> = media_props.Thumbnail().ok();

    Some(MediaInfo {
        title: media_props.Title().ok().map(|s| s.to_string()),
        artist: media_props.Artist().ok().map(|s| s.to_string()),
        album: media_props.AlbumTitle().ok().map(|s| s.to_string()),
        album_artist: media_props.AlbumArtist().ok().map(|s| s.to_string()),
        playback_status: status_str.to_string(),
        has_thumbnail: thumbnail.is_some(),
    })
}

/// Gets the thumbnail image for the currently playing media as a byte array
#[napi]
pub fn get_thumbnail() -> Option<Vec<u8>> {
    let manager = request_session_manager()?;
    let session = manager.GetCurrentSession().ok()?;
    
    let async_props = session.TryGetMediaPropertiesAsync().ok()?;
    let media_props = async_props.get().ok()?;

    let thumbnail = media_props.Thumbnail().ok()?;
    let async_stream = thumbnail.OpenReadAsync().ok()?;
    let stream = async_stream.get().ok()?;
    
    let size = stream.Size().ok()? as u32;
    let buffer = Buffer::Create(size).ok()?;
    
    let input_stream = stream.GetInputStreamAt(0).ok()?;
    let async_read = input_stream.ReadAsync(
        &buffer,
        size,
        InputStreamOptions::default(),
    ).ok()?;
    
    let _bytes_read = async_read.get().ok()?;
    
    // Get bytes from buffer
    let ibuffer: IBuffer = buffer.cast().ok()?;
    let len = ibuffer.Length().ok()? as usize;
    let mut vec = vec![0u8; len];
    
    // Create a new DataReader to read from the buffer
    let reader = windows::Storage::Streams::DataReader::FromBuffer(&ibuffer).ok()?;
    reader.ReadBytes(&mut vec).ok()?;
    
    Some(vec)
}

/// Attempts to play the current media
/// @returns Whether the play command was successful
#[napi]
pub async fn play() -> bool {
    if let Some(session) = get_current_session() {
        if let Ok(async_op) = session.TryPlayAsync() {
            async_op.get().is_ok()
        } else {
            false
        }
    } else {
        false
    }
}

/// Attempts to pause the current media
/// @returns Whether the pause command was successful
#[napi]
pub async fn pause() -> bool {
    if let Some(session) = get_current_session() {
        if let Ok(async_op) = session.TryPauseAsync() {
            async_op.get().is_ok()
        } else {
            false
        }
    } else {
        false
    }
}

/// Attempts to skip to the next track
/// @returns Whether the next command was successful
#[napi]
pub async fn next() -> bool {
    if let Some(session) = get_current_session() {
        if let Ok(async_op) = session.TrySkipNextAsync() {
            async_op.get().is_ok()
        } else {
            false
        }
    } else {
        false
    }
}

/// Attempts to go back to the previous track
/// @returns Whether the previous command was successful
#[napi]
pub async fn previous() -> bool {
    if let Some(session) = get_current_session() {
        if let Ok(async_op) = session.TrySkipPreviousAsync() {
            async_op.get().is_ok()
        } else {
            false
        }
    } else {
        false
    }
}

/// Attempts to stop playback of the current media
/// @returns Whether the stop command was successful
#[napi]
pub async fn stop() -> bool {
    if let Some(session) = get_current_session() {
        if let Ok(async_op) = session.TryStopAsync() {
            async_op.get().is_ok()
        } else {
            false
        }
    } else {
        false
    }
}

fn get_current_session() -> Option<GlobalSystemMediaTransportControlsSession> {
    let manager = request_session_manager()?;
    manager.GetCurrentSession().ok()
}

/// Sets the system volume level
/// @param level - Volume level between 0.0 and 1.0
/// @returns Whether setting the volume was successful
#[napi]
pub async fn set_system_volume(level: f64) -> bool {
    // Validate input range
    if level < 0.0 || level > 1.0 {
        return false;
    }

    unsafe {
        // Skip CoInitializeEx if already initialized
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        let device_enumerator: IMMDeviceEnumerator = match CoCreateInstance(
            &MMDeviceEnumerator,
            None,
            CLSCTX_ALL
        ) {
            Ok(enumerator) => enumerator,
            Err(_) => return false,
        };

        let device = match device_enumerator.GetDefaultAudioEndpoint(eRender, eConsole) {
            Ok(device) => device,
            Err(_) => return false,
        };

        let volume: IAudioEndpointVolume = match device.Activate(CLSCTX_ALL, None) {
            Ok(volume) => volume,
            Err(_) => return false,
        };

        volume.SetMasterVolumeLevelScalar(level as f32, &GUID::zeroed()).is_ok()
    }
}

/// Gets the current system volume level
/// @returns Volume level between 0.0 and 1.0, or null if unable to get volume
#[napi]
pub async fn get_system_volume() -> Option<f64> {
    unsafe {
        // Skip CoInitializeEx if already initialized
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        let device_enumerator: IMMDeviceEnumerator = CoCreateInstance(
            &MMDeviceEnumerator,
            None,
            CLSCTX_ALL
        ).ok()?;

        let device = device_enumerator.GetDefaultAudioEndpoint(eRender, eConsole).ok()?;
        let volume: IAudioEndpointVolume = device.Activate(CLSCTX_ALL, None).ok()?;
        
        volume.GetMasterVolumeLevelScalar().ok().map(|v| v as f64)
    }
}

/// Sets the system mute state
/// @param mute - Whether to mute (true) or unmute (false) the system audio
/// @returns Whether setting the mute state was successful
#[napi]
pub async fn set_system_mute(mute: bool) -> bool {
    unsafe {
        // Skip CoInitializeEx if already initialized
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        let device_enumerator: IMMDeviceEnumerator = match CoCreateInstance(
            &MMDeviceEnumerator,
            None,
            CLSCTX_ALL
        ) {
            Ok(enumerator) => enumerator,
            Err(_) => return false,
        };

        let device = match device_enumerator.GetDefaultAudioEndpoint(eRender, eConsole) {
            Ok(device) => device,
            Err(_) => return false,
        };

        let volume: IAudioEndpointVolume = match device.Activate(CLSCTX_ALL, None) {
            Ok(volume) => volume,
            Err(_) => return false,
        };

        volume.SetMute(mute, &GUID::zeroed()).is_ok()
    }
}

/// Gets the current system mute state
/// @returns Whether the system is muted (true) or not (false), or null if unable to get state
#[napi]
pub async fn get_system_mute() -> Option<bool> {
    unsafe {
        // Skip CoInitializeEx if already initialized
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        let device_enumerator: IMMDeviceEnumerator = CoCreateInstance(
            &MMDeviceEnumerator,
            None,
            CLSCTX_ALL
        ).ok()?;

        let device = device_enumerator.GetDefaultAudioEndpoint(eRender, eConsole).ok()?;
        let volume: IAudioEndpointVolume = device.Activate(CLSCTX_ALL, None).ok()?;
        
        volume.GetMute().ok().map(|m| m.as_bool())
    }
}
