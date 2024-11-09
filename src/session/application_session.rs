use windows::{
    Win32::Foundation::BOOL,
    Win32::Media::Audio::{ISimpleAudioVolume, Endpoints::IAudioEndpointVolume},
    core::GUID,
};
use std::process::exit;
use super::Session;

pub struct ApplicationSession {
    simple_audio_volume: ISimpleAudioVolume,
    name: String,
    guid: GUID,
}

impl ApplicationSession {
    pub fn new(simple_audio_volume: ISimpleAudioVolume, name: String) -> Self {
        let guid = GUID::new().unwrap_or_else(|err| {
            eprintln!("ERROR: Couldn't generate GUID {err}");
            exit(1);
        });
        
        Self {
            simple_audio_volume,
            name,
            guid,
        }
    }
}

impl Session for ApplicationSession {
    unsafe fn get_audio_endpoint_volume(&self) -> Option<IAudioEndpointVolume> {
        None
    }

    unsafe fn get_name(&self) -> String {
        self.name.clone()
    }

    unsafe fn get_volume(&self) -> f32 {
        self.simple_audio_volume
            .GetMasterVolume()
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't get volume {err}");
                0.0
            })
    }

    unsafe fn set_volume(&self, vol: f32) {
        self.simple_audio_volume
            .SetMasterVolume(vol, &self.guid)
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't set volume: {err}");
            });
    }

    unsafe fn get_mute(&self) -> bool {
        self.simple_audio_volume
            .GetMute()
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't get mute {err}");
                BOOL(0)
            })
            .as_bool()
    }

    unsafe fn set_mute(&self, mute: bool) {
        self.simple_audio_volume
            .SetMute(mute, &self.guid)
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't set mute: {err}");
            });
    }
} 