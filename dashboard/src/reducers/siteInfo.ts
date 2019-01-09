import { Reducer } from 'redux'

import { ISiteState, SiteActionTypes } from '../actions'

const initialState: ISiteState = {}

export const siteInfo: Reducer<ISiteState> = (state = initialState, action) => {
    switch (action.type) {
        case SiteActionTypes.REFRESH:
            return { ...state, ...action.payload }
        default:
            return state
    }
}

