#! /usr/bin/env node
import { argv, platform } from 'node:process'
import { execFileSync } from 'node:child_process'

const params = argv.slice(2)

if (platform === 'win32') {
  execFileSync('bin/x86_64-pc-windows-gnu/truth-rs.exe', params, {
    stdio: 'inherit',
  })
  console.log(cmd)
} else if (platform === 'linux') {
  execFileSync('bin/x86_64-unknown-linux-musl/truth-rs', params, {
    stdio: 'inherit',
  })
} else if (platform === 'darwin') {
  execFileSync('bin/x86_64-apple-darwin/truth-rs', params, {
    stdio: 'inherit',
  })
} else {
  console.log('unknown os')
}
