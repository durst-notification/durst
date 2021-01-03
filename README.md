# durst

Durst is a new notification daemon to act as the replacement of [dunst](https://github.com/dunst-project/dunst) on wayland based systems.

It's written from scratch in Rust.

## How to build and run durst
```
git clone https://github.com/durst-notification/durst.git
cd durst
mkdir -p "$HOME/.config/durst"
cp config.yml "$HOME/.config/durst/config.yml"
cargo run
```
