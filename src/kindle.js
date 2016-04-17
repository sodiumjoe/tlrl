import { assign, map } from 'lodash';
import { createReadStream } from 'fs';
import { post } from 'request';
import { mailgun } from './config.js';

const {
    auth,
    email
} = mailgun;

const url = 'https://api.mailgun.net/v3/mg.xoxomoon.com/messages';

export const send = filenames =>
  Promise.all(map(filenames, filename => new Promise((resolve, reject) => {
    post({
      url,
      auth,
      formData: assign({}, email, {
        attachment: createReadStream(filename)
      })
    }, err => err ? reject(err) : resolve());
  })));
