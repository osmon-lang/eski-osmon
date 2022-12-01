export default [
  "@vuepress/plugin-docsearch",
  {
    appId: "NE2QLUS92G",
    apiKey: "b1cff83b4504a7b7a60fae91b0f26973",
    indexName: "osmon",
    searchParameters: {
      facetFilters: ["tags:v2"],
    },
    placeholder: "Qidirish",
    locales: {
      "/": {
        placeholder: "Qidirishni boshlang...",
        translations: {
          button: {
            buttonText: "Qidirish",
            buttonAriaLabel: "Qidirish",
          },
          modal: {
            searchBox: {
              resetButtonTitle: "O'chirish",
              resetButtonAriaLabel: "O'chirish",
              cancelButtonText: "Bekor qilish",
              cancelButtonAriaLabel: "Bekor qilish",
            },
            startScreen: {
              recentSearchesTitle: "Qidiruv tarixi",
              noRecentSearchesText: "Tarix mavjud emas",
              saveRecentSearchButtonTitle: "Qidiruv tarixiga saqlash",
              removeRecentSearchButtonTitle: "Qidiruv tarixidan olib tashlash",
              favoriteSearchesTitle: "kolleksiya",
              removeFavoriteSearchButtonTitle: "Kolleksiyadan olib tashlash",
            },
            errorScreen: {
              titleText: "Natija topilmadi",
              helpText: "Internet faolligini tekshirib ko'ring",
            },
            footer: {
              selectText: "tanlash",
              navigateText: "yo'naltirish",
              closeText: "yopish",
              searchByText: "matn bo'yicha qidirish tizimi",
            },
            noResultsScreen: {
              noResultsText: "Bog'liq mavzular topilmadi",
              suggestedQueryText: "Urinib ko'rishingiz mumkin",
              openIssueText: "Natija bo'lishi kerak deb o'ylaysizmi?",
              openIssueLinkText: "Shu yerda yozib qoldiring",
            },
          },
        },
      },
    },
  },
];
