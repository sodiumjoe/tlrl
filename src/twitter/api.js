import {
  assign,
  first,
  isEmpty,
  last,
  reduce
} from 'lodash';
import Twitter from 'twitter';
import Promise from 'bluebird';
import { twitter, getConfig, write } from '../config';

const { auth, id } = twitter;

const COUNT = 20;

const client = new Twitter(auth);
const defaultParams = { count: COUNT, id };

export const api = reduce(['get', 'post'], (memo, method) => {
  return assign(memo, {
    [method]: (endpoint, params) => new Promise((resolve, reject) => {
      client[method](endpoint, assign({}, defaultParams, params), (err, data) => {
        if (err) { return reject(err); }
        resolve(data);
      });
    })
  });
}, {});

export const getFavs = params => api.get('favorites/list', params);

export const unFav = params => api.post('favorites/destroy', params);

export const getNewFavs = () =>
  getFavs({ since_id: getConfig().twitter.newestId })
  .then(favs => favs.length < COUNT
    ? favs
    : getFavs({ since_id: first(favs).id }).then(more => favs.concat(more)))
  .then(favs => isEmpty(favs) ? favs : write({
    twitter: { newestId: first(favs).id }
  }).then(() => favs));

export const getOldFavs = () =>
  getFavs({ max_id: getConfig().twitter.oldestId })
  .then(favs => {
    return write({
      twitter: { oldestId: last(favs).id }
    }).then(() => favs);
  });
