import {
  // first,
  isEmpty,
  // last,
  map,
  reduce
} from 'lodash';
import { prompt, Separator } from 'inquirer';
import { getFavs, unFav } from './twitter.js';

import { error } from './utils.js';

// import { write , newestId, oldestId } from './config';

// @TODO use since_id from config, run recursively
getFavs().then(favs => {

  const choices = reduce(favs, (memo, fav) => {

    const {
      id_str: id,
      text,
      entities: { urls }
    } = fav;

    return isEmpty(urls)
    ? memo
    : memo.concat(reduce(urls, (memo, { expanded_url }) => memo.concat([{
      name: text.replace(/(\r\n|\n|\r)/gm,' '),
      disabled: ' '
    }, {
      name: expanded_url,
      value: { expanded_url, id }
    }]), [])).concat(new Separator(' '));

  }, [new Separator(), new Separator(), new Separator()]);

  const questions = [{
    type: 'checkbox',
    name: 'unfavs',
    message: 'too long; read later',
    choices,
    pageSize: Math.min(process.stdout.rows, choices.length)
  }];

  prompt(questions).then(({ unfavs }) => {
    // @TODO readability/parse urls
    // @TODO make html documents
    // @TODO write to /tmp
    // @TODO send to kindle
    // unfav selections
    return Promise.all(map(unfavs, ({ id }) => unFav({ id })));
    // @TODO write first/last from unfavs to config
    // const config = {
    //   newestId: first(favs).id,
    //   oldestId: last(favs).id
    // };
    // write(config).then(() => console.log('done')).catch(error);
  })
  .then(() => console.log('done'))
  .catch(error);

});
