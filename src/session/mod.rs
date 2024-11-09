mod endpoint_session;
mod application_session;

pub use endpoint_session::EndPointSession;
pub use application_session::ApplicationSession;

use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;

pub trait Session {
    unsafe fn get_audio_endpoint_volume(&self) -> Option<IAudioEndpointVolume>;
    unsafe fn get_name(&self) -> String;
    unsafe fn get_volume(&self) -> f32;
    unsafe fn set_volume(&self, vol: f32);
    unsafe fn get_mute(&self) -> bool;
    unsafe fn set_mute(&self, mute: bool);
} 