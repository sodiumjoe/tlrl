module.exports = {
  domain: "www.wired.com",
  title: {
    selectors: ["h1.content-header__hed"],
  },

  author: {
    selectors: [['meta[name="author"]', "value"], 'a[rel="author"]'],
  },

  content: {
    selectors: ["article.article.main-content", "article.content"],

    // Is there anything in the content you selected that needs transformed
    // before it's consumable content? E.g., unusual lazy loaded images
    transforms: [],

    // Is there anything that is in the result that shouldn't be?
    // The clean selectors will remove anything that matches from
    // the result
    clean: [".visually-hidden", "figcaption img.photo"],
  },

  date_published: {
    selectors: [
      "time.content-header__publish-date",
      ['meta[itemprop="datePublished"]', "value"],
    ],
  },

  lead_image_url: {
    selectors: [['meta[name="og:image"]', "value"]],
  },

  dek: {
    selectors: [],
  },

  next_page_url: null,

  excerpt: null,
};
