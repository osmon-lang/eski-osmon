export const arches: { arch: string; status: string }[] = [
  {
    arch: 'x86_64',
    status: 'v0.5.0'
  },
  {
    arch: 'i686',
    status: "Bo'lmaydi"
  },
  {
    arch: 'arm | arm64',
    status: 'v0.5.0'
  }
]

export const logs: { version: string; date: Date; text: string }[] = [
  {
    version: '0.1',
    date: new Date(2022, 3, 17),
    text: 'Birinchi alfa nashr chiqarildi.'
  },
  {
    version: '0.2',
    date: new Date(2022, 4, 1),
    text: 'Birinchi beta nashr va cargo reliz.'
  },
  {
    version: '0.3',
    date: new Date(2022, 4, 2),
    text: "Xatoliklar to'g'irlangan va stabil holatga keltirilgan."
  },
  {
    version: '0.4',
    date: new Date(2022, 4, 2),
    text: "Ishga tushuruvchi haqida ma'lumotlar qo'shildi."
  },
  {
    version: '0.5',
    date: new Date(2022, 5, 1),
    text: "ARM64 va ko'plab sintaktik o'zgarishlar."
  }
]
