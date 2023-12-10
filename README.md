
<h1 align="center">
  Truth-rs
</h1>
<p align="center">用于分析依赖的命令行工具</p>

> Inspired by [truth-cli](https://github.com/truthRestorer/truth-cli), powered by [Rust](https://github.com/rust-lang/rust).

# 快速开始

## npx

```bash
# 启动网页
npx truth-rs web
# 生成 json 文件
npx truth-rs json
# 生成 txt 文件
npx truth-rs txt
# 生成 html 文件 (暂未支持)
# npx truth-rs html
```

## 安装

```bash
npm install -g truth-rs
```

## 示例

```bash
# 启动网页
truth-rs web
# 生成 json 文件
truth-rs json
# 生成 txt 文件
truth-rs txt
# 生成 html 文件(暂未支持)
# truth-rs html
```

## 包含 devDependencies

默认情况下, `truth-rs` 不包含其他依赖 devDependencies，只包含当前项目，如果希望依赖也包含 devDependencies 节点，可以使用 `--dev` 参数

```bash
truth-rs web --dev
truth-rs json --dev
truth-rs txt --dev
```

## 指定深度

> 深度只针对生成 `json` 和 `txt` 命令 ，网页端采取动态加载节点的策略。

使用 `--depth` 或 `-D` 参数:

```bash
truth-rs json --depth 2
```

```bash
truth-rs txt --depth 2
```

## 指定路径(暂未支持)

`truth-rs` 默认在项目的根目录生成文件，如果想要更改路径，可以在将路径加在 `--path` 或 `-p` 参数后

```bash
truth-rs json --path dist/
```

```bash
truth-rs txt --path dist/
```

> **WARNNING: 请不要再路径开头加上 /，这会被 nodejs 识别成根路径，从而生成失败**

# 生成文件的格式

`truth-rs json` 命令会生成 `pkgs.json` 文件:

```json
{
  "name": "_root_",
  "version": "1.0.0",
  "packages": {
    "@antfu/eslint-config": {
      "version": "^0.39.8",
      "type": 0,
      "packages": {
        // ...
      }
    }
  }
}
```

`truth-rs tree` 命令会生成 `pkgs.txt` 文件:

```txt
__root__ 1.0.0:
│
├─@antfu/eslint-config ^0.39.8
├─@changesets/cli ^2.26.2
├─@commitlint/cli ^17.7.1
├─@commitlint/config-conventional ^17.7.0
├─@rollup/plugin-commonjs ^25.0.4
├─@rollup/plugin-json ^6.0.0
├─@rollup/plugin-node-resolve ^15.2.0
├─@rollup/plugin-terser ^0.4.3
├─@rollup/plugin-typescript ^11.1.2
├─@truth-rs/core workspace:^
├─@types/minimist ^1.2.2
├─@types/node ^20.5.1
├─@vitejs/plugin-vue ^4.3.3
├─commitizen ^4.3.0
├─cz-git ^1.7.1
├─eslint ^8.47.0
├─husky ^8.0.3
├─lint-staged ^13.3.0
├─minimist ^1.2.8
├─prettier ^3.0.2
├─rollup ^3.28.0
├─ts-node ^10.9.1
├─typescript ^5.1.6
├─unplugin-auto-import ^0.16.6
├─unplugin-vue-components ^0.25.1
├─vite ^4.4.9
├─vite-plugin-compression ^0.5.1
├─vite-plugin-singlefile ^0.13.5
├─vitest ^0.34.2
├─vue ^3.3.4
```
