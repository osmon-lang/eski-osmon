import type { NavbarConfig } from "@vuepress/theme-default";

const navbar: NavbarConfig = [
  {
    text: "Ma'lumot",
    link: "/about/",
  },
  {
    text: "O'rganish",
    link: "/learn/",
  },
  {
    text: "Dasturchilar",
    children: [
      {
        text: "Rust",
        children: ["/devs/rust/readme.md", "/devs/rust/install.md"],
      },
      {
        text: "Qo'llanma",
        children: ["/devs/guide/preparation.md", "/devs/guide/compilation.md"],
      },
    ],
  },
];

export default navbar;
