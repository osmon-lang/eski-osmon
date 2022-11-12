import type { SidebarConfig } from "@vuepress/theme-default";

const devs = [
  {
    text: "Rust",
    children: ["/devs/rust/readme.md", "/devs/rust/install.md"],
  },
  {
    text: "Qo'llanma",
    children: ["/devs/guide/preparation.md", "/devs/guide/compilation.md"],
  },
];

const sidebar: SidebarConfig = {
  "/devs/rust/": devs,
  "/devs/guide/": devs,
};

export default sidebar;
