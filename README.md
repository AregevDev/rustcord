# discord_rpc
A safe wrapper around the Discord Rich Presence C API, updated to the latest library version.  
Wrapper version: 0.1.0
Discord RPC version: 3.4.0

### Example
```rust
use discord_rpc::{DiscordRPC, EventHandlers, RichPresence, User};
use std::time::SystemTime;

struct Handlers;

impl EventHandlers for Handlers {
    fn ready(_user: User) {
        println!("Welcome {}#{}", _user.username, _user.discriminator);
    }
}

fn main() {
    let discord = DiscordRPC::init::<Handlers>("544523578855391241", true, None)
        .expect("Could no initialize RPC");

    let presence = RichPresence {
        state: Some("Mining few crystals".into()),
        start_time: Some(SystemTime::now()),
        large_image_key: Some("rust".into()),
        large_image_text: Some("Rust".into()),
        small_image_key: Some("amethyst".into()),
        small_image_text: Some("Amethyst".into()),
        ..Default::default()
    };

    discord.update_presence(presence).expect("Could not update presence");
    loop {
        discord.run_callbacks()
    }
}
```

### Documentation
[here](https://docs.rs/discord_rpc)

### Useful links
[The C API Documentation](https://discordapp.com/developers/docs/rich-presence/how-to)  

### License
Apache-2.0