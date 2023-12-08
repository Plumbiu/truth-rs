/* eslint-disable @stylistic/template-tag-spacing */
import { exec, execSync } from 'node:child_process'
import fs from 'node:fs/promises'
import path from 'node:path'
import colors from 'picocolors'

async function resolveDev() {
  let retry = 0
  const timer = setInterval(async () => {
    try {
      let start = Date.now()
      const promises = [
        fetch('http://localhost:8080/relations.json').then((res) => res.text()),
        fetch('http://localhost:8080/tree.json').then((res) => res.text()),
        fetch('http://localhost:8080/graph.json').then((res) => res.text()),
      ]
      const [relations, tree, graph] = await Promise.all(promises)
      const writePath = path.join(process.cwd(), 'packages', 'web', 'public')
      const writePromises = [
        fs.writeFile(path.join(writePath, 'relations.json'), relations),
        fs.writeFile(path.join(writePath, 'tree.json'), tree),
        fs.writeFile(path.join(writePath, 'graph.json'), graph),
      ]
      await Promise.race(writePromises)
      clearInterval(timer)
      console.log(
        '\n' +
          colors.green(colors.bold('TRUTH-RS    ')) +
          colors.dim('ready in ') +
          colors.white(Date.now() - start) +
          'ms\n',
      )
      console.log(
        colors.bold('[SERVER]: ') +
          colors.cyan(colors.underline('http://localhost:8080/api') + '\n'),
      )
      console.log(
        colors.bold('[WEB]: ') +
          colors.cyan(colors.underline('http://localhost:5174') + '\n'),
      )
      execSync('pnpm -F web run dev')
      process.exit()
    } catch (err) {
      if (retry > 50) {
        console.log(colors.red('Faild!'))
        process.exit()
      }
      if (err.message === 'fetch failed') {
        console.log(`server not start, waiting ~~~ x${++retry}`)
      }
    }
  }, 1000)
  exec('cargo watch -x \'run -- web\'')
}

resolveDev()
