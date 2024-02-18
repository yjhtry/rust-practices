import type { PathLike, ReadStream } from 'node:fs'
import fs from 'node:fs'
import process from 'node:process'
import readline from 'node:readline'
import { glob } from 'glob'
import pc from 'picocolors'

interface GrepProps {
  pattern: string
  glob: string
  global: boolean
  ignoreCase: boolean
}

interface StrategyParams {
  path: PathLike
  reader: ReadStream
  pattern: RegExp
  writer: NodeJS.WriteStream
}

type Strategy = (params: StrategyParams) => void

export class Grep {
  pattern: string
  glob: string
  global: boolean
  ignoreCase: boolean

  constructor(props: GrepProps) {
    this.pattern = props.pattern
    this.glob = props.glob
    this.global = props.global
    this.ignoreCase = props.ignoreCase
  }

  buildPattern() {
    return new RegExp(this.pattern, this.ignoreCase ? 'i' : '')
  }

  async match_with_default_strategy() {
    this.match_with(default_strategy)
  }

  async match_with(strategy: Strategy) {
    const pattern = this.buildPattern()
    const files = await glob(this.glob)
    const writer = process.stdout

    for (const path of files) {
      const reader = fs.createReadStream(path)

      await strategy({
        path,
        reader,
        pattern,
        writer,
      })
    }
  }
}

function default_strategy({ path, reader, pattern, writer }: StrategyParams) {
  const rl = readline.createInterface({
    input: reader,
    output: process.stdout,
    terminal: false,
  })

  const matches: string[] = []
  let lineNumber = 0

  // execute hhh
  rl.on('line', (chunk) => {
    const match = chunk.match(pattern)

    lineNumber += 1

    if (match) {
      matches.push(formatLine({
        lineNumber,
        line: chunk,
        match: match[0],
        start: match.index as number,
      }))
    }
  })

  rl.on('close', () => {
    if (matches.length > 0) {
      writer.write(`${pc.green(path as string)}\n`)

      writer.write(`${matches.join('\n')}\n\n`)
    }
  })
}

function formatLine({
  line,
  lineNumber,
  start,
  match,
}: {
  line: string
  lineNumber: number
  match: string
  start: number
}) {
  const before = line.slice(0, start)
  const after = line.slice(start + match.length)
  const prefix = `${' '.repeat(6)}${pc.blue(lineNumber)}:${pc.cyan(before.length + 1)}${' '.repeat(2)}`
  const content = `${before}${pc.red(match)}${after}`.replaceAll(match, pc.red(match))

  return `${prefix}${content}`
}
