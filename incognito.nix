{ lib
, rustPlatform
}:

rustPlatform.buildRustPackage rec {
  pname = "nix-incognito-test";
  version = "0.1.3";
  
  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  src = builtins.fetchGit {
    url = "ssh://git@github.com/dedsyn4ps3/nix-incognito-test.git";
    ref = "main";
  };

  cargoSha256 = lib.fakeSha256;

  preInstall = ''
    mkdir -p $out/share/themes
    mkdir -p $out/share/icons
  '';

  installPhase = ''
    runHook preInstall
    mkdir -p $out/share/backgrounds/incognito
    cp -r backgrounds/* $out/share/backgrounds/incognito/
    cp -r themes/* $out/share/themes/
    cp -r icons/* $out/share/icons/
  '';

  meta = with lib; {
    description = "A NixOS utility that makes your desktop look like Windows";
    homepage = "https://github.com/dedsyn4ps3/nix-incognito-test";
    maintainers = with maintainers; [ dedsyn4ps3 ];
    platforms = platforms.unix;
    license = licenses.gpl3Plus;
  };
}
