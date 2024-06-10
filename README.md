# Messaging App
Small mess around with rust. making a chat app with a backend API to handle the messaging

#### Windows compilation

https://github.com/Jake-Shadle/xwin
https://bevy-cheatbook.github.io/setup/cross/linux-windows.html#first-time-setup-msvc

cargo install xwin

xwin --accept-license splat --output /opt/xwin

sudo apt-get install lld 

cat << EOF >> ~/.cargo/config.toml
[target.x86_64-pc-windows-msvc]
linker = "lld"
rustflags = [
  "-Lnative=/opt/xwin/crt/lib/x86_64",
  "-Lnative=/opt/xwin/sdk/lib/um/x86_64",
  "-Lnative=/opt/xwin/sdk/lib/ucrt/x86_64"
]
EOF

## creating dynamodb table
```bash
 aws dynamodb delete-item --region eu-west-1 --endpoint-url http://localhost:8000 --table-name test --key '{"name": {"S": "hn"},"timestamp":{"S":"2024-06-04 13:10:32.929340763 UTC"}}'
```
