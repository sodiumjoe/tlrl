import {
  chain,
  includes,
  isEmpty,
  last,
  map,
  property,
  reduce
} from 'lodash';
import url from 'url';
import { prompt, Separator } from 'inquirer';
import { getNewFavs, getOldFavs, unFav } from './api.js';

import { error } from '../utils.js';
import { parse } from './readability.js';
import { send } from './kindle.js';

const IGNORED_HOSTNAMES = [
  'www.echojs.com',
  'www.youtube.com',
  'youtu.be',
  'vimeo.com',
  'www.nytimes.com',
  'imgur.com'
];

const IGNORED_EXTS = [
  'png',
  'jpg',
  'gif'
];

getNewFavs().then(pickFavs);

function pickFavs(favs) {

  const choices = chain(favs)
  .reject(fav => isEmpty(fav.entities.urls))
  .reduce((memo, fav) => {

    const {
      id_str: id,
      text,
      entities: { urls }
    } = fav;

    return memo.concat(reduce(urls, (memo, { expanded_url }) => {
      if (includes(IGNORED_EXTS, last(url.parse(expanded_url).pathname.split('.')))) {
        return memo;
      }
      if (includes(IGNORED_HOSTNAMES, url.parse(expanded_url).hostname)) {
        return memo;
      }
      return memo.concat([{
        name: text.replace(/(\r\n|\n|\r)/gm,' '),
        disabled: ' '
      }, {
        name: expanded_url,
        value: { expanded_url, id }
      }]);
    }, [])).concat(new Separator(' '));

  }, [new Separator(), new Separator(), new Separator()])
  .value();

  const questions = [{
    type: 'checkbox',
    name: 'unfavs',
    message: 'too long; read later',
    choices,
    pageSize: Math.min(process.stdout.rows, choices.length)
  }];

  return prompt(questions)
  .then(property('unfavs'))
  .then(unfavs => {
    if (isEmpty(unfavs)) {
      return olderFavs();
    }
    return Promise.all(map(unfavs, ({ expanded_url }) => parse(expanded_url)))
    .then(send)
    .then(() => Promise.all(map(unfavs, ({ id }) => unFav({ id }))));
  })
  .catch(error)
  .then(olderFavs);

}

function olderFavs() {
  return prompt([{
    type: 'confirm',
    name: 'more',
    default: true,
    message: 'check older favs?'
  }])
  .then(property('more'))
  .then(more => more ? getOldFavs() : process.exit())
  .then(pickFavs);
}
