import UsersEdit from './users/Edit'

export default {
    routes: [{
        path: '/users/:id/edit',
        name: 'vpn.users.edit',
        component: UsersEdit,
    }]
}