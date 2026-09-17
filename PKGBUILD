# Maintainer: Ven0m0 <ven0m0.wastaken@gmail.com>
pkgname=shell-mommy
_name=mommy
pkgver=0.1.6
pkgrel=1
pkgdesc="Affirmations in your terminal, for shell commands and cargo (mommy + cargo-mommy)"
arch=('x86_64' 'aarch64')
url="https://github.com/Ven0m0/mommy"
license=('Unlicense')
depends=('gcc-libs' 'glibc')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
# No v$pkgver tag is published yet; run `updpkgsums` after pushing the tag.
sha256sums=('SKIP')

prepare() {
  cd "$_name-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
  cd "$_name-$pkgver"
  export RUSTUP_TOOLCHAIN=stable CARGO_TARGET_DIR=target
  # .cargo/config.toml targets x86-64-v3 for local builds. Setting RUSTFLAGS
  # (even to makepkg's value or empty) replaces those flags, so the package
  # runs on any CPU the arch supports.
  export RUSTFLAGS="${RUSTFLAGS-}"
  # Default features only: the opt-in `beg` feature writes ~/.mommy.state.
  cargo build --frozen --release
}

check() {
  cd "$_name-$pkgver"
  export RUSTUP_TOOLCHAIN=stable CARGO_TARGET_DIR=target RUSTFLAGS="${RUSTFLAGS-}"
  cargo test --frozen
}

package() {
  cd "$_name-$pkgver"
  install -Dm755 target/release/mommy -t "$pkgdir/usr/bin"
  # mommy picks shell or cargo mode from its own filename, and current_exe()
  # resolves symlinks, so cargo-mommy must be a hard link, not a symlink.
  ln "$pkgdir/usr/bin/mommy" "$pkgdir/usr/bin/cargo-mommy"
  install -Dm644 README.md -t "$pkgdir/usr/share/doc/$pkgname"
  install -Dm644 examples/config.json -t "$pkgdir/usr/share/doc/$pkgname/examples"
  install -Dm644 LICENSE -t "$pkgdir/usr/share/licenses/$pkgname"
}
