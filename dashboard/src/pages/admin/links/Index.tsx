import { message } from 'antd'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

import { Authorized, RoleTypes } from '../../../components/authorized'
import { ACTION_WIDTH, LANGUAGE_WIDTH, TIMESTAMP_WIDTH } from '../../../components/form'
import Timestamp from '../../../components/moment/Timestamp'
import ActionCell from '../../../components/table/action/Cell'
import ActionColumn from '../../../components/table/action/Column'
import Layout from '../../../components/table/Layout'
import { httpDelete, httpGet } from '../../../utils/request'

export interface IItem {
  id: number,
  label: string,
  href: string,
  loc: string,
  lang: string,
  x: number,
  y: number,
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
    httpDelete(`/admin/links/${id}`).then(() => {
      message.success(formatMessage({ id: 'flashes.success' }))
      const items = this.state.items.filter((it) => it.id !== id)
      this.setState({ items })
    }).catch(message.error)
  }
  public componentDidMount() {
    httpGet(`/admin/links`).then((rst) => {
      this.setState({ items: rst })
    }).catch(message.error)
  }
  public render() {
    const columns = [{
      dataIndex: 'lang',
      key: 'lang',
      render: (v: string) => (<FormattedMessage id={`languages.${v}`} />),
      title: (<FormattedMessage id="form.labels.lang" />),
      width: LANGUAGE_WIDTH,
    }, {
      key: 'content',
      render: (v: IItem) => (<a href={v.href} target="_blank">{v.label}</a>),
      title: (<FormattedMessage id="form.labels.content" />),
    }, {
      key: 'loc',
      render: (v: IItem) => `${v.loc}[${v.x}, ${v.y}]`,
      title: (<FormattedMessage id="form.labels.loc" />),
    }, {
      dataIndex: 'updatedAt',
      key: 'updatedAt',
      render: (v: Date) => (<Timestamp date={v} />),
      title: (<FormattedMessage id="form.labels.updated-at" />),
      width: TIMESTAMP_WIDTH,
    }, {
      key: 'action',
      render: (it: IItem) => (<ActionCell toEdit={`/admin/links/${it.id}/edit`} confirmRemove={{ id: 'nut.admin.links.index.confirm', values: { label: it.label } }} onRemove={() => this.handleRemove(it.id)} />),
      title: (<ActionColumn to="/admin/links/new" />),
      width: ACTION_WIDTH,
    }]
    return (<Authorized authority={RoleTypes.ADMIN}>
      <Layout
        rowKey="id"
        columns={columns}
        data={this.state.items}
        title={{ id: 'nut.admin.links.index.title' }} />
    </Authorized>)
  }
}

export default injectIntl(Widget)
