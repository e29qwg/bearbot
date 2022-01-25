# BearBot

## About
**BearBot** is a host-it-yourself Discord bot written in rust language
based on [serenity](https://github.com/serenity-rs/serenity)
and [parrot](https://github.com/aquelemiguel/parrot).
This project aims to provide some simple commands mainly used in (our) everyday Discord's guild.

## Deployment
### Dependencies
Some tools and libraries are required for building the project.
```sh
apt install build-essential autoconf automake libtool pkg-config ffmpeg
pip install -U yt-dlp
```

### Building and Running
```sh
cargo build --release
cargo run --release
```

## Licensing
This project is under [MIT](./LICENSE) license.

