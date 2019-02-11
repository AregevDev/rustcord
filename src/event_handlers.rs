use crate::event_wrappers::*;
use crate::join_request::*;
use discord_rpc_sys as sys;

/// A trait for handling the presence events
pub trait EventHandlers {
    /// Called when the presence is displayed
    fn ready(_user: User) {}

    /// Called an error occurred with the presence
    fn errored(_errcode: i32, _message: &str) {}

    /// Called when the presence is shut down or closed
    fn disconnected(_errcode: i32, _message: &str) {}

    /// Called when another user is joining your game
    fn join_game(_secret: &str) {}

    /// Called when a join request is received
    fn spectate_game(_secret: &str) {}

    /// Called when a join request is received
    fn join_request<R: FnOnce(JoinRequestReply)>(_request: User, _respond: R) {}
}

pub(crate) fn wrap<EH: EventHandlers>() -> sys::DiscordEventHandlers {
    sys::DiscordEventHandlers {
        ready: Some(ready_wrapper::<EH>),
        disconnected: Some(disconnected_wrapper::<EH>),
        errored: Some(errored_wrapper::<EH>),
        joinGame: Some(join_game_wrapper::<EH>),
        spectateGame: Some(spectate_game_wrapper::<EH>),
        joinRequest: Some(join_request_wrapper::<EH>),
    }
}
