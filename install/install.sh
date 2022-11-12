#!/bin/sh

set -e

if ! command -v unzip >/dev/null; then
	echo "Xatolik: Osmon ni o'rnatish uchun unzip kerak..." 1>&2
	exit 1
fi

if [ "$OS" = "Windows_NT" ]; then
	target="x86_64-pc-windows-msvc"
else
	case $(uname -sm) in
	"Darwin x86_64") target="x86_64-apple-darwin" ;;
	"Darwin arm64") target="aarch64-apple-darwin" ;;
	*) target="x86_64-unknown-linux-gnu" ;;
	esac
fi

if [ $# -eq 0 ]; then
	osmon_uri="https://github.com/osmon-lang/osmon/releases/latest/download/osmon-${target}.zip"
else
	osmon_uri="https://github.com/osmon-lang/osmon/releases/download/${1}/osmon-${target}.zip"
fi

osmon_install="${OSMON_INSTALL:-$HOME/.osmon}"
bin_dir="$osmon_install/bin"
exe="$bin_dir/osmon"

if [ ! -d "$bin_dir" ]; then
	mkdir -p "$bin_dir"
fi

curl --fail --location --progress-bar --output "$exe.zip" "$osmon_uri"
unzip -d "$bin_dir" -o "$exe.zip"
chmod +x "$exe"
rm "$exe.zip"

echo "Osmon $exe da muvaffaqiyatli o'rnatildi"
if command -v osmon >/dev/null; then
	echo "Ko'proq ma'lumot uchun 'osmon --help' buyruq satrini ishga tushuring"
else
	case $SHELL in
	/bin/zsh) shell_profile=".zshrc" ;;
	*) shell_profile=".bash_profile" ;;
	esac
	echo "O'zingiz ushbu satrlarni \$HOME/$shell_profile (yoki shunga o'xshash) ga qo'shib qo'ying"
	echo "  export OSMON_INSTALL=\"$osmon_install\""
	echo "  export PATH=\"\$OSMON_INSTALL/bin:\$PATH\""
	echo "Ko'proq ma'lumot uchun '$exe --help' buyruq satrini ishga tushuring"
fi
