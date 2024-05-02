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

with import <nixpkgs>
{
  overlays = [
    (import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
  ];
};

let
  rustPlatform = makeRustPlatform {
    cargo = rust-bin.nightly.latest.minimal;
    rustc = rust-bin.nightly.latest.minimal;
  };
in

rustPlatform.buildRustPackage {
  pname = "nix-incognito";
  version = "v0.2.0";

  src = builtins.fetchGit {
    url = "https://github.com/dedsyn4ps3/nix-incognito.git";
    ref = "main";
  };

  cargoSha256 = "sha256-dgW2toCpqrJYYEaN5cwGhzXPky3jL+1vRsYiV7u6rLc=";

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

  meta = with lib; {
    description = "A nifty utility that makes your desktop look like Windows";
    homepage = "https://github.com/dedsyn4ps3/nix-incognito";
    maintainers = with maintainers; [ dedsyn4ps3 ];
    platforms = platforms.unix;
    license = licenses.gpl3Plus;
  };
}
