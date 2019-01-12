import { Icon, List } from 'antd'
import * as React from 'react'
import { FormattedMessage } from 'react-intl'
import { Link } from "react-router-dom"

import Layout from '../form/Layout'

interface IProps {
  title: string,
}

interface ILink {
  icon: string,
  label: string,
  to: string,
}

class Widget extends React.Component<IProps> {
  public render() {
    return (<Layout title={{ id: this.props.title }}>
      {this.props.children}
      <List
        size="small"
        bordered={true}
        dataSource={[
          { icon: 'login', label: 'nut.users.sign-in.title', to: '/users/sign-in' },
          { icon: 'user-add', label: 'nut.users.sign-up.title', to: '/users/sign-up' },
          { icon: 'key', label: 'nut.users.forgot-password.title', to: '/users/forgot-password' },
          { icon: 'safety', label: 'nut.users.confirm.title', to: '/users/confirm' },
          { icon: 'unlock', label: 'nut.users.unlock.title', to: '/users/unlock' },
          { icon: 'message', label: 'nut.leave-words.new.title', to: '/leave-words/new' },
        ]}
        renderItem={(it: ILink) => (<List.Item>
          <Icon type={it.icon} />
          &nbsp;
          <Link to={it.to}>
            <FormattedMessage id={it.label} />
          </Link>
        </List.Item>)}
      />
    </Layout>)
  }
}

export default Widget
