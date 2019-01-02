import { combineReducers } from 'redux'

import { IUserState } from '../actions'
import { currentUser } from './currentUser'

export interface IApplicationState {
    user: IUserState,
}

export const rootReducers = combineReducers<IApplicationState>({
    user: currentUser
})