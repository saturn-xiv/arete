const KEY = 'token'

export interface IToken {
  uid: string,
  act: string,
  exp: number,
  nbf: number,
  roles: string[],
}

export const get = () => {
  return sessionStorage.getItem(KEY)
}

export const set = (token: string) => {
  sessionStorage.setItem(KEY, token)
}

export const remove = () => {
  sessionStorage.removeItem(KEY)
}
