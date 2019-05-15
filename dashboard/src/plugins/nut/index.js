import UsersSignIn from './users/SignIn'
import Home from './Home'
import Install from './Install'

export default {
    routes: [{
            path: '/users/sign-in',
            component: UsersSignIn,
        },
        {
            path: '/install',
            component: Install,
        }, {
            path: '/',
            component: Home,
        }
    ]
}