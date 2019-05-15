import Vue from 'vue'
import VueRouter from 'vue-router'

import nut from './nut'
import ops_vpn from './ops/vpn'

Vue.use(VueRouter)

const plugins = [ops_vpn, nut]

export const router = new VueRouter({
    base: '/my/',
    mode: 'history',
    routes: plugins.reduce((a, i) => a.concat(i.routes), [])
})