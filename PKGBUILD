# Maintainer: Joao Freitas <joaj dot freitas at gmail dot com>

pkgname=python-marcador
_name=${pkgname#python-}
pkgver=0.5.0
pkgrel=1
pkgdesc='Lightweight bookmark manager with rofi integration'
arch=('any')
url='https://pypi.org/project/marcador'
license=('GPLv3')
depends=('python' 'python-clipboard' 'python-click' 'python-serde' 'python-appdirs', 'python-toml')
optdepends=()
makedepends=('python-build' 'python-installer' 'python-wheel' 'poetry')
provides=('marcador')
source=("https://files.pythonhosted.org/packages/source/${_name::1}/${_name}/${_name}-${pkgver}.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$_name-$pkgver"
    python -m build --wheel --no-isolation
}

package() {
    cd "$_name-$pkgver"
    python -m installer --destdir="$pkgdir" dist/*.whl
    install -D --mode=644 ${srcdir}/systemd/marcador.service ${pkgdir}/usr/lib/systemd/user/marcador.service
}
