import { message } from 'antd'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

import { Authorized, RoleTypes } from '../../../components/authorized'
import { ACTION_WIDTH, ID_WIDTH, TIMESTAMP_WIDTH } from '../../../components/form'
import Timestamp from '../../../components/moment/Timestamp'
import ActionCell from '../../../components/table/action/Cell'
import ActionColumn from '../../../components/table/action/Column'
import Layout from '../../../components/table/Layout'
import { httpDelete, httpGet } from '../../../utils/request'

export interface IItem {
  id: number,
  parentId?: number,
  name: string,
  color: string,
  icon: string,
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
    httpDelete(`/admin/categories/${id}`).then(() => {
      message.success(formatMessage({ id: 'flashes.success' }))
      const items = this.state.items.filter((it) => it.id !== id)
      this.setState({ items })
    }).catch(message.error)
  }
  public componentDidMount() {
    httpGet(`/admin/categories`).then((rst) => {
      this.setState({ items: rst })
    }).catch(message.error)
  }
  public render() {
    const columns = [{
      dataIndex: 'id',
      key: 'id',
      title: (<FormattedMessage id="form.labels.id" />),
      width: ID_WIDTH,
    }, {
      dataIndex: 'name',
      key: 'name',
      title: (<FormattedMessage id="form.labels.name" />),
    }, {
      dataIndex: 'color',
      key: 'color',
      title: (<FormattedMessage id="form.labels.color" />),
    }, {
      dataIndex: 'icon',
      key: 'icon',
      title: (<FormattedMessage id="form.labels.icon" />),
    }, {
      dataIndex: 'position',
      key: 'position',
      title: (<FormattedMessage id="form.labels.position" />),
    }, {
      dataIndex: 'parentId',
      key: 'parent',
      title: (<FormattedMessage id="form.labels.parent" />),
    }, {
      dataIndex: 'updatedAt',
      key: 'updatedAt',
      render: (v: Date) => (<Timestamp date={v} />),
      title: (<FormattedMessage id="form.labels.updated-at" />),
      width: TIMESTAMP_WIDTH,
    }, {
      key: 'action',
      render: (it: IItem) => (<ActionCell toEdit={`/admin/categories/${it.id}/edit`} confirmRemove={{ id: 'nut.admin.categories.index.confirm', values: { name: it.name } }} onRemove={() => this.handleRemove(it.id)} />),
      title: (<ActionColumn to="/admin/categories/new" />),
      width: ACTION_WIDTH,
    }]
    return (<Authorized authority={RoleTypes.ADMIN}>
      <Layout
        rowKey="id"
        columns={columns}
        data={this.state.items}
        title={{ id: 'nut.admin.categories.index.title' }} />
    </Authorized>)
  }
}

export default injectIntl(Widget)
