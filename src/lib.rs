//! A safe wrapper around the Discord RichPresence API
//! ### Example
//! ```rust
//! use discord_rpc::{DiscordRPC, EventHandlers, RichPresenceBuilder, User};
//! use std::time::SystemTime;
//!
//! struct Handlers;
//
//! impl EventHandlers for Handlers {
//!     fn ready(_user: User) {
//!         println!("Welcome {}#{}", _user.username, _user.discriminator);
//!     }
//! }
//!
//! fn main() {
//!     let discord = DiscordRPC::init::<Handlers>("544523578855391241", true, None)
//!         .expect("Could no initialize RPC");
//!
//!     let presence = RichPresenceBuilder::new()
//!         .state("Rusting")
//!         .details("Mining few crystals")
//!         .large_image_key("rust")
//!         .large_image_text("Rust")
//!         .small_image_key("amethyst")
//!         .small_image_text("Amethyst")
//!         .build();
//!
//!     discord.update_presence(presence).expect("Could not update presence");
//!     loop {
//!         discord.run_callbacks()
//!     }
//! }
//! ```

mod event_handlers;
mod event_wrappers;
mod join_request;
mod presence;

pub use crate::event_handlers::EventHandlers;
pub use crate::join_request::{JoinRequestReply, User};
pub use crate::presence::{RichPresence, RichPresenceBuilder};
use rustcord_sys as sys;

use std::ffi::{CString, NulError};
use std::ptr;

/// The main struct of the API providing the RPC methods
pub struct Rustcord;

impl Rustcord {
    /// Initializes the Discord Rich Presence API.
    pub fn init<EH: EventHandlers>(app_id: &str, auto_register: bool, steam_id: Option<&str>) -> Result<Rustcord, NulError> {
        let mut sys_handlers = event_handlers::wrap::<EH>();
        unsafe {
            sys::Discord_Initialize(
                CString::new(app_id)?.into_raw(),
                &mut sys_handlers,
                auto_register as libc::c_int,
                match steam_id {
                    None => ptr::null(),
                    Some(id) => CString::new(id)?.into_raw(),
                },
            );
        }

        Ok(Rustcord)
    }

    /// Updates the callback handlers.
    pub fn update_handlers<EH: EventHandlers>(&self) {
        let mut sys_handlers = event_handlers::wrap::<EH>();
        unsafe {
            sys::Discord_UpdateHandlers(&mut sys_handlers);
        }
    }

    /// Updates the rich presence screen.
    pub fn update_presence(&self, presence: RichPresence) -> Result<(), NulError> {
        let sys_presence = presence.wrap()?;
        unsafe {
            sys::Discord_UpdatePresence(&sys_presence);
        }

        Ok(())
    }

    /// Clears the rich present screen.DiscordRPC
    pub fn clear_presence(&self) {
        unsafe {
            sys::Discord_ClearPresence();
        }
    }

    /// Invokes any pending callbacks from Discord on the calling thread.
    /// This function is allegedly thread safe.
    pub fn run_callbacks(&self) {
        unsafe {
            sys::Discord_RunCallbacks();
        }
    }
}

impl Drop for Rustcord {
    fn drop(&mut self) {
        unsafe {
            sys::Discord_Shutdown();
        }
    }
}
