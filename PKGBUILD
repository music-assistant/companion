# Maintainer: Jonathan Bangert <jonathan@bangert.dk>
pkgname='massapp'
pkgver=0.0.4
pkgrel=1
pkgdesc="Music Assistant Desktop app"
arch=('x86_64')
url="https://github.com/Un10ck3d/massapp"
conflicts=(squeezelite)
provides=(squeezelite, musicassistantdesktop)
license=('Apache-2.0')
depends=(webkit2gtk)
makedepends=(cargo git rust)
md5sums=('2df9ac54714c5f78cfc8949f78cf244b')
source=("$pkgname-$pkgver.tar.gz::https://github.com/Un10ck3d/massapp/archive/v$pkgver.tar.gz")
changelog=$pkgname.changelog

build() {
  cd "$srcdir/$pkgname"
  git submodule update --init --recursive
	sudo npm install -g yarn
  yarn
  yarn run tauri build -b none
}

package() {
  cd "$srcdir/$pkgname"
	install -DCm644 ./musicassistant.desktop "$pkgdir/usr/share/applications/musicassistant.desktop"
  install -DCm644 ./app-icon.png "$pkgdir/usr/share/icons/hicolor/512x512/apps/musicassistant.png"
  install -DCm0755 -t "$pkgdir/usr/bin/" ./src-tauri/target/release/musicassistantdesktop
  install -DCm0755 -t "$pkgdir/usr/bin/" ./src-tauri/target/release/squeezelite
}
