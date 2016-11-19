import { property } from 'lodash';
import { promisify, promisifyAll } from 'bluebird';
const request = promisify(require('request'));
const { writeFileAsync } = promisifyAll(require('fs'));
import { mercury } from './config.js';

const {
  token,
  tmpDir
} = mercury;

const baseUrl = 'https://mercury.postlight.com/parser';

export const parse = url => request({
  url: `${baseUrl}?url=${url}`,
  headers: {
    'Content-Type': 'application/json',
    'x-api-key': token
  }
})
.then(property('body'))
.then(JSON.parse)
.then(body => {
  const content = makeDocument(body);
  const {
    title,
    author,
    domain
  } = body;
  const fileName = `${tmpDir}/${title} - ${author} - ${domain}.html`;
  return writeFileAsync(fileName, content)
  .then(() => fileName);
});

function makeDocument({
  title,
  domain,
  content,
  author,
  date_published,
  lead_image_url,
  url
}) {
  return `<!doctype html>
<html>
  <head>
    <title>${title}</title>
  </head>
  <body>
    <article>
      <h1>${title}</h1>
      <address>${author}</address>
      <time pubdate datetime="${date_published}" title="${date_published}">
        ${date_published}
      </time>
      <div><a href="${url}">${domain}</a></div>
      <img src="${lead_image_url}" />
      ${content}
    </article>
  </body>
</html>`;
}
