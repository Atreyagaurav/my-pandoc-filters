# Maintainer: Gaurav Atreya <allmanpride@gmail.com>
pkgname=zero-pandoc-filters
pkgver=0.1
pkgrel=1
pkgdesc="My Pandoc Filters"
arch=('x86_64')
license=('GPL3')
depends=('gcc-libs')
makedepends=('rust' 'cargo')

build() {
	cargo build --release
}

package() {
    cd "$srcdir"
    mkdir -p "$pkgdir/usr/bin"
    cp "../target/release/my-pandoc-filters" "$pkgdir/usr/bin/my-pandoc-filters"
}
