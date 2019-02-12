use libc;
use rustcord_sys as sys;
use std::ffi::{CString, NulError};
use std::ptr;
use std::time::*;

/// Defines the data displayed on the rich presence screen on a user's profile.
/// Due to the C API limits, this data is limited to a maximum amount of bytes will be listed below
#[derive(Default, Clone, Hash, PartialEq, Debug)]
pub struct RichPresence {
    /// The user's current party status. Maximum of 128 bytes.
    pub state: Option<String>,

    /// What the player is currently doing. Maximum of 128 bytes.
    pub details: Option<String>,

    /// Time of game start. Including will show time as "elapsed".
    pub start_time: Option<SystemTime>,

    /// Time of game end. Including will show time as "remaining".
    pub end_time: Option<SystemTime>,

    /// Name of the uploaded image for the large profile artwork. Maximum of 32 bytes.
    pub large_image_key: Option<String>,

    /// Tooltip for the large image. Maximum of 128 bytes.
    pub large_image_text: Option<String>,

    /// Name of the uploaded image for the large profile artwork. Maximum of 32 bytes.
    pub small_image_key: Option<String>,

    /// Tooltip for the large image. Maximum of 128 bytes.
    pub small_image_text: Option<String>,

    /// ID of the player's party, lobby, or group. Maximum of 128 bytes.
    pub party_id: Option<String>,

    /// Current size of the player's party, lobby, or group.
    pub party_size: Option<u32>,

    /// Maximum size of the player's party, lobby, or group.
    pub party_max: Option<u32>,

    /// Unique hashed string for Spectate button. Maximum of 128 bytes.
    pub spectate_secret: Option<String>,

    /// Unique hashed string for chat invitations and Ask to Join. Maximum of 128 bytes.
    pub join_secret: Option<String>,
}

impl RichPresence {
    pub(crate) fn wrap(self) -> Result<sys::DiscordRichPresence, NulError> {
        Ok(sys::DiscordRichPresence {
            state: match self.state {
                None => ptr::null(),
                Some(state) => CString::new(state.clone())?.into_raw(),
            },
            details: match self.details {
                None => ptr::null(),
                Some(details) => CString::new(details)?.into_raw(),
            },
            startTimestamp: match self.start_time {
                None => 0,
                Some(time) => time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            },
            endTimestamp: match self.end_time {
                None => 0,
                Some(time) => time.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            },
            largeImageKey: match self.large_image_key {
                None => ptr::null(),
                Some(key) => CString::new(key)?.into_raw(),
            },
            largeImageText: match self.large_image_text {
                None => ptr::null(),
                Some(text) => CString::new(text)?.into_raw(),
            },
            smallImageKey: match self.small_image_key {
                None => ptr::null(),
                Some(key) => CString::new(key)?.into_raw(),
            },
            smallImageText: match self.small_image_text {
                None => ptr::null(),
                Some(text) => CString::new(text)?.into_raw(),
            },
            partyId: match self.party_id {
                None => ptr::null(),
                Some(id) => CString::new(id)?.into_raw(),
            },
            partySize: match self.party_size {
                None => 0,
                Some(size) => size as libc::c_int,
            },
            partyMax: match self.party_max {
                None => 0,
                Some(max) => max as libc::c_int,
            },
            matchSecret: ptr::null(), // deprecated
            joinSecret: match self.join_secret {
                None => ptr::null(),
                Some(secret) => CString::new(secret)?.into_raw(),
            },
            spectateSecret: match self.spectate_secret {
                None => ptr::null(),
                Some(secret) => CString::new(secret)?.into_raw(),
            },
            instance: 0, // deprecated
        })
    }
}

/// A Builder struct for RichPresence which gives more ergonomic API
#[derive(Clone, Debug)]
pub struct RichPresenceBuilder {
    inner: RichPresence,
}

impl RichPresenceBuilder {
    pub fn new() -> Self {
        RichPresenceBuilder {
            inner: RichPresence::default(),
        }
    }

    pub fn state(mut self, state: &str) -> Self {
        self.inner.state = Some(state.to_owned());
        self
    }

    pub fn details(mut self, details: &str) -> Self {
        self.inner.details = Some(details.to_owned());
        self
    }

    pub fn start_time(mut self, start_time: SystemTime) -> Self {
        self.inner.start_time = Some(start_time);
        self
    }

    pub fn end_time(mut self, end_time: SystemTime) -> Self {
        self.inner.end_time = Some(end_time);
        self
    }

    pub fn large_image_key(mut self, large_image_key: &str) -> Self {
        self.inner.large_image_key = Some(large_image_key.to_owned());
        self
    }

    pub fn large_image_text(mut self, large_image_text: &str) -> Self {
        self.inner.large_image_text = Some(large_image_text.to_owned());
        self
    }

    pub fn small_image_key(mut self, small_image_key: &str) -> Self {
        self.inner.small_image_key = Some(small_image_key.to_owned());
        self
    }

    pub fn small_image_text(mut self, small_image_text: &str) -> Self {
        self.inner.small_image_text = Some(small_image_text.to_owned());
        self
    }

    pub fn party_id(mut self, party_id: &str) -> Self {
        self.inner.party_id = Some(party_id.to_owned());
        self
    }

    pub fn party_size(mut self, party_size: u32) -> Self {
        self.inner.party_size = Some(party_size);
        self
    }

    pub fn party_max(mut self, party_max: u32) -> Self {
        self.inner.party_max = Some(party_max);
        self
    }

    pub fn spectate_secret(mut self, spectate_secret: &str) -> Self {
        self.inner.spectate_secret = Some(spectate_secret.to_owned());
        self
    }

    pub fn join_secret(mut self, join_secret: &str) -> Self {
        self.inner.join_secret = Some(join_secret.to_owned());
        self
    }

    pub fn build(mut self) -> RichPresence {
        self.inner
    }
}
