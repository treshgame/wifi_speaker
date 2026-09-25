I didn't have a speaker for my windows PC, so I make this project to transfer sound from windows PC to a linux laptop.
- Requirements: the Rust toolchain, PulseAudio on Linux, WASAPI on Windows.
- Server: cargo run --release -- -m server on the computer whose sound you want to share.
- Client: cargo run --release -- -m client -d <server-ip> on each computer that should play it. You can start any number of clients.
- Options: a table of -m, -d, -s, -c and -h with their defaults.
- Notes: start the server first, expect dropouts on a weak connection because UDP has no loss recovery, and always pass -m, since running with no arguments does nothing yet.
