export default [
  [
    "link",
    {
      rel: "icon",
      type: "image/png",
      sizes: "16x16",
      href: `/favicon-16x16.png`,
    },
  ],
  [
    "link",
    {
      rel: "icon",
      type: "image/png",
      sizes: "32x32",
      href: `/favicon-32x32.png`,
    },
  ],
  ["link", { rel: "manifest", href: "/site.webmanifest" }],
  ["meta", { name: "application-name", content: "Osmon" }],
  ["meta", { name: "apple-mobile-web-app-title", content: "Osmon" }],
  ["meta", { name: "apple-mobile-web-app-status-bar-style", content: "black" }],
  [
    "link",
    {
      rel: "apple-touch-icon",
      sizes: "180x180",
      href: `/apple-touch-icon.png`,
    },
  ],
  ["meta", { name: "msapplication-TileColor", content: "#000000" }],
  ["meta", { name: "theme-color", content: "#000000" }],
];
