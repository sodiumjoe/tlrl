import { merge } from 'lodash';
import { promisifyAll } from 'bluebird';
const { writeFileAsync } = promisifyAll(require('fs'));
const { env: { HOME } } = process;
const file = `${HOME}/.tlrl.json`;
let config = require(file);

export const getConfig = () => config;

export const { twitter, readability, mailgun } = getConfig();

export const write = params => {
  const newConfig = merge({}, getConfig(), params);
  return writeFileAsync(file, JSON.stringify(newConfig, null, 2))
  .then(() => config = newConfig);
};
