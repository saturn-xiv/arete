import Vue from 'vue'
import Vuex from 'vuex'
import * as jwtDecode from 'jwt-decode'

import {
    set as setToken,
    remove as remoteToken
} from './token'

Vue.use(Vuex)

export const USERS_SIGN_IN = 'users.sign-in'
export const USERS_SIGN_OUT = 'users.sign-out'

export default new Vuex.Store({
    state: {
        user: null,
    },
    mutations: {
        [USERS_SIGN_IN](state, token) {
            try {
                const it = jwtDecode(token)
                const now = new Date().getTime() / 1000
                if (it.act === "signIn" && it.nbf < now && it.exp > now) {
                    setToken(token)
                    state.user = it.uid
                }
            } catch (e) {
                console.error(e)
                remoteToken()
                state.user = null
            }
        },
        [USERS_SIGN_OUT](state) {
            state.user = null
        }
    },
    getters: {
        isSignIn: state => {
            return state.user != null
        }
    }
})