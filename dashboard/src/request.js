import {
    get as getToken
} from './token'

export const backend = (u) => `/api${u}`

export const options = (method) => {
    return {
        credentials: 'include',
        headers: {
            'Authorization': `Bearer ${getToken()}`,
            'Content-Type': 'application/json; charset=utf-8',
        },
        method,
    }
}

export const get = (path) => fetch(backend(path), options('GET')).then((res) => res.ok ?
    res.json() :
    res.text().then(err => {
        throw err
    }))


export const delete_ = (path) => fetch(backend(path), options('DELETE')).then((res) => res.ok ?
    res.json() :
    res.text().then(err => {
        throw err
    }))


// https://github.github.io/fetch/#options
export const post = (path, body) => {
    const data = options('POST')
    data.body = JSON.stringify(body)
    return fetch(backend(path), data).then((res) => res.ok ?
        res.json() :
        res.text().then(err => {
            throw err
        }))
}

export const patch = (path, body) => {
    const data = options('PATCH')
    data.body = JSON.stringify(body)
    return fetch(backend(path), data).then((res) => res.ok ?
        res.json() :
        res.text().then(err => {
            throw err
        }))
}

export const put = (path, body) => {
    const data = options('PUT')
    data.body = JSON.stringify(body)
    return fetch(backend(path), data).then((res) => res.ok ?
        res.json() :
        res.text().then(err => {
            throw err
        }))
}