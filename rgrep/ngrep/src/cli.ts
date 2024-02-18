import process from 'node:process'
import yargs from 'yargs'
import { hideBin } from 'yargs/helpers'
import { version } from '../package.json'
import { Grep } from './grep'

// eslint-disable-next-line no-unused-expressions
yargs(hideBin(process.argv))
  .command('$0 <pattern> <glob>', 'Execute ngrep command', {
    pattern: {
      describe: 'Pattern to search for',
      type: 'string',
      required: true,
    },
    glob: {
      describe: 'glob pattern to search',
      type: 'string',
      required: true,
    },
    ignoreCase: {
      describe: 'Ignore case',
      type: 'boolean',
      default: false,
      alias: 'i',
    },
  }, (args) => {
    const grep = new Grep(args)

    grep.match_with_default_strategy()
  })
  .showHelpOnFail(false)
  .alias('h', 'help')
  .version('version', version)
  .alias('v', 'version')
  .help()
  .argv
