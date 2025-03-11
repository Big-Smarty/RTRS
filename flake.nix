{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          config.allowUnfree = true;
          inherit system overlays;
        };
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        nativeBuildInputs = with pkgs; [
          rustup
          pkg-config
          sccache
          rustToolchain
        ];
        buildInputs = with pkgs; [
          xorg.libxcb.dev

          shaderc
          glslang
          vulkan-headers
          vulkan-loader
          vulkan-validation-layers

          openssl
          pkg-config
        ];
      in
        with pkgs; {
          devShells.default = mkShell {
            inherit buildInputs nativeBuildInputs;
            SHADERC_LIB_DIR = lib.makeLibraryPath [
              pkgs.shaderc
            ];
            LD_LIBRARY_PATH = lib.makeLibraryPath (
              with pkgs; [
                libxkbcommon
                vulkan-loader
                wayland
              ]
            );
            VULKAN_SDK = "${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d";
            RUSTC_WRAPPER = "${pkgs.sccache}/bin/sccache";
          };
        }
    );
}
