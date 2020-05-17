import React from 'react';
import { IRouteComponentProps } from 'umi';
import Application from './Application';

const non_sign_in = function(path: string): boolean {
  if (
    [
      '/install',
      '/users/sign-in',
      '/users/sign-up',
      '/users/confirm',
      '/users/unlock',
      '/users/forgot-password',
      '/leave-words/new',
      '/404',
    ].includes(path)
  ) {
    return true;
  }
  if (path.startsWith('/users/') && path.split('/').length === 4) {
    for (let it of ['confirm', 'unlock', 'reset-password']) {
      if (path.endsWith(`/${it}`)) {
        return true;
      }
    }
  }
  return false;
};

export default function({ children, location }: IRouteComponentProps) {
  return non_sign_in(location.pathname) ? (
    <Application>{children}</Application>
  ) : (
    <div>
      <h1>dashboard header</h1>
      {children}
      <h2>dashboard footer</h2>
    </div>
  );
}
