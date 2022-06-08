#!/bin/sh

rm -rf ~/.local/share/kierancli/
git clone https://github.com/kierancrossland/kierancli ~/.local/share/kierancli/
cd ~/.local/share/kierancli
echo you should have cargo installed
sleep 0.5
cargo build --release
mkdir ~/.local/share/kierancli/bin
mv ~/.local/share/kierancli/target/release/kierancli ~/.local/share/kierancli/bin/
echo ""
echo attempting to symlink binary to /usr/local/bin/
echo symlink could FAIL on BSD/Unix due to lack of sudo
sudo ln -sf ~/.local/share/kierancli/bin/kierancli /usr/local/bin/kierancli
echo install succeeded!
echo ""
echo run kierancli
