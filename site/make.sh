cat ./docs/_index.md > ./docs/index.md
tail -n +3 ../README.md >> ./docs/index.md
cp -R ../img ./docs/img
uv run mkdocs build
