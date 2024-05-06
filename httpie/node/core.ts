const pc = require('picocolors');
const mime = require('mime-types');

export async function handleGet(url: string) {
  const res = await fetch(url);

  printResponse(res);

}

export async function handlePost(url: string, body: string[]) {
  const data = parseKvPairs(body);

  const res = await fetch(url, {
    method: 'POST',
    body: JSON.stringify(data)
  });

  printResponse(res);
}

export function parseKvPairs(body: string[]) {
  return body.reduce((acc, curr) => {
    const [key, value] = curr.split('=');
    acc[key] = value;
    return acc;
  }, {} as Record<string, string>);
}

function printResponse(res: Response) {
  printStatus(res);
  printHeaders(res);
  printBody(res);
}


function printStatus(res: Response) {
  const status = res.status;
  const statusText = res.statusText;

  switch (status) {
    case 200:
      console.log(`${pc.blue("Response status:")} ${pc.green(status)} ${pc.green(statusText)}`);
      break;
    default:
      console.log(`${pc.blue("Response status:")} ${pc.red(status)} ${pc.red(statusText)}`);
      break;
  }
}

function printHeaders(res: Response) {
  res.headers.forEach((value, name) => {
    console.log(`${pc.blue(name)}: ${pc.green(value)}`);
  })
  
  console.log('\n');
}

async function printBody(res: Response) {
  const contentType = res.headers.get('content-type');

  if (mime.extension(contentType) === 'json') {
    const json = await res.json();
    console.log(pc.blue(JSON.stringify(json, null, 2)));
    return;
  }

  console.log(pc.cyan(await res.text()));
}
