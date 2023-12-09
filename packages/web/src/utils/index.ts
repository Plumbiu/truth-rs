import type { ECharts } from 'echarts/core'
import type {
  Legend,
  PkgInfo,
  Links,
  Nodes,
  Relations,
  Tree,
} from '@truth-rs/types'
import { loadGraph, loadTree, genGraph } from './tools'

const apiUrl = import.meta.env.DEV ? 'http://localhost:8080/api/' : '/api/'

export async function request<T>(rest: string) {
  const data = await fetch(apiUrl + rest).then((res) => res.json())
  return data as T
}

// export async function getPkgInfo(name: string) {
//   const data = await request(`relations.jon/${name}`)
//   return data
// }

let echart: ECharts
const treeNodeMap = new Map()
const nodesSet = new Set()
let relations: Relations
let graphNodes: Nodes[]
let graphLinks: Links[]
let tree: Tree
// let versions: Versions
// let circulation: Record<string, string[]>

export async function initChart(_echart: ECharts, _relation: Relations) {
  const { nodes, links } = await genGraph({ name: '__root__', category: 1 })
  relations = await request('relations.json')
  _echart.setOption(loadGraph((graphNodes = nodes), (graphLinks = links)))
  echart = _echart
}

export async function changeGraphRoot(name: string, isAim: boolean) {
  if (!relations[name]) return
  if (isAim) {
    resetChart({ nodes: graphNodes, links: graphLinks })
    return
  }
  const newNodes = [{ name, category: 2, value: relations[name].version! }]
  const { nodes, links } = await genGraph({ name })
  newNodes.push(...nodes.filter((node) => node.name !== name))
  resetChart({ nodes: newNodes, links })
}

export async function collapseNode(legend: Legend) {
  if (legend === 'Graph') {
    const { nodes, links } = await genGraph({ name: '__root__' })
    resetChart({ nodes: (graphNodes = nodes), links: (graphLinks = links) })
    nodesSet.clear()
    return
  }
  for (const map of treeNodeMap.values()) map.collapsed = true
  resetChart({ tree })
  treeNodeMap.clear()
}

export async function dealGraphNode(name: string) {
  if (name === '__root__' || !relations[name]) return
  const { nodes, links } = await genGraph({ name })
  if (!nodes.length) return
  const linkHad = new Map<string, Set<string>>()
  for (let i = 0; i < graphLinks.length; i++) {
    const { source, target } = graphLinks[i]
    const link = linkHad.get(source)
    if (!link) linkHad.set(source, new Set([target]))
    else link.add(target)
  }
  if (nodesSet.has(name)) {
    nodesSet.delete(name)
    const nodeHad = new Set(nodes.map((node) => node.name))
    graphNodes = graphNodes.filter(
      ({ name: _name }) =>
        // eslint-disable-next-line @stylistic/implicit-arrow-linebreak
        !nodeHad.has(_name) || linkHad.get(_name)!.size > 1 || _name === name,
    )
  } else {
    nodesSet.add(name)
    const nodeHad = new Set(graphNodes.map((node) => node.name))
    graphLinks.push(
      ...links.filter(
        ({ source, target }) => !linkHad.get(source)?.has(target),
      ),
    )
    graphNodes.push(...nodes.filter(({ name }) => !nodeHad.has(name)))
  }
  resetChart({ nodes: graphNodes, links: graphLinks })
}

export function dealTreeNode(data: any, collapsed: boolean, ancestors?: any) {
  // if (collapsed) {
  //   const node = treeNodeMap.get(data.name)
  //   node && (node.collapsed = true)
  //   treeNodeMap.delete(data.name)
  //   return
  // }
  // const { dependencies = {}, devDependencies } =
  //   relations[formatName(data.name)] ?? {}
  // const pkg = Object.assign(dependencies, devDependencies)
  // if (isEmptyObj(pkg) || data.children.length) return
  // let child = tree.children
  // for (let i = 2; i < ancestors.length; i++) {
  //   const item = child.find((item) => item.name === ancestors[i].name)!
  //   item.collapsed = false
  //   treeNodeMap.set(item.name, item)
  //   child = item.children
  // }
  // child.push(
  //   ...Object.entries(pkg).map(([name, value]) => ({
  //     // echarts 对相同名字的标签会动画重叠，这里用 -- 区分一下
  //     name: `${name}--${data.name}`,
  //     value,
  //     children: [],
  //   })),
  // )
  resetChart({ tree })
}

export function toggleChart(legend: Legend) {
  const isGraph = legend === 'Graph'
  echart.setOption(isGraph ? loadTree(tree) : loadGraph(graphNodes, graphLinks))
  return isGraph ? 'Tree' : 'Graph'
}

export function getPkgInfo(name: string): PkgInfo {
  // return {
  //   info:
  //     relations[name] ??
  //     Object.values(relations).find((val) => val.name?.includes(name)),
  //   extra: relations.__extra__[name],
  //   circulation: circulation[name],
  //   versions: versions[name],
  // }
}

function resetChart(data: { tree?: Tree; nodes?: Nodes[]; links?: Links[] }) {
  echart.setOption({
    series: data.tree
      ? { name: 'Tree', data: [data.tree] }
      : { name: 'Force', ...data },
  })
}

export function formatName(name: string) {
  return name.split('_')[0]
}

export function download() {
  const el = document.createElement('a')
  el.href = echart.getDataURL()
  el.download = 'ecahrts'
  const event = new MouseEvent('click')
  el.dispatchEvent(event)
}