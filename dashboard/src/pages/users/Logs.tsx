import { message } from 'antd'
import * as React from 'react'
import { FormattedMessage } from 'react-intl'

import { Authorized } from '../../components/authorized'
import Timestamp from '../../components/moment/Timestamp'
import Layout from '../../components/table/Layout'
import { httpGet } from '../../utils/request'

interface IItem {
  id: number,
  message: string,
  ip: string,
  createdAt: Date,
}

interface IState {
  items: IItem[],
}

class Widget extends React.Component<any, IState> {
  constructor(props: any) {
    super(props)
    this.state = {
      items: [],
    }
  }
  public componentDidMount() {
    httpGet(`/users/logs`).then((rst) => {
      this.setState({ items: rst })
    }).catch(message.error)
  }
  public render() {
    const columns = [{
      dataIndex: 'createdAt',
      key: 'createdAt',
      render: (v: Date) => (<Timestamp date={v} />),
      title: (<FormattedMessage id="form.labels.created-at" />),
      width: 280,
    }, {
      dataIndex: 'ip',
      key: 'ip',
      title: (<FormattedMessage id="form.labels.ip" />),
      width: 120,
    }, {
      dataIndex: 'message',
      key: 'message',
      title: (<FormattedMessage id="form.labels.message" />),
    }]
    return (<Authorized>
      <Layout rowKey="id" columns={columns} data={this.state.items} title={{ id: 'nut.users.logs.title' }} />
    </Authorized>)
  }
}

export default Widget
