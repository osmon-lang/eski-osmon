
<header>
<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/osmon-lang/.github/raw/main/ASSETS/Osmon%20White.png">
  <img alt="Osmon's Logo" height="100" align="left" src="https://github.com/osmon-lang/.github/raw/main/ASSETS/Osmon%20Black.png">
</picture>
<h1 style="display: inline">Osmon</h1>

Programming language made by Uzbek developers.

</header>

[![GitHub top language](https://img.shields.io/github/languages/top/osmon-lang/osmon?color=232323&logo=github&labelColor=232323)](https://github.com/osmon-lang/osmon)
[![Channel](https://img.shields.io/badge/telegram-grey?color=232323&label=chat&logo=telegram&labelColor=232323)](https://t.me/osmonlang)
[![Release CI](https://img.shields.io/github/actions/workflow/status/osmon-lang/osmon/release.yaml?color=232323&label=release&logo=github-actions&labelColor=232323)](https://github.com/osmon-lang/osmon/actions/workflows/release.yaml)
[![Tests CI](https://img.shields.io/github/actions/workflow/status/osmon-lang/osmon/test.yaml?color=232323&label=test&logo=github-actions&labelColor=232323)](https://github.com/osmon-lang/osmon/actions/workflows/test.yaml)
 
## About

Programming language that adopts uzbek dialect for its keywords and delivers a new experience for developers. This project is heavily 
inspired from [Sukhrob Khakimov's](https://github.com/sukhrobkhakimov) open source projects.

## Features

- Has a virtual machine that is written in Rust
- Uses libgccjit for static compilation
- Has a simple syntax adopted from C family languages

## Example

```
funksiya faktorial(n) {
    agar n == 0 {
        qaytar 1;
    }
    qaytar faktorial(n - 1) * n;
}

klass Faktorial {
    funksiya yarat(v) {
        shu._v = v;
        qaytar shu;
    }

    funksiya qiymat() {
        agar shu._v == 0 {
            qaytar 1;
        }

        joy f = Faktorial(shu._v - 1);
        joy v = shu._v;


        qaytar f.qiymat() * v;
    }
}

funksiya asosiy() {
    yoz("Klasslik faktorial(5) = ", Faktorial(5).qiymat());
    yoz("Rekursiv faktorial(5) = ", faktorial(5));
}
```

## Installation

For *NIX based operating systems, you can install Osmon by running the following command:

```bash
curl -fsSL https://osmon.dev/install/install.sh | sh
```

for Windows, open PowerShell and run the following command:

```powershell
iwr https://osmon.dev/install/install.ps1 -useb | iex
```

## License

This project is licensed under dual licence MIT and Apache-2.0 Licenses - see the [MIT](../license-mit) and [Apache](../license-apache) files for details.