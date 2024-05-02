# Maintainer: Ed Rutherford <erutherford@nullsecurity.tech>

#######################################################
#   NOTE: This PKGBUILD is still a work in progress!  #
#######################################################
pkgname=nix-incognito
_pkgname=nix-incognito
pkgver=0.2.0
pkgrel=1
pkgdesc="A command line utility for changing the appearance of your system to look like Windows"
arch=('x86_64')
url="https://github.com/dedsyn4ps3/nix-incognito"
license=('GPL')
depends=()
makedepends=(
    "rustup"
    "git"
    "base-devel"
    "curl"
    "wget"
    "openssl"
)
provides=("nix-incognito")
conflicts=("nix-incognito")
source=("$pkgname::git+https://github.com/dedsyn4ps3/nix-incognito.git")
sha256sums=("SKIP")

build() {
    PURPLE=$(tput setaf 201)
    WHITE=$(tput setaf 255)
    END="\e[0m"
    
    cd "$pkgname"

    echo
    echo -e "${PURPLE}|============================|${END}"
    echo -e "${WHITE}   Installing Rust Nightly     ${END}"
    echo -e "${PURPLE}|============================|${END}"

    rustup toolchain install nightly
    rustup default nightly

    echo
    echo -e "${PURPLE}|=============================|${END}"
    echo -e "${WHITE}     Compiling Application     ${END}"
    echo -e "${PURPLE}|=============================|${END}"

    cargo build -r
}

package() {
    PURPLE=$(tput setaf 201)
    WHITE=$(tput setaf 255)
    END="\e[0m"

    cp -r backgrounds/* usr/share/backgrounds/incognito/
    cp -r themes/* usr/share/themes/
    cp -r icons/* usr/share/icons/

    install -Dm755 "usr/bin/${pkgname}" "${pkgdir}/target/x86_64-unknown-linux-gnu/release/${pkgname}"

    echo
    echo -e "${PURPLE}|=============================|${END}"
    echo -e "${WHITE}     Packaging Complete     ${END}"
    echo -e "${PURPLE}|=============================|${END}"
    echo
}
