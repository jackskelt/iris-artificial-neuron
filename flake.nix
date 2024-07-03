{
  description = "Rust environment";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustPkg = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
        buildDependencies = with pkgs; [
          openssl.dev
          pkg-config
          gcc
          rustPkg
          wayland
          glfw
          pkgsCross.mingwW64.stdenv.cc
        ];

        nativeBuildInputs = with pkgs; [
          cmake
          pkg-config
          clang
          wayland
          libclang
        ];

        LIBCLANG_PATH = pkgs.libclang.lib + "/lib";
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildDependencies;
        CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS = "-L native=${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib";
       
      in with pkgs; {
        devShells = {
          default = mkShell {
            name = "rust-env";
            buildInputs = buildDependencies ++ (with pkgs.xorg; [
                libX11.dev
                libXrandr.dev
                libXinerama.dev
                libXcursor.dev
                libXi.dev
            ]);

            inherit nativeBuildInputs LIBCLANG_PATH LD_LIBRARY_PATH CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS;
          };
        };
      });
}
