const apiUrl = import.meta.env.DEV ? 'http://localhost:8080/api/' : '/api/'

export async function request(rest: string) {
  const data = await fetch(apiUrl + rest).then((res) => res.json())
  return data
}
