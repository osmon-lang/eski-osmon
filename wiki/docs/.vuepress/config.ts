import { defineUserConfig } from "vuepress";
import type { DefaultThemeOptions } from "vuepress";
import themeConfig from "./configs/theme";
import plugins from "./configs/plugins";
import head from "./configs/head";

export default defineUserConfig<DefaultThemeOptions>({
  // @ts-ignore
  head,
  base: "/",
  lang: "uz",
  themeConfig,
  title: "Osmon Wiki",
  theme: "@vuepress/theme-default",
  description: "Osmon Dasturlash Tili Wikipediasi",
  bundler: process.env.DOCS_BUNDLER ?? "@vuepress/vite",
  // @ts-ignore
  plugins,
});
