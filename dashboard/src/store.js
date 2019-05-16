import Vue from 'vue'
import Vuex from 'vuex'
import * as jwtDecode from 'jwt-decode'

import {
    remove as remoteToken
} from './token'

Vue.use(Vuex)

export const USERS_SIGN_IN = 'users.sign-in'
export const USERS_SIGN_OUT = 'users.sign-out'
export const NOTIFICATION_SUCCESS = 'notification.success'
export const NOTIFICATION_ERROR = 'notification.error'
export const NOTIFICATION_INFO = 'notification.info'

export default new Vuex.Store({
    state: {
        user: null,
        notification: {
            color: null,
            message: null,
        }
    },
    mutations: {
        [USERS_SIGN_IN](state, token) {
            try {
                const it = jwtDecode(token)
                const now = new Date().getTime() / 1000
                if (it.act === "signIn" && it.nbf < now && it.exp > now) {
                    state.user = it.uid
                    return
                }
            } catch (e) {
                window.console.error(e)
            }
            remoteToken()
            state.user = null
        },
        [USERS_SIGN_OUT](state) {
            remoteToken()
            state.user = null
        },
        [NOTIFICATION_SUCCESS](state, message) {
            state.notification = {
                color: 'success',
                message
            }
        },
        [NOTIFICATION_ERROR](state, message) {
            state.notification = {
                color: 'error',
                message
            }
        },
        [NOTIFICATION_INFO](state, message) {
            state.notification = {
                color: 'info',
                message
            }
        }
    },
    getters: {
        isSignIn: state => {
            return state.user != null
        },
        notification: state => {
            return state.notification
        }
    }
})