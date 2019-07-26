# Fuite

A game where you have to escape... but will everybody make it?

[Ludum Dare 43](https://ldjam.com/events/ludum-dare/43):
Sacrifices must be made.

Link to the game on Ludum Dare website: https://ldjam.com/events/ludum-dare/43/fuite-escape

**Controls**:
 - Click on Hums to control them:
   - Left click: go to your left.
   - Right right: go to your other left.
 - R: restart the current level.

For players stuck with a touch-pad:
 - Click + Space = right click.


*No sound... Sorry :/*


## Screenshots

![Little house](screenshots/nice-and-easy.png "Hi there!")

![Level 2](screenshots/hard-core.png "Sacrifices must be made")



## How to run the game?
You can download a bundled version of the game on
[GitHub](https://github.com/totorigolo/fuite-rs/releases).

**Note**: Doesn't seem to work on Wayland, use X instead.

**Note**: Rust 1.36.0+ introduce a ["acceptable breakage"](https://github.com/rust-lang/rust/issues/60958), so the game won't compile with newer versions of the compiler. Amethyst fixed the issue, but in 0.11, and I don't want to migrate to Rendy.

## How to build the game?
 - Install _rustup_ ([rustup](https://rustup.rs/) is the easy way):
 - You need the Rust **1.35.0** version: `rustup install 1.35.0`.
 - Go check the [Amethyst README](https://github.com/amethyst/amethyst).
 - `git clone https://github.com/totorigolo/fuite.git`
 - `cargo +1.35.0 run --release`


## License
This game is license under the [MIT License](LICENSE).
