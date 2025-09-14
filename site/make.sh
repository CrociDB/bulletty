# Generating rustdoc using `rustdoc-md`. It uses a nightly version or rust
RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="-Z unstable-options --output-format json" cargo doc --no-deps
rustdoc-md --path ../target/doc/bulletty.json --output docs/docs/bulletty.md

# Generate index page using README.md
cat ./docs/_index.md > ./docs/index.md
tail -n +3 ../README.md >> ./docs/index.md

# Generate contributing page using CONTRIBUTING.md
cat ./docs/_contributing.md > ./docs/contributing.md
tail -n +3 ../CONTRIBUTING.md >> ./docs/contributing.md

cp -R ../img ./docs/img
uv venv --clear
uv tool install mkdocs
uv pip install mkdocs-shadcn
uv run mkdocs build
