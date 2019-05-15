import UsersSignIn from './users/SignIn'
import UsersSignUp from './users/SignUp'
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
        },
        {
            path: '/install',
            name: 'install',
            component: Install,
        }, {
            path: '/',
            name: 'home',
            component: Home,
        }
    ]
}