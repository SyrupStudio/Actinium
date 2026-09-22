mod utils;

use utils::discord::Presence;

fn main() {
    let presence = Presence::start("1551696179983818914");

    presence.set("Making a game", "");

    println!("Hello, world!");
}
