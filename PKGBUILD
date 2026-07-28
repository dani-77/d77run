# Maintainer: Daniel Azevedo
#
# Real Arch package for d77run. Build and install with:
#
#   makepkg -si
#
# from inside this repository (it packages the working tree in place, no
# network fetch of the source itself).

pkgname=d77run
pkgver=0.1.0
pkgrel=1
pkgdesc="Minimal GTK4 run-dialog launcher (a rejuvenation of gmrun), with .desktop icon rendering"
arch=('x86_64' 'aarch64')
url="https://github.com/dani-77/d77run"
license=('MIT')
depends=('gtk4')
makedepends=('cargo')

build() {
    cd "$startdir"
    cargo build --release --locked
}

check() {
    cd "$startdir"
    cargo test --release --locked
}

package() {
    cd "$startdir"
    install -Dm755 target/release/d77run "$pkgdir/usr/bin/d77run"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
