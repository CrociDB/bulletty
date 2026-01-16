{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    (python3.withPackages (ps: with ps; [
      aiohttp
      tomlkit
    ]))
    wget
  ];

  shellHook = ''
    # Download flatpak-cargo-generator if not present
    if [ ! -f "flatpak-cargo-generator.py" ]; then
      echo "Downloading flatpak-cargo-generator.py..."
      wget -q https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py
    fi

    # Generate cargo sources
    echo "Generating cargo-sources.json..."
    python3 flatpak-cargo-generator.py ../../Cargo.lock -o cargo-sources.json

    echo "cargo-sources.json generated. Exiting nix-shell..."
    echo "Now run: ./build-flatpak.sh"
    exit 0
  '';
}
