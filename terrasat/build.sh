# builds both the client and the server executables
cargo build --release
cp target/release/terrasat builds/
cp target/release/server builds/
chmod +x builds/terrasat
chmod +x builds/server
cp cli-view/src/config.toml builds/cli-config.toml
cp server/src/config.toml builds/server-config.toml
echo "Ok: Server and CLI are built and located at builds/"