import React, { ReactNode } from 'react';
import { IRouteComponentProps } from 'umi';
import { Responsive, WidthProvider } from 'react-grid-layout';
import { useIntl, history } from 'umi';
import { Text } from 'office-ui-fabric-react/lib/Text';
import { Nav, INavLink, INavLinkGroup } from 'office-ui-fabric-react/lib/Nav';

const ResponsiveGridLayout = WidthProvider(Responsive);

interface ILink {
  icon: string;
  label: string;
  path: string;
}

const sharedLinks: ILink[] = [
  { icon: 'SignIn', label: 'users.sign-in.title', path: '/users/sign-in' },
  {
    icon: 'TemporaryUser',
    label: 'users.sign-up.title',
    path: '/users/sign-up',
  },
  {
    icon: 'WaitlistConfirm',
    label: 'users.confirm.title',
    path: '/users/confirm',
  },
  { icon: 'Unlock', label: 'users.unlock.title', path: '/users/unlock' },
  {
    icon: 'SignIn',
    label: 'users.forgot-password.title',
    path: '/users/forgot-password',
  },
  { icon: 'Message', label: 'leave-words.new.title', path: '/leave-words/new' },
];

export interface IProps {
  children: ReactNode;
}

export default function({ children }: IProps) {
  const intl = useIntl();
  return (
    <ResponsiveGridLayout cols={{ lg: 12, md: 10, sm: 6, xs: 4, xxs: 2 }}>
      <div
        key="left"
        data-grid={{ x: 0, y: 0, w: 4, h: 0, minH: 0, maxH: 1, static: true }}
      />
      <div key="main" data-grid={{ x: 4, y: 0, w: 4, h: 1, static: true }}>
        {children}
        <br />
        <Nav
          onLinkClick={(ev, item) => {
            ev?.preventDefault();
            history.push(item?.url);
          }}
          onRenderLink={it => <Text>{it?.name}</Text>}
          groups={[
            {
              links: sharedLinks.map(it => {
                return {
                  icon: it.icon,
                  key: it.path,
                  name: intl.formatMessage({ id: it.label }),
                  url: it.path,
                };
              }),
            },
          ]}
        />
      </div>
      <div
        key="right"
        data-grid={{ x: 8, y: 0, w: 4, h: 0, minH: 0, maxH: 1, static: true }}
      />
    </ResponsiveGridLayout>
  );
}
