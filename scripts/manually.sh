echo "Build x86_64-unknown-linux-gnu"
cross build --release --target x86_64-unknown-linux-gnu

echo "Build x86_64-pc-windows-gnu"
cross build --release --target x86_64-pc-windows-gnu