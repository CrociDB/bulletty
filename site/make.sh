RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="-Z unstable-options --output-format json" cargo doc --no-deps
rustdoc-md --path ../target/doc/bulletty.json --output docs/docs/bulletty.md

cat ./docs/_index.md > ./docs/index.md
tail -n +3 ../README.md >> ./docs/index.md
cp -R ../img ./docs/img
uv run mkdocs build
