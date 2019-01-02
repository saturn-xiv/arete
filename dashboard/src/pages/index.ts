interface IPage {
    path: string,
    component: any
}

const routes: IPage[] = [{
    component: () => import('./users/SignIn'),
    path: '/users/sign-in',
}, {
    component: () => import('./users/SignUp'),
    path: '/users/sign-up',
}, {
    component: () => import('./Install'),
    path: '/install',
}, {
    component: () => import('./Home'),
    path: '/',
}]

export default routes

