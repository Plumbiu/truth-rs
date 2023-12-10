import type { ECharts } from 'echarts/core'
import type { Legend, Links, Nodes, Relation, Relations, Tree } from '../types'
import { loadGraph, loadTree, genGraph, isEmptyObj } from './tools'

const apiUrl = import.meta.env.DEV ? 'http://localhost:8080/api/' : '/api/'

export async function request<T>(rest: string) {
  const data = await fetch(apiUrl + rest).then((res) => res.json())
  return data as T
}

let echart: ECharts
const treeNodeMap = new Map()
const nodesSet = new Set()
let relations: Relations
let graphNodes: Nodes[]
let graphLinks: Links[]
let tree: Tree

export async function initChart(_echart: ECharts) {
  const { nodes, links } = await genGraph({ name: '__root__', category: 1 })
  relations = await request('relations.json')
  tree = await request('tree.json')
  _echart.setOption(loadGraph((graphNodes = nodes), (graphLinks = links)))
  echart = _echart
}

export async function changeGraphRoot(name: string, isAim: boolean) {
  if (!relations[name]) return
  if (isAim) {
    resetChart({ nodes: graphNodes, links: graphLinks })
    return
  }
  const newNodes = [{ name, category: 0, value: relations[name].version! }]
  const { nodes, links } = await genGraph({ name, category: 2 })
  newNodes.push(...nodes.filter((node) => node.name !== name))
  resetChart({ nodes: newNodes, links })
}

export async function collapseNode(legend: Legend) {
  if (legend === 'Graph') {
    const { nodes, links } = await genGraph({ name: '__root__', category: 1 })
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
        !nodeHad.has(_name) ||
        (linkHad.get(_name)?.size ?? 0 > 1) ||
        _name === name,
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

// TODO: handle by rust
export function dealTreeNode(data: any, collapsed: boolean, ancestors?: any) {
  if (collapsed) {
    const node = treeNodeMap.get(data.name)
    node && (node.collapsed = true)
    treeNodeMap.delete(data.name)
    return
  }
  const { dependencies = {}, devDependencies } =
    relations[formatName(data.name)] ?? {}
  const pkg = Object.assign(dependencies, devDependencies)
  if (isEmptyObj(pkg) || data.children.length) return
  let child = tree.children
  for (let i = 2; i < ancestors.length; i++) {
    const item = child.find((item) => item.name === ancestors[i].name)!
    item.collapsed = false
    treeNodeMap.set(item.name, item)
    child = item.children
  }

  child.push(
    ...Object.values(pkg).map((value) => ({
      // echarts 对相同名字的标签会动画重叠，这里用 -- 区分一下
      name: formatName(data.id),
      value,
      children: [],
    })),
  )
  resetChart({ tree })
}

export function toggleChart(legend: Legend) {
  const isGraph = legend === 'Graph'
  echart.setOption(isGraph ? loadTree(tree) : loadGraph(graphNodes, graphLinks))
  return isGraph ? 'Tree' : 'Graph'
}

export async function getPkgInfo(name: string): Promise<Relation> {
  const key = relations[name]
    ? name
    : Object.keys(relations).find((item) => item.includes(name))

  const {
    name: relName,
    version,
    dependencies,
    devDependencies,
    homepage,
  } = await request<Relation>(`relations.json/query?name=${key}`)

  return {
    name: relName,
    version,
    homepage,
    dependencies,
    devDependencies,
  }
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
