#! /usr/bin/env node

import { each, map } from 'lodash';
import yargs from 'yargs';

import { error } from './utils.js';
import { parse } from './mercury.js';
import { send } from './kindle.js';

const argv = yargs
  .env('TLRL')
  .version()
  .alias('V', 'version')
  .wrap(null)
  .boolean('t')
  .alias('twitter')
  .count('v')
  .alias('v', 'verbose')
  .argv;

const urls = argv._;

argv.v && console.log('url(s): ');
argv.v && each(urls, url => console.log(url));

Promise.all(map(urls, url => parse(url).then(filename => {
  argv.v && console.log(`parsed ${url} saved to ${filename}`);
  return filename;
}))).then(filenames => {
  argv.v && console.log('sending to kindle:');
  argv.v && each(filenames, f => console.log(f));
  return send(filenames, argv.v);
}).catch(error);
