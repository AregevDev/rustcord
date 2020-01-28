# rustcord
A safe wrapper around the Discord Rich Presence API, updated to the latest library version.  
Wrapper version: 0.2.4  
Discord RPC version: 3.4.0

### Example
```rust
use rustcord::{Rustcord, EventHandlers, User, RichPresenceBuilder};
use std::io;

pub struct Handlers;

impl EventHandlers for Handlers {
    fn ready(user: User) {
        println!("User {}#{} logged in...", user.username, user.discriminator);
    }
}

fn main() -> Result<(), io::Error> {
    let discord = Rustcord::init::<Handlers>("APP_ID_HERE", true, None)?;

    let presence = RichPresenceBuilder::new()
        .state("Rusting")
        .details("Mining few crystals")
        .large_image_key("rust")
        .large_image_text("Rust")
        .small_image_key("amethyst")
        .small_image_text("Amethyst")
        .build();

    discord.update_presence(presence)?;
    loop {
        discord.run_callbacks();
    }

    Ok(())
}
```

### Documentation
[docs.rs][docs_rs]

### Useful links
[The C API Documentation][rpc_docs]

### License
Apache-2.0

[docs_rs]:https://docs.rs/rustcord
[rpc_docs]:https://discordapp.com/developers/docs/rich-presence/how-to
