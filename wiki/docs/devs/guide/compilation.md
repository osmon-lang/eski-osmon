# Kompilyatsiya

Agar reliz bilan qoniqmasangiz, ushbu dasturlash tilini o'zingizni qurilmangizda
eng oxirgi yangilanishlar bilan kompilyatsiya qilib olishingiz mumkin.

:::warning

Eng oxirgi komitada yuklangan reliz ishlashiga muallif kafolat berolmaydi. Agar
muammolardan holis bo'lishni istasangiz, relizni yuklang!

:::

Kompilyatisya qilish uchun avval Rust dasturlash tili o'rnatilganligini
tekshiring. Agar o'rnatilmagan bo'lsa [rustni o'rnatish](/devs/rust/install.md)
sahifasiga o'tib rustni o'rnatib oling.

## Ko'chirib olish

Kompilyatsiya qilish uchun avval kerakli havolalarni internetdan ko'chirib
olaylik. Ikki xil usulda ko'chirib olishimiz mumkin:
[git orqali](#git-orqali-kochirish) yoki
[zip faylda](#togridan-togri-zip-faylda-kochirish). Ko'proq git yordamida
ko'chirish tavsiya qilinadi.

### Git orqali ko'chirish

Agar sizning qurilmangizda `git` dasturi mavjud bo'lsa, `git` yordamida quyidagi
buyruq satri yordamida proyektni ko'chirib olishingiz mumkin:

```shell
git clone https://github.com/osmon-lang/osmon.git
```

### To'gridan to'g'ri zip faylda ko'chirish

Agar sizda `git` bo'lmasa, ushbu tugmachani bosish yordamida eng oxirgi
yangilanishni zip faylda ko'chirib olishingiz mumkin:

- [Eng oxirgi kommita (main branch)](https://github.com/osmon-lang/osmon/archive/refs/heads/main.zip)

## Kompilyatsiya bosqichi

Dasturni kompilyatisya qilish uchun endi ko'chirilgan proyekt papkasi ichida
terminal ochamiz va ushbu buyruq satrini ishga tushuramiz:

```shell
cargo build --release
```

va ushbu buyruq ishga tushurgandan so'ng uzoq sabr tilayman sizlarga!

![Kompilyatsiya natijasi](/devs/guide/compiled-binary.png)

Agar oxirida sizda ham shunday natija ko'rsatsa, demak kompilyatsiya
muvaffaqiyatli o'tdi. Ishga tushuruvchi faylni esa `target/release` papkasi
ichida topasiz. Agar sizda \*NIX oilasiga mansub operatsion tizim bo'lsa,
`osmon` nomida bo'ladi, aks holda `osmon.exe`.

![Kompilyatsiya payti](/devs/guide/osmon-compiled.png)

## Sinab ko'rish

Kompilyatsiya qilingan ishga tushuruvchini olib istalgan joyga qo'yamiz va
yonida yana fayl yaratamiz. Faylga `salom.osm` deb nom beramiz. Fayl ichida esa,
quyidagi dasturni yozamiz:

```
funksiya asosiy() {
    yoz("Ismingiz nima?");
    joy ism = oqi();
    yoz("Salom, ",ism);
}
```

va undan so'ng esa, shu fayllar joylashgan joyda terminal ochib, quyidagi buyruq
satrini ishga tushuramiz:

<CodeGroup>
  <CodeGroupItem title="Windows/Powershell" active>

```shell
.\osmon.exe .\salom.osm
```

  </CodeGroupItem>

  <CodeGroupItem title="*NIX/Bash">

```bash
./osmon ./salom.osm
```

  </CodeGroupItem>
</CodeGroup>

va ishga tushurgandan so'ng, ko'rsatilgan natijani olamiz.

![Ishga tushurilgan natija](/devs/guide/test-run.png)

Agar yanada ko'proq misollar bilan sinab ko'rishni istasangiz, muallif tomonidan
yaratilgan [osmon-examples](https://github.com/osmon-lang/osmon-examples)
repozitoriyasiga murojat qilishingiz mumkin.
