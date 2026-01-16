#!/usr/bin/env bash

# Generate cargo-sources.json using nix-shell
nix-shell

# Ensure Flathub remote is added
flatpak remote-add --user --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

# Install SDK and runtime if needed
echo "Ensuring SDK and runtime are installed..."
flatpak install --user -y flathub org.freedesktop.Platform//25.08 org.freedesktop.Sdk//25.08 org.freedesktop.Sdk.Extension.rust-stable//25.08 || true

# Build the flatpak
echo "Building flatpak..."
flatpak-builder --user --install --force-clean build-dir dev.croci.bulletty.yml

echo "Done! Run with: flatpak run dev.croci.bulletty"
