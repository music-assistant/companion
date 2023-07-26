# This is an example PKGBUILD file. Use this as a start to creating your own,
# and remove these comments. For more information, see 'man PKGBUILD'.
# NOTE: Please fill out the license field for your package! If it is unknown,
# then please put 'unknown'.

# Maintainer: Jonathan Bangert <jonathan@bangert.dk>
pkgname='MassDesktop'
pkgver=0.0.1
pkgrel=1
pkgdesc="The mass desktop app"
arch=('x86_64')
url="https://github.com/Un10ck3d/massapp"
license=('Apache-2.0')
depends=(webkit2gtk)
makedepends=(cargo git rust)
source=("git+https://github.com/Un10ck3d/massapp.git")
md5sums=("SKIP")

build() {
  cd "$srcdir/../pkg/$pkgname"
	sudo npm install -g yarn
  yarn
  yarn run tauri build
}

package() {
  cd "$srcdir/../pkg/$pkgname"
  cd "src-tauri/target/release"
	install -Dm644 ./bundle/appimage/mass.AppDir/mass.desktop "$pkgdir/usr/share/applications/massdesktop.desktop"
  install -Dm 644 ./bundle/appimage/mass.AppDir/mass.png "$pkgdir/usr/share/icons/hicolor/512x512/apps/mass.png"
  install -Dm0755 ./mass "$pkgdir/usr/bin/"
}