{
  description = "yt-yapper: A Rust-based Discord bot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ ];
        };
        
        nativeBuildInputs = with pkgs; [
          rustToolchain
          pkg-config
          cmake
        ];
        
        # Common dependencies for all platforms
        commonBuildInputs = with pkgs; [
          openssl
          sqlite
          ffmpeg
          yt-dlp
          
          # Development tools
          sqlx-cli
        ];
        
        # Platform-specific dependencies
        platformBuildInputs = if pkgs.stdenv.isDarwin then
          # macOS dependencies
          with pkgs; [
            darwin.apple_sdk.frameworks.AudioToolbox
            darwin.apple_sdk.frameworks.CoreAudio
            darwin.apple_sdk.frameworks.CoreServices
          ]
        else
          # Linux dependencies
          with pkgs; [
            alsa-lib
          ];
          
        # Combine common and platform-specific dependencies
        buildInputs = commonBuildInputs ++ platformBuildInputs;
        
      in
      {
        devShells.default = pkgs.mkShell {
          inherit nativeBuildInputs buildInputs;
          
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          SQLX_OFFLINE = "true";
          
          shellHook = ''
            echo "yt-yapper dev environment"
          '';
        };
      }
    );
} 