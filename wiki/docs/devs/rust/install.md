# Rustni o'rnatish

Rustni o'rnatish judaya ko'p usullar bor. Xohlang paket menejerlari, xohlang
rasmiy websaytidan o'rnatuvchilarni yuklang, xohlang qaysidir dasturchining
websaytidan. Ammo, osmon ga hissangizni qo'shishni istasangiz va dasturchilik
paytida muammolardan holis bo'lishni istasangiz osmon muallifi keltirib o'tgan
usullar yordamida rust ni o'rnatish va sozlashni tavsiya qilamiz.

## Rustup usuli (rasman ham shu usul taklif qilinadi)

Rustup bu Rust ishlashi uchun kerakli bo'lgan komponentlar o'rnatish, yangilash
va ularni boshqarish uchun yordam beradigan dasturdir. Rustup ni o'zini ham
o'rnatish har xil usullari mavjud.

### Paket Menejeri

Agar sizning operatsion tizimingizda paket menejeri mavjud bo'lsa, quyidagi
keltirilgan tanlovlardan o'zingizning paket menejeringizni tanlab o'rnatish
buyruq satrini ko'chiring va ishga tushuring:

<CodeGroup>
  <CodeGroupItem title="Brew" active>

```bash
brew install rustup-init
```

  </CodeGroupItem>

  <CodeGroupItem title="Pacman">

```bash
sudo pacman -S rustup
```

  </CodeGroupItem>
</CodeGroup>

Keyin o'rnatilgan rustup yordamida rust kompilyator va komponentlarini o'rnatib
qo'yamiz:

```bash
rustup install stable
```

Agar sizning operatsion tizimingizda keltirilgan paket menejerlari bo'lmasa,
pastgi qismlardagi o'rnatish uslublari bo'yicha o'rnating

### \*NIX / Linux

Agar sizda \*NIX oilasiga mansub operatsion tizim bo'lsa, quyidagi buyruq
satrini ishga tushurish orqali osongina o'rnatib olsangiz bo'ladi:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Bu bizga avtomatik tarzda rustup o'rnatib sozlab beradi. Bundan so'ng esa rustup
yordamida rust kompilyatori va uning paket menejeri hisoblanmish "cargo" ni
o'rnatamiz. Buning uchun esa, terminal ochamiz va quyidagi buyruq satrini ishga
tushuramiz:

```bash
rustup install stable
```

Agar ushbu jarayon muvaffaqiyatli tugallansa, siz rust va uning komponentlarini
eng oxirgi versiyada o'rnatgan hisoblanasiz va bunga `rustc --version`
komandasini terib ko'rish yordamida amin bo'lsangiz bo'ladi.

### Windows

Windowsda esa ushbu jarayon sal boshqacharoqdir. Operatsion tizim razryadiga
qarab turib o'rnatuvchi tanlaysiz. Agar sizning operatsion tizmingiz x32 bitlik
bo'lsa,
[shu havola](https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe)
dan rustup o'rnatuvchisini yuklab, ishga tushurasiz. Aks holda x64 bitligini
[shu havola](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)
dan o'rnatasiz. Undan so'ng esa, rustc kompilyatori uchun kerakli bo'lgan
komponent, ya'ni
[Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
ni o'rnatamiz. Bunda Visual Studioni o'zi emas, uning komponentlari ichida
keladi xolos.

![Visual Studio Build Tools o'rnati jarayoni](/devs/rust/only-c++.png) O'rnatish
jarayonida faqat ko'rsatilgan variantni tanlash bilan cheklanamiz.

Hamma kerakli narsalar va rustup o'zini o'rnatib bo'lgach, rustup yordamida rust
kompilyatori va uning paket menejeri hisoblanmish "cargo" ni o'rnatamiz. Buning
uchun esa, powershell yoki terminal ochib turib, quyidagi buyruq satrini ishga
tushuramiz:

```bash
rustup install stable
```

Agar ushbu jarayon muvaffaqiyatli tugallansa, siz rust va uning komponentlarini
eng oxirgi versiyada o'rnatgan hisoblanasiz va bunga `rustc --version`
komandasini terib ko'rish yordamida amin bo'lsangiz bo'ladi.
