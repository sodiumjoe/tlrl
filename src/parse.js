const Mercury = require("@postlight/mercury-parser");
const { JSDOM } = require("jsdom");
const fetch = require("node-fetch");
const sharp = require("sharp");
const NewYorkerExtractor = require("./extractors/new-yorker");
const WiredExtractor = require("./extractors/wired");

Mercury.addExtractor(NewYorkerExtractor);
Mercury.addExtractor(WiredExtractor);

const getBase64EncodedImg = async (src) => {
  const response = await fetch(src);
  const buffer = await response.buffer();
  const img = await sharp(buffer);
  const metadata = await img.metadata();
  const resized = metadata.width < 758 ? img : await img.resize({ width: 758 });
  const output = await resized.greyscale().jpeg({ quality: 40 }).toBuffer();
  return output.toString("base64");
};

const inlineImages = async (html) => {
  const dom = new JSDOM(html);
  const { window } = dom;
  for (const img of window.document.querySelectorAll("img")) {
    try {
      const base64Img = await getBase64EncodedImg(img.src);
      img.src = `data:image/jpeg;base64,${base64Img}`;
      if (img.srcset) {
        img.removeAttribute("srcset");
      }
    } catch (e) {
      console.warn(`Skipping img: ${img.src}`);
    }
    img.remove();
  }
  return dom.serialize();
};

module.exports = async (url) => {
  const article = await Mercury.parse(url);
  const { title, content, author, date_published, domain } = article;

  const html = await inlineImages(
    `<!DOCTYPE html><html lang="en"><head><meta http-equiv="content-type" content="text/html; charset=UTF-8"><title>${title}</title></head><body><h1>${title}</h1>${content}</body></html>`
  );

  const date = date_published && new Date(date_published).toLocaleDateString();

  return { title, content: html, author, date, domain };
};
