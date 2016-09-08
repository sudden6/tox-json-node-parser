This is a parser for Tox bootstrap nodes in the JSON format from <https://nodes.tox.chat/json>.
It outputs the nodes in a format suitable for <https://github.com/zetok/tox-check-online-bootstraps>.

# Usage

Compile it:

1. Install [Rust](http://www.rust-lang.org/)
2. Make with `cargo build`
3. Download the data as nodes.json from <https://nodes.tox.chat/json>
4. Run with `./target/debug/./target/debug/tox-json-node-parser < nodes.json`

# Options

Use `-6` to only output IPv6 nodes or `-4` to only output IPv4 nodes, if no
or invalid arguments are used both are output. Additional arguments will be
ignored.

# License

Licensed under GPLv3+, for details see [LICENSE](/LICENSE). 
