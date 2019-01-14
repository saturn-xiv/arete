import { message } from 'antd'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

import { Authorized, RoleTypes } from '../../../components/authorized'
import Timestamp from '../../../components/moment/Timestamp'
import ActionCell from '../../../components/table/action/Cell'
import ActionColumn from '../../../components/table/action/Column'
import Layout from '../../../components/table/Layout'
import { httpDelete, httpGet } from '../../../utils/request'

export interface IItem {
  id: number,
  lang: string,
  code: string
  message: string,
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
    httpDelete(`/admin/locales/${id}`).then(() => {
      message.success(formatMessage({ id: 'flashes.success' }))
      const items = this.state.items.filter((it) => it.id !== id)
      this.setState({ items })
    }).catch(message.error)
  }
  public componentDidMount() {
    httpGet(`/admin/locales`).then((rst) => {
      this.setState({ items: rst })
    }).catch(message.error)
  }
  public render() {
    const columns = [{
      dataIndex: 'lang',
      key: 'lang',
      render: (v: string) => (<FormattedMessage id={`languages.${v}`} />),
      title: (<FormattedMessage id="nut.models.locale.lang" />),
      width: 120,
    }, {
      dataIndex: 'code',
      key: 'code',
      title: (<FormattedMessage id="nut.models.locale.code" />),
    }, {
      dataIndex: 'updatedAt',
      key: 'updatedAt',
      render: (v: Date) => (<Timestamp date={v} />),
      title: (<FormattedMessage id="form.labels.updated-at" />),
      width: 280,
    }, {
      key: 'action',
      render: (it: IItem) => (<ActionCell toEdit={`/admin/locales/${it.id}/edit`} confirmRemove={{ id: 'nut.admin.locales.index.confirm', values: { code: it.code, lang: it.lang } }} onRemove={() => this.handleRemove(it.id)} />),
      title: (<ActionColumn to="/admin/locales/new" />),
      width: 120,
    }]
    return (<Authorized authority={RoleTypes.ADMIN}>
      <Layout
        rowKey="id"
        columns={columns}
        data={this.state.items}
        expandedRowRender={(it: IItem) => (<>{it.message}</>)}
        title={{ id: 'nut.admin.locales.index.title' }} />
    </Authorized>)
  }
}

export default injectIntl(Widget)
