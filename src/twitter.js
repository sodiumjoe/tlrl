import {
  assign,
  reduce
} from 'lodash';
import Twitter from 'twitter';
import Promise from 'bluebird';
import { auth, id } from './config';

const client = new Twitter(auth);
const defaultParams = { count: 200, id };

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
