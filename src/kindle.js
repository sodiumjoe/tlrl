import { assign, map } from 'lodash';
import { createReadStream } from 'fs';
import { mailgun as config } from './config.js';
import mailgun from 'mailgun.js';

const {
  auth: key,
  email
} = config;

const mg = mailgun.client({ username: 'api', key });

export const send = (filenames, verbose) =>
  Promise.all(map(filenames, filename => {
    verbose && console.log({ key, email });
    return mg.messages.create('mg.xoxomoon.com', assign({}, email, {
      subject: 'tl;rl',
      attachment: [createReadStream(filename)],
      text: 'article'
    }))
      .then(() => verbose && console.log(`sent: ${filename}`))
      .catch(console.error.bind(console));
  }));
