//! A safe wrapper around the Discord RPC C API

mod event_handlers;
mod event_wrappers;
mod join_request;
mod presence;

pub use crate::event_handlers::EventHandlers;
pub use crate::join_request::{JoinRequestReply, User};
pub use crate::presence::RichPresence;
use rustcord_sys as sys;

use std::ffi::{CString, NulError};
use std::ptr;

/// The main struct of the API providing the RPC methods
pub struct DiscordRPC;

impl DiscordRPC {
    /// Initializes the Discord Rich Presence API.
    pub fn init<EH: EventHandlers>(
        app_id: &str,
        auto_register: bool,
        steam_id: Option<&str>,
    ) -> Result<DiscordRPC, NulError> {
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

        Ok(DiscordRPC)
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

    /// Clears the rich present screen.
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

impl Drop for DiscordRPC {
    fn drop(&mut self) {
        unsafe {
            sys::Discord_Shutdown();
        }
    }
}
