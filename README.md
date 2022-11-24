# how to setup
- install NodeJS (https://nodejs.org/en/download/)
- install Rust (https://www.rust-lang.org/tools/install)
- install Yarn (https://classic.yarnpkg.com/lang/en/docs/install)

# how to run
run `cargo run` in the base folder

# deployment note
frustrating out of memory sigterm occurs if less than 1GB of RAM

# sqlite notes
sqlite will actually ignore a lot of things that one might initially assume it shouldn't, examples include:
- `VARCHAR(n)`: ignored, it's actually just `TEXT` (use `CHECK` instead)
- `REFERENCES`: ignored, use `FOREIGN KEY` instead

for some reason using the `bundled` feature for `libsqlite3-sys` makes it ignore `PRAGMA legacy_alter_table = on;`
