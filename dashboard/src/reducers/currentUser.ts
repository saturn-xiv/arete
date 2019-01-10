import { Reducer } from 'redux'

import { IUserState, UserActionTypes } from '../actions'

const initialState: IUserState = {}

export const currentUser: Reducer<IUserState> = (state = initialState, action) => {
  switch (action.type) {
    case UserActionTypes.SIGN_IN:
      // TODO
      window.console.log(action.payload)
      return { ...state, uid: 'aaa' }
    case UserActionTypes.SIGN_OUT:
      // TODO
      return {}
    default:
      return state
  }
}
