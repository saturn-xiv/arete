import UsersSignIn from './users/SignIn'
import UsersSignUp from './users/SignUp'
import UsersLogs from './users/Logs'
import UsersChangePassword from './users/ChangePassword'
import UsersProfile from './users/Profile'
import Home from './Home'
import Install from './Install'

export default {
    routes: [{
        path: '/users/sign-in',
        name: 'users.sign-in',
        component: UsersSignIn,
    }, {
        path: '/users/sign-up',
        name: 'users.sign-up',
        component: UsersSignUp,
    }, {
        path: '/users/logs',
        name: 'users.logs',
        component: UsersLogs,
    }, {
        path: '/users/change-password',
        name: 'users.change-password',
        component: UsersChangePassword,
    }, {
        path: '/users/profile',
        name: 'users.profile',
        component: UsersProfile,
    }, {
        path: '/install',
        name: 'install',
        component: Install,
    }, {
        path: '/',
        name: 'home',
        component: Home,
    }]
}