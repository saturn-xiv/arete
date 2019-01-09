import { combineReducers } from 'redux'

import { ISiteState, IUserState } from '../actions'
import { currentUser } from './currentUser'
import { siteInfo } from './siteInfo'

export interface IApplicationState {
    user: IUserState,
    site: ISiteState,
}

export const rootReducers = combineReducers<IApplicationState>({
    site: siteInfo,
    user: currentUser,
})