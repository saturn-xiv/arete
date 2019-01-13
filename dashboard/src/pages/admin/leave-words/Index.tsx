import { message } from 'antd'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

import { MediaType } from '../../../components'
import { Authorized } from '../../../components/authorized'
import Content from '../../../components/Content'
import Delete from '../../../components/Delete'
import Timestamp from '../../../components/moment/Timestamp'
import Table from '../../../components/Table'
import { httpDelete, httpGet } from '../../../utils/request'

interface IItem {
  id: number,
  ip: string
  body: string,
  mediaType: MediaType,
  createdAt: Date,
}

interface IState {
  items: IItem[],
}

class Widget extends React.Component<InjectedIntlProps, IState> {
  public static propTypes: React.ValidationMap<InjectedIntlProps> = {
    intl: intlShape.isRequired,
  }
  constructor(props: any) {
    super(props)
    this.state = {
      items: [],
    }
  }
  public handleRemove = (id: number) => {
    const { formatMessage } = this.props.intl
    httpDelete(`/admin/leave-words/${id}`).then(() => {
      message.success(formatMessage({ id: 'flashes.success' }))
      const items = this.state.items.filter((it) => it.id !== id)
      this.setState({ items })
    }).catch(message.error)
  }
  public componentDidMount() {
    httpGet(`/admin/leave-words`).then((rst) => {
      this.setState({ items: rst })
    }).catch(message.error)
  }
  public render() {
    const columns = [{
      dataIndex: 'id',
      key: 'id',
      title: (<FormattedMessage id="form.labels.id" />),
      width: 80,
    }, {
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
      key: 'body',
      render: (it: IItem) => (<Content mediaType={it.mediaType} body={it.body} />),
      title: (<FormattedMessage id="form.labels.body" />),
    }, {
      key: 'action',
      render: (it: IItem) => (<Delete id={it.id} title={{ id: 'nut.admin.leave-words.index.confirm', values: { id: it.id } }} onRemove={this.handleRemove} />),
      title: (<FormattedMessage id="buttons.action" />),
      width: 50,
    }]
    return (<Authorized>
      <Table
        rowKey="id"
        columns={columns}
        data={this.state.items}
        title={{ id: 'nut.admin.leave-words.index.title' }} />
    </Authorized>)
  }
}

export default injectIntl(Widget)
