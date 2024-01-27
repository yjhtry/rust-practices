import fs from 'node:fs';

import TurndownService from 'turndown';

async function main() {
  const url = 'https://www.rust-lang.org/'
  const output = 'rust.md'

  console.log(`Fetching ${url}...`);

  const res = await (await fetch(url)).text();

  const turndownService = new TurndownService();

  console.log('Converting to markdown...')

  const markdown = turndownService.turndown(res);

  fs.writeFileSync(output, markdown);

  console.log(`Markdown saved to ${output}`);
}

main()
