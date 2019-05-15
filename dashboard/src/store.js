import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export const USERS_SIGN_IN = 'users.sign-in'
export const USERS_SIGN_OUT = 'users.sign-out'

export default new Vuex.Store({
    state: {
        user: null,
    },
    mutations: {
        [USERS_SIGN_IN](state, user) {
            state.user = user
        },
        [USERS_SIGN_OUT](state) {
            state.user = null
        }
    }
})