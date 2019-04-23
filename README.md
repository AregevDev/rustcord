# rustcord
A safe wrapper around the Discord Rich Presence API, updated to the latest library version.  
Wrapper version: 0.2.2  
Discord RPC version: 3.4.0

### Example
```rust
use discord_rpc::{DiscordRPC, EventHandlers, RichPresenceBuilder, User};
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

    let presence = RichPresenceBuilder::new()
                .state("Rusting")
                .details("Mining few crystals")
                .large_image_key("rust")
                .large_image_text("Rust")
                .small_image_key("amethyst")
                .small_image_text("Amethyst")
                .build();
                
    discord.update_presence(presence).expect("Could not update presence");
    loop {
        discord.run_callbacks()
    }
}
```

### Documentation
[here](https://docs.rs/rustcord)

### Useful links
[The C API Documentation](https://discordapp.com/developers/docs/rich-presence/how-to)  

### License
Apache-2.0
