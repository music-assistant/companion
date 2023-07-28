# This is an example PKGBUILD file. Use this as a start to creating your own,
# and remove these comments. For more information, see 'man PKGBUILD'.
# NOTE: Please fill out the license field for your package! If it is unknown,
# then please put 'unknown'.

# Maintainer: Jonathan Bangert <jonathan@bangert.dk>
pkgname='MassDesktop'
pkgver=0.0.3
pkgrel=1
pkgdesc="The mass desktop app"
arch=('x86_64')
url="https://github.com/Un10ck3d/massapp"
conflicts=(squeezelite)
license=('Apache-2.0')
depends=(webkit2gtk)
makedepends=(cargo git rust)
md5sums=("SKIP")
source=("git+$url.git")

build() {
  cd ".."
	sudo npm install -g yarn
  yarn
  yarn run tauri build -b none
}

package() {
  cd ".."
	install -DCm644 ./massdesktop.desktop "$pkgdir/usr/share/applications/massdesktop.desktop"
  install -DCm644 ./app-icon.png "$pkgdir/usr/share/icons/hicolor/512x512/apps/mass.png"
  install -DCm0755 -t "$pkgdir/usr/bin/" ./src-tauri/target/release/mass
  sudo install -DCm0755 -t "$pkgdir/usr/bin/" ./src-tauri/target/release/squeezelite
}
