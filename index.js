#! /usr/bin/env node
const { argv, platform } = require('node:process')
const { execFileSync } = require('node:child_process')
const path = require('node:path')

const binaryMap = {
  win32: 'x86_64-pc-windows-gnu/truth-rs.exe',
  linux: 'x86_64-unknown-linux-musl/truth-rs',
  darwin: 'x86_64-apple-darwin/truth-rs',
}

const binPath = path.join(__dirname, 'bin')

function execBinary() {
  try {
    let params = argv.slice(2)
    execFileSync(path.join(binPath, binaryMap[platform]), params, {
      stdio: 'inherit',
    })
  } catch (error) {}
}

execBinary()
