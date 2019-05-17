import UsersEdit from './users/Edit'
import UsersNew from './users/New'
import UsersChangePassword from './users/ChangePassword'
import UsersIndex from './users/Index'
import LogsIndex from './logs/Index'
import Settings from './Settings'
import Status from './Status'

export default {
    routes: [{
        path: '/ops/vpn/users/new',
        name: 'ops.vpn.users.new',
        component: UsersNew,
    }, {
        path: '/ops/vpn/users/change-password',
        name: 'ops.vpn.users.change-password',
        component: UsersChangePassword,
    }, {
        path: '/ops/vpn/users/:id/edit',
        name: 'ops.vpn.users.edit',
        component: UsersEdit,
    }, {
        path: '/ops/vpn/users',
        name: 'ops.vpn.users.index',
        component: UsersIndex,
    }, {
        path: '/ops/vpn/logs',
        name: 'ops.vpn.logs.index',
        component: LogsIndex,
    }, {
        path: '/ops/vpn/settings',
        name: 'ops.vpn.settings',
        component: Settings,
    }, {
        path: '/ops/vpn/status',
        name: 'ops.vpn.status',
        component: Status,
    }]
}