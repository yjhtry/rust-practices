#!/usr/bin/env node
import yargs  from 'yargs';
import { hideBin }  from 'yargs/helpers';

import { handleGet, handlePost } from './core';
import { validateBody, validateUrl } from './validate';


yargs(hideBin(process.argv))
  .command('get <url>', 'Print the get request response', (yargs) => {
    return yargs
      .positional('url', {
        describe: 'The get request url',
        type: 'string',
        demandOption: true
      }).check((argv) => {
        validateUrl(argv.url);

        return true;
      })
  }, async (argv) => {
    await handleGet(argv.url);
  })
  .command('post <url>', 'Print the post request response', (yargs) => {
    return yargs
      .positional('url', {
        describe: 'The post request url',
        type: 'string',
        demandOption: true
      })
      .option('body', {
        describe: 'The post request body',
        type: 'string',
        demandOption: true,
        array: true
      })
      .check((argv) => {
        validateUrl(argv.url);

        validateBody(argv.body);

        return true;
      })
  }, async (argv) => {
    handlePost(argv.url, argv.body);
  })
  .alias('h', 'help')
  .version('version', '1.0.0')
  .alias('v', 'version')
  .help()
  .argv
