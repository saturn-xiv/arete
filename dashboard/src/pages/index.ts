interface IPage {
  path: string,
  component: any
}

const routes: IPage[] = [
  //     {
  //     component: () => import('./users/SignIn'),
  //     path: '/users/sign-in',
  // }, {
  //     component: () => import('./users/SignUp'),
  //     path: '/users/sign-up',
  // }, {
  //     component: () => import('./users/Confirm'),
  //     path: '/users/confirm',
  // }, {
  //     component: () => import('./users/Unlock'),
  //     path: '/users/unlock',
  // }, {
  //     component: () => import('./users/ForgotPassword'),
  //     path: '/users/forgot-password',
  // }, {
  //     component: () => import('./users/ResetPassword'),
  //     path: '/users/reset-password/:token',
  // }, {
  //     component: () => import('./leave-words/New'),
  //     path: '/leave-words/new',
  // }, {
  //     component: () => import('./Install'),
  //     path: '/install',
  // },
  {
    component: () => import('./Home'),
    path: '/',
  }]

export default routes
