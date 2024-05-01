{lib, makeRustPlatform, rust-bin, ...}:

let
  rustPlatform = makeRustPlatform {
    cargo = rust-bin.nightly.latest.minimal;
    rustc = rust-bin.nightly.latest.minimal;
  };
in

rustPlatform.buildRustPackage {
  pname = "nix-incognito-test";
  version = "v0.1.5";

  src = builtins.fetchGit {
    url = "ssh://git@github.com/dedsyn4ps3/nix-incognito-test.git";
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

    cp -r target/x86_64-unknown-linux-gnu/release/nix-incognito-test $out/bin/
    cp -r backgrounds/* $out/share/backgrounds/incognito/
    cp -r themes/* $out/share/themes/
    cp -r icons/* $out/share/icons/
  '';

  meta = with lib; {
    description = "A nifty utility that makes your desktop look like Windows";
    homepage = "https://github.com/dedsyn4ps3/nix-incognito-test";
    maintainers = with maintainers; [ dedsyn4ps3 ];
    platforms = platforms.unix;
    license = licenses.gpl3Plus;
  };
}
