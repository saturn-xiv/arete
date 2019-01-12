import * as jwtDecode from 'jwt-decode'
import { Reducer } from 'redux'

import { IUserState, UserActionTypes } from '../actions'
import { IToken, remove as removeToken, set as setToken } from '../utils/token'

const initialState: IUserState = { roles: [] }

export const currentUser: Reducer<IUserState> = (state = initialState, action) => {
  switch (action.type) {
    case UserActionTypes.SIGN_IN:
      try {
        const token: IToken = jwtDecode(action.payload)
        const now = new Date().getTime() / 1000
        if (token.act === "SignIn" && token.nbf < now && token.exp > now) {
          setToken(action.payload)
          return { ...state, uid: token.uid, roles: token.roles }
        }
      } catch (e) {
        window.console.error(e)
      }
      removeToken()
      return { roles: [] }
    case UserActionTypes.SIGN_OUT:
      removeToken()
      return { roles: [] }
    default:
      return state
  }
}
