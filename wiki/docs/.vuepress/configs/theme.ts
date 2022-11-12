import { ThemeConfig } from "vuepress";
import sidebar from "./sidebar";
import navbar from "./navbar";

export default <ThemeConfig>{
  home: "/",
  logo: "https://raw.githubusercontent.com/osmon-lang/.github/main/ASSETS/Osmon%20Black.png",
  logoDark:
    "https://raw.githubusercontent.com/osmon-lang/.github/main/ASSETS/Osmon%20White.png",
  repo: "osmon-lang/osmon",
  navbar: navbar,
  sidebar: sidebar,
  repoLabel: "Repozitoriya",
  docsRepo: "osmon-lang/osmon-wiki",
  docsDir: "docs",
  docsBranch: "main",
  editLink: true,
  editLinkText: "Sahifaga o'zgartirish kiritish",
  lastUpdated: true,
  lastUpdatedText: "Oxirgi o'zgarish",
  contributors: true,
  contributorsText: "Mualliflar",
  tip: "Maslahat",
  warning: "Ogohlantiruv",
  danger: "Ehtiyot bo'ling",
  notFound: [
    "Afsuski ushbu sahifa mavjud emas...",
    "Bu yerda nimalar qilib yuribsiz?",
  ],
  backToHome: "Orqaga qaytish",
  openInNewWindow: "yangi oynada ochish",
  toggleDarkMode: "qiyofa rejimini o'zgartirish",
};
