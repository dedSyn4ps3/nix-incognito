####################################################################################################################
#                                                                                                                  # 
#                                                                                                                  #
#            ███╗   ██╗██╗██╗  ██╗    ██╗███╗   ██╗ ██████╗ ██████╗  ██████╗ ███╗   ██╗██╗████████╗ ██████╗        #
#            ████╗  ██║██║╚██╗██╔╝    ██║████╗  ██║██╔════╝██╔═══██╗██╔════╝ ████╗  ██║██║╚══██╔══╝██╔═══██╗       #
#            ██╔██╗ ██║██║ ╚███╔╝     ██║██╔██╗ ██║██║     ██║   ██║██║  ███╗██╔██╗ ██║██║   ██║   ██║   ██║       #
#            ██║╚██╗██║██║ ██╔██╗     ██║██║╚██╗██║██║     ██║   ██║██║   ██║██║╚██╗██║██║   ██║   ██║   ██║       #
#            ██║ ╚████║██║██╔╝ ██╗    ██║██║ ╚████║╚██████╗╚██████╔╝╚██████╔╝██║ ╚████║██║   ██║   ╚██████╔╝       #
#            ╚═╝  ╚═══╝╚═╝╚═╝  ╚═╝    ╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═══╝╚═╝   ╚═╝    ╚═════╝        #
#                                                                                                                  #
#                  ===============================================================================                 #
#                  ||  🪪  Created by: dedsyn4ps3       ✨ Inspiration from: Kali (of course!)  ||                 #
#                  ===============================================================================                 #
#                                                                                                                  #
####################################################################################################################

{
  description = "Flake for Nix-Incognito";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.2305.491812.tar.gz";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      # Systems supported (more can be added)
      allSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f {
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            # Provides Nixpkgs with a rust-bin attribute for building Rust toolchains
            rust-overlay.overlays.default
            # Uses the rust-bin attribute to select a Rust toolchain
            self.overlays.default
          ];
        };
      });
    in
    {
      overlays.default = final: prev: {
        # The Rust toolchain used for the package build
        rustToolchain = final.rust-bin.nightly.latest.default;
      };

      packages = forAllSystems ({ pkgs }: {
        default =
          let
            rustPlatform = pkgs.makeRustPlatform {
              cargo = pkgs.rustToolchain;
              rustc = pkgs.rustToolchain;
            };
          in
          # Build the package for each system
          rustPlatform.buildRustPackage {
              name = "nix-incognito";
              version = "v0.2.1";

              src = builtins.fetchGit {
                url = "https://github.com/dedsyn4ps3/nix-incognito.git";
                ref = "main";
              };

              cargoSha256 = "sha256-S9bhnxOqklCFMktwEk70r7bRm/o5nG3DP7drCsXZz8E=";

              preInstall = ''
                mkdir -p $out/share/themes
                mkdir -p $out/share/icons
                mkdir -p $out/share/backgrounds/incognito
                mkdir $out/bin
              '';

              installPhase = ''
                runHook preInstall

                cp -r target/x86_64-unknown-linux-gnu/release/nix-incognito $out/bin/
                cp -r backgrounds/* $out/share/backgrounds/incognito/
                cp -r themes/* $out/share/themes/
                cp -r icons/* $out/share/icons/
              '';

              meta = with pkgs.lib; {
                description = "A nifty utility that makes your desktop look like Windows";
                homepage = "https://github.com/dedsyn4ps3/nix-incognito";
                maintainers = with maintainers; [ dedsyn4ps3 ];
                platforms = platforms.unix;
                license = licenses.gpl3Plus;
              };  
          };
      });
    };
}
