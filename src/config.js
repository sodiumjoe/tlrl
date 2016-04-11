import { assign } from 'lodash';
import { promisifyAll } from 'bluebird';
const { writeFileAsync } = promisifyAll(require('fs'));
const { env: { HOME } } = process;
const file = `${HOME}/.tlrl.json`;
const config = require(file);
export const { auth, id, newestId, oldestId } = config;

export const write = params => {
  const newConfig = assign({}, config, params);
  return writeFileAsync(file, JSON.stringify(newConfig, null, 2));
};
