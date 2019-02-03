import { Button, message } from 'antd'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'
import { RouteComponentProps, withRouter } from 'react-router'

import { Authorized, RoleTypes } from '../../../components/authorized'
import { ACTION_WIDTH, TIMESTAMP_WIDTH } from '../../../components/form'
import Timestamp from '../../../components/moment/Timestamp'
import ActionColumn from '../../../components/table/action/Column'
import Layout from '../../../components/table/Layout'
import { httpGet } from '../../../utils/request'

export interface IItem {
  id: number,
  nickName: string,
  realName: string,
  email: string,
  providerType: string,
  currentSignInAt: string,
  currentSignInIp: string,
  lastSignInAt: string,
  lastSignInIp: string,
  signInCount: number,
}

interface IState {
  items: IItem[],
}

class Widget extends React.Component<RouteComponentProps<any> & InjectedIntlProps, IState> {
  public static propTypes: React.ValidationMap<RouteComponentProps<any> & InjectedIntlProps> = {
    intl: intlShape.isRequired,
  }
  constructor(props: any) {
    super(props)
    this.state = {
      items: [],
    }
  }

  public componentDidMount() {
    httpGet(`/admin/users`).then((rst) => {
      this.setState({ items: rst })
    }).catch(message.error)
  }
  public render() {
    const { history } = this.props
    const columns = [{
      dataIndex: 'signInCount',
      key: 'signInCount',
      title: (<FormattedMessage id="nut.models.user.sign-in.count" />),
    }, {
      key: 'user',
      render: (v: IItem) => `${v.nickName}, ${v.realName}<${v.email}>`,
      title: (<FormattedMessage id="form.labels.user" />),
    }, {
      key: 'last',
      render: (v: IItem) => `${v.lastSignInAt}[${v.lastSignInIp}]`,
      title: (<FormattedMessage id="nut.models.user.sign-in.last" />),
    }, {
      key: 'current',
      render: (v: IItem) => `${v.currentSignInAt}[${v.currentSignInIp}]`,
      title: (<FormattedMessage id="nut.models.user.sign-in.current" />),
    }, {
      dataIndex: 'updatedAt',
      key: 'updatedAt',
      render: (v: Date) => (<Timestamp date={v} />),
      title: (<FormattedMessage id="form.labels.updated-at" />),
      width: TIMESTAMP_WIDTH,
    }, {
      key: 'action',
      render: (it: IItem) => (<Button onClick={() => history.push(`/admin/users/${it.id}/authority`)} icon="idcard" type="dashed" />),
      title: (<ActionColumn />),
      width: ACTION_WIDTH,
    }]
    return (<Authorized authority={RoleTypes.ADMIN} >
      <Layout
        rowKey="id"
        columns={columns}
        data={this.state.items}
        title={{ id: 'nut.admin.tags.index.title' }} />
    </Authorized>)
  }
}

export default withRouter(injectIntl(Widget))
