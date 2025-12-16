# Maintainer: Ven0m0 <ven0m0.wastaken@gmail.com>
pkgname=shell-mommy
pkgver=0.1.6
pkgrel=1
pkgdesc="Affirmations in your terminal! Now with cargo-mommy compatibility!"
arch=('x86_64' 'aarch64')
url="https://github.com/Ven0m0/mommy"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
options=('strip')
source=("$pkgname-$pkgver.tar.gz::https://github.com/Ven0m0/mommy/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP') # Update with actual checksum when releasing

prepare() {
  cd "$srcdir/mommy-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  cd "$srcdir/mommy-$pkgver"
  export RUSTUP_TOOLCHAIN=stable CARGO_TARGET_DIR=target
  cargo build --frozen -r --all-features
}

check() {
  cd "$srcdir/mommy-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  cargo test --frozen --all-features
}

package() {
  cd "$srcdir/mommy-$pkgver"
  # Install binaries
  install -Dm755 target/release/mommy "$pkgdir/usr/bin/mommy"
  install -Dm755 target/release/cargo-mommy "$pkgdir/usr/bin/cargo-mommy"
  # Install documentation
  install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
  # Install license
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE" 2>/dev/null || :
}
