import { Button, message } from 'antd'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'
import { RouteComponentProps, withRouter } from 'react-router'

import { MediaType } from '../../../components'
import { Authorized } from '../../../components/authorized'
import { ACTION_WIDTH, TIMESTAMP_WIDTH } from '../../../components/form'
import Timestamp from '../../../components/moment/Timestamp'
import ActionCell from '../../../components/table/action/Cell'
import ActionColumn from '../../../components/table/action/Column'
import Layout from '../../../components/table/Layout'
import { httpDelete, httpGet } from '../../../utils/request'

export interface IItem {
    id: number,
    title: string,
    body: string,
    mediaType: MediaType,
    updatedAt: Date,
}

interface IState {
    items: IItem[],
}

class Widget extends React.Component<RouteComponentProps<any> & InjectedIntlProps, IState> {
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
        httpDelete(`/forum/topics/${id}`).then(() => {
            message.success(formatMessage({ id: 'flashes.success' }))
            const items = this.state.items.filter((it) => it.id !== id)
            this.setState({ items })
        }).catch(message.error)
    }
    public componentDidMount() {
        httpGet(`/forum/topics`).then((rst) => {
            this.setState({ items: rst })
        }).catch(message.error)
    }
    public render() {
        const { history } = this.props
        const columns = [{
            key: 'content',
            render: (v: IItem) => (<a href={`/forum/topics/${v.id}`} target="_blank">{v.title}</a>),
            title: (<FormattedMessage id="form.labels.content" />),
        }, {
            dataIndex: 'updatedAt',
            key: 'updatedAt',
            render: (v: Date) => (<Timestamp date={v} />),
            title: (<FormattedMessage id="form.labels.updated-at" />),
            width: TIMESTAMP_WIDTH,
        }, {
            key: 'action',
            render: (it: IItem) => (<ActionCell toEdit={`/forum/topics/${it.id}/edit`} confirmRemove={{ id: 'forum.topics.index.confirm', values: { title: it.title } }} onRemove={() => this.handleRemove(it.id)} >
                <Button onClick={() => history.push(`/forum/posts/new?topic=${it.id}`)} size="small" icon="message" type="default" />
            </ActionCell>),
            title: (<ActionColumn to="/forum/topics/new" />),
            width: ACTION_WIDTH + 20,
        }]
        return (<Authorized>
            <Layout
                rowKey="id"
                columns={columns}
                data={this.state.items}
                title={{ id: 'forum.topics.index.title' }} />
        </Authorized>)
    }
}

export default withRouter(injectIntl(Widget))
