import { action } from 'typesafe-actions'

export const enum UserActionTypes {
    SIGN_IN = '@@users/sign-in',
    SIGN_OUT = '@@users/sign-out',
}

export const enum SiteActionTypes {
    REFRESH = '@@site/refresh',
}

export interface IUserState {
    readonly uid?: string,
}

export interface ISiteState {
    readonly version?: string,
    readonly languages?: string[],
}

export const signIn = (token: string) => action(UserActionTypes.SIGN_IN, token)

export const signOut = () => action(UserActionTypes.SIGN_OUT)

export const refresh = (info: ISiteState) => action(SiteActionTypes.REFRESH, info)
