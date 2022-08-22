# Get all values

cargo run -bin projector -- --config $(pwd)/conf.json

# Get value of 'foo'

cargo run -bin projector -- --config $(pwd)/conf.json foo

# Add key 'foo' with value 'bar'

cargo run -bin projector -- --config $(pwd)/conf.json add foo bar 2>/dev/null

# Get 'foo' after building release (cargo build --release)

./target/release/projector --config $(pwd)/conf.json --pwd $(pwd) foo
