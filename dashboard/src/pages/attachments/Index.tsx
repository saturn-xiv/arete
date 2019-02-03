import { message } from 'antd'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

import { Authorized } from '../../components/authorized'
import { ACTION_WIDTH, TIMESTAMP_WIDTH } from '../../components/form'
import Timestamp from '../../components/moment/Timestamp'
import ActionCell from '../../components/table/action/Cell'
import ActionColumn from '../../components/table/action/Column'
import Layout from '../../components/table/Layout'
import { httpDelete, httpGet } from '../../utils/request'

export interface IItem {
    id: number,
    title: string,
    mimeType: string,
    url: string,
    size: number,
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
        httpDelete(`/attachments/${id}`).then(() => {
            message.success(formatMessage({ id: 'flashes.success' }))
            const items = this.state.items.filter((it) => it.id !== id)
            this.setState({ items })
        }).catch(message.error)
    }
    public componentDidMount() {
        httpGet(`/attachments`).then((rst) => {
            this.setState({ items: rst })
        }).catch(message.error)
    }
    public render() {
        const columns = [{
            key: 'content',
            render: (v: IItem) => (<a href={`/${v.url}`} target="_blank">{v.title}</a>),
            title: (<FormattedMessage id="form.labels.content" />),
        }, {
            dataIndex: 'mimeType',
            key: 'mimeType',
            title: (<FormattedMessage id="form.labels.mime-type" />),
            width: 120,
        }, {
            dataIndex: 'size',
            key: 'size',
            render: (v: number) => v > 1024 * 1024 ? `${v / 1024 / 1024}M` : (v > 1024 ? `${v / 1024}K` : `${v}B`),
            title: (<FormattedMessage id="form.labels.size" />),
            width: 80,
        }, {
            dataIndex: 'updatedAt',
            key: 'updatedAt',
            render: (v: Date) => (<Timestamp date={v} />),
            title: (<FormattedMessage id="form.labels.updated-at" />),
            width: TIMESTAMP_WIDTH,
        }, {
            key: 'action',
            render: (it: IItem) => (<ActionCell confirmRemove={{ id: 'nut.attachments.index.confirm', values: { title: it.title } }} onRemove={() => this.handleRemove(it.id)} />),
            title: (<ActionColumn to="/attachments/new" />),
            width: ACTION_WIDTH,
        }]
        return (<Authorized>
            <Layout
                rowKey="id"
                columns={columns}
                data={this.state.items}
                title={{ id: 'nut.attachments.index.title' }} />
        </Authorized>)
    }
}

export default injectIntl(Widget)
