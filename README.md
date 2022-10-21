# Messaging App


## Build

#### Windows

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