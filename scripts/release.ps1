cargo build --release --target-dir ./dist
Move-Item ./dist/release/iser ./dist
Remove-Item ./dist -Exclude iser, iser.* -Force -Recurse