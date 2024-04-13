{ lib
, stdenvNoCC
, fetchFromGitHub
}:

stdenvNoCC.mkDerivation (finalAttrs: {
  pname = "nix-incognito-test";
  version = "0.1.2";

  src = fetchFromGitHub {
    owner = "dedsyn4ps3";
    repo = "nix-incognito-test";
    rev = "";
    hash = "sha256-";
  };

  installPhase = ''
    mkdir -p $out/share/backgrounds/incognito}
    cp -r backgrounds/* $out/share/backgrounds/incognito/
  '';

  meta = with lib; {
    description = "Nix-Incognito utility resources";
    homepage = "https://github.com/dedsyn4ps3/nix-incognito-test";
    maintainers = with maintainers; [ dedsyn4ps3 ];
    platforms = platforms.unix;
    license = licenses.gpl3Plus;
  };
})