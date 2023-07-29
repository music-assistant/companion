# This is an example PKGBUILD file. Use this as a start to creating your own,
# and remove these comments. For more information, see 'man PKGBUILD'.
# NOTE: Please fill out the license field for your package! If it is unknown,
# then please put 'unknown'.

# Maintainer: Jonathan Bangert <jonathan@bangert.dk>
pkgname='Music Assistant'
_pkgname='massapp'
pkgver=0.0.4
pkgrel=1
pkgdesc="Music Assistant Desktop app"
arch=('x86_64')
url="https://github.com/Un10ck3d/massapp"
conflicts=(squeezelite)
license=('Apache-2.0')
depends=(webkit2gtk)
makedepends=(cargo git rust)
md5sums=("SKIP")
source=("git+$url.git")

build() {
  cd "$srcdir/$_pkgname"
  git submodule update --init --recursive
	sudo npm install -g yarn
  yarn
  yarn run tauri build -b none
}

package() {
  cd "$srcdir/$_pkgname"
	install -DCm644 ./musicassistant.desktop "$pkgdir/usr/share/applications/musicassistant.desktop"
  install -DCm644 ./app-icon.png "$pkgdir/usr/share/icons/hicolor/512x512/apps/musicassistant.png"
  install -DCm0755 -t "$pkgdir/usr/bin/" ./src-tauri/target/release/musicassistantdesktop
  sudo install -DCm0755 -t "$pkgdir/usr/bin/" ./src-tauri/target/release/squeezelite
}
