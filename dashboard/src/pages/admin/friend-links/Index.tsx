import { message } from 'antd'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

import { Authorized, RoleTypes } from '../../../components/authorized'
import { ACTION_WIDTH, TIMESTAMP_WIDTH } from '../../../components/form'
import Timestamp from '../../../components/moment/Timestamp'
import ActionCell from '../../../components/table/action/Cell'
import ActionColumn from '../../../components/table/action/Column'
import Layout from '../../../components/table/Layout'
import { httpDelete, httpGet } from '../../../utils/request'

export interface IItem {
  id: number,
  title: string,
  home: string,
  logo: string,
  position: number,
  updatedAt: Date,
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
    httpDelete(`/admin/friend-links/${id}`).then(() => {
      message.success(formatMessage({ id: 'flashes.success' }))
      const items = this.state.items.filter((it) => it.id !== id)
      this.setState({ items })
    }).catch(message.error)
  }
  public componentDidMount() {
    httpGet(`/admin/friend-links`).then((rst) => {
      this.setState({ items: rst })
    }).catch(message.error)
  }
  public render() {
    const columns = [{
      dataIndex: 'position',
      key: 'position',
      title: (<FormattedMessage id="form.labels.position" />),
    }, {
      key: 'content',
      render: (v: IItem) => (<a href={v.home} target="_blank">{v.title}</a>),
      title: (<FormattedMessage id="form.labels.content" />),
    }, {
      dataIndex: 'updatedAt',
      key: 'updatedAt',
      render: (v: Date) => (<Timestamp date={v} />),
      title: (<FormattedMessage id="form.labels.updated-at" />),
      width: TIMESTAMP_WIDTH,
    }, {
      key: 'action',
      render: (it: IItem) => (<ActionCell toEdit={`/admin/friend-links/${it.id}/edit`} confirmRemove={{ id: 'nut.admin.friend-links.index.confirm', values: { title: it.title } }} onRemove={() => this.handleRemove(it.id)} />),
      title: (<ActionColumn to="/admin/friend-links/new" />),
      width: ACTION_WIDTH,
    }]
    return (<Authorized authority={RoleTypes.ADMIN}>
      <Layout
        rowKey="id"
        columns={columns}
        data={this.state.items}
        title={{ id: 'nut.admin.friend-links.index.title' }} />
    </Authorized>)
  }
}

export default injectIntl(Widget)
