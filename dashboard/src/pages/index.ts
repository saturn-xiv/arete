interface IPage {
  path: string,
  component: any
}

const routes: IPage[] = [{
  component: () => import('./attachments/Index'),
  path: '/attachments',
}, {
  component: () => import('./attachments/New'),
  path: '/attachments/new',
}, {
  component: () => import('./users/SignIn'),
  path: '/users/sign-in',
}, {
  component: () => import('./users/SignUp'),
  path: '/users/sign-up',
}, {
  component: () => import('./users/Confirm'),
  path: '/users/confirm',
}, {
  component: () => import('./users/Unlock'),
  path: '/users/unlock',
}, {
  component: () => import('./users/ForgotPassword'),
  path: '/users/forgot-password',
}, {
  component: () => import('./users/ResetPassword'),
  path: '/users/reset-password/:token',
}, {
  component: () => import('./users/Logs'),
  path: '/users/logs',
}, {
  component: () => import('./users/ChangePassword'),
  path: '/users/change-password',
}, {
  component: () => import('./users/Profile'),
  path: '/users/profile',
}, {
  component: () => import('./leave-words/New'),
  path: '/leave-words/new',
}, {
  component: () => import('./Install'),
  path: '/install',
}, {
  component: () => import('./admin/leave-words/Index'),
  path: '/admin/leave-words',
}, {
  component: () => import('./admin/locales/Index'),
  path: '/admin/locales',
}, {
  component: () => import('./admin/locales/Form'),
  path: '/admin/locales/new',
}, {
  component: () => import('./admin/locales/Form'),
  path: '/admin/locales/:id/edit',
}, {
  component: () => import('./admin/site/Status'),
  path: '/admin/site/status',
}, {
  component: () => import('./admin/site/Info'),
  path: '/admin/site/info',
}, {
  component: () => import('./admin/site/Author'),
  path: '/admin/site/author',
}, {
  component: () => import('./admin/site/Seo'),
  path: '/admin/site/seo',
}, {
  component: () => import('./admin/site/Smtp'),
  path: '/admin/site/smtp',
}, {
  component: () => import('./admin/tags/Index'),
  path: '/admin/tags',
}, {
  component: () => import('./admin/tags/Form'),
  path: '/admin/tags/new',
}, {
  component: () => import('./admin/tags/Form'),
  path: '/admin/tags/:id/edit',
}, {
  component: () => import('./admin/categories/Index'),
  path: '/admin/categories',
}, {
  component: () => import('./admin/categories/Form'),
  path: '/admin/categories/new',
}, {
  component: () => import('./admin/categories/Form'),
  path: '/admin/categories/:id/edit',
}, {
  component: () => import('./admin/cards/Index'),
  path: '/admin/cards',
}, {
  component: () => import('./admin/cards/Form'),
  path: '/admin/cards/new',
}, {
  component: () => import('./admin/cards/Form'),
  path: '/admin/cards/:id/edit',
}, {
  component: () => import('./admin/links/Index'),
  path: '/admin/links',
}, {
  component: () => import('./admin/links/Form'),
  path: '/admin/links/new',
}, {
  component: () => import('./admin/links/Form'),
  path: '/admin/links/:id/edit',
}, {
  component: () => import('./admin/friend-links/Index'),
  path: '/admin/friend-links',
}, {
  component: () => import('./admin/friend-links/Form'),
  path: '/admin/friend-links/new',
}, {
  component: () => import('./admin/friend-links/Form'),
  path: '/admin/friend-links/:id/edit',
}, {
  component: () => import('./admin/votes/Index'),
  path: '/admin/votes',
}, {
  component: () => import('./admin/users/Index'),
  path: '/admin/users',
}, {
  component: () => import('./admin/users/Authority'),
  path: '/admin/users/:id/authority',
}, {
  component: () => import('./Home'),
  path: '/',
}]

export default routes
