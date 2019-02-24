import { message } from 'antd'

import { get as getToken } from './token'

interface IQuery {
  query: string,
}

export function graphql<T>(body: IQuery, handler: (v: T) => void) {
  return fetch('/graphql', {
    body: JSON.stringify(body),
    credentials: 'include',
    headers: {
      'Authorization': `Bearer ${getToken()}`,
      'Content-Type': 'application/json; charset=utf-8',
    },
    method: 'POST',
  }).then((res) => res.ok
    ? res.json()
    : res.text().then(err => {
      throw err
    })).then((rst) => handler(rst.data)).catch(message.error)
}

export const backend = (u: string) => `/api${u}`

export const options = (method: string): RequestInit => {
  return {
    credentials: 'include',
    headers: {
      'Authorization': `Bearer ${getToken()}`,
      'Content-Type': 'application/json; charset=utf-8',
    },
    method,
  }
}

export const httpGet = (path: string) => fetch(backend(path), options('GET')).then((res) => res.ok
  ? res.json()
  : res.text().then(err => {
    throw err
  }))


export const httpDelete = (path: string) => fetch(backend(path), options('DELETE')).then((res) => res.ok
  ? res.json()
  : res.text().then(err => {
    throw err
  }))


// https://github.github.io/fetch/#options
export const httpPost = (path: string, body: object) => {
  const data = options('POST')
  data.body = JSON.stringify(body)
  return fetch(backend(path), data).then((res) => res.ok
    ? res.json()
    : res.text().then(err => {
      throw err
    }))
}

export const httpPatch = (path: string, body: object) => {
  const data = options('PATCH')
  data.body = JSON.stringify(body)
  return fetch(backend(path), data).then((res) => res.ok
    ? res.json()
    : res.text().then(err => {
      throw err
    }))
}

export const httpPut = (path: string, body: any) => {
  const data = options('PUT')
  data.body = JSON.stringify(body)
  return fetch(backend(path), data).then((res) => res.ok
    ? res.json()
    : res.text().then(err => {
      throw err
    }))
}
