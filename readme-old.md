# Osmon Tili

[![license](https://img.shields.io/github/license/osmon-lang/osmon)](https://github.com/osmon-lang/osmon/blob/main/LICENSE)
[![release](https://github.com/osmon-lang/osmon/actions/workflows/release.yaml/badge.svg)](https://github.com/osmon-lang/osmon/actions/workflows/release.yaml)

Osmon bu registrlarga asoslangan virtual mashinalik va yengil dasturlash tili

Osmon boshqa o'zbek open source dasturchisi [Sukhrob Khakimovning](https://github.com/sukhrobkhakimov) proyektlaridan ilhomlanadi.

## Maqsad

- **O'zbekona sintaksis**
- **Rust bilan integratsiya**
- **O'rganish darajasini oshirish**
- **Virtual Mashinani OOP ga moslash**

## Holis (istamaymiz)

- Judayam haddan ortiq ko'p resurslar ko'paytirish
- JIT kompilyatsiya
- Baytkod fayllarni yaratish

# Misol

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
    yoz("Klasslik faktorial(5) = ",Faktorial(5).qiymat());
    yoz("Rekursiv faktorial(5) = ",faktorial(5));
}
```
