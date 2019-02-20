import { Form, message } from 'antd'
import { FormComponentProps } from 'antd/lib/form/Form'
import * as React from 'react'
import { InjectedIntlProps, injectIntl, intlShape } from 'react-intl'
import { connect } from 'react-redux'
import { RouteComponentProps, withRouter } from 'react-router'

import { ILabel, MediaType } from '../../../components'
import { Authorized } from '../../../components/authorized'
import Layout from '../../../components/form/Layout'
import Quill from '../../../components/form/Quill'
import Submit from '../../../components/form/Submit'
import { IApplicationState } from '../../../reducers'
import { httpGet, httpPost } from '../../../utils/request'

interface IState {
    body: string,
    title: ILabel,
}

class Widget extends React.Component<RouteComponentProps<any> & InjectedIntlProps & FormComponentProps, IState> {
    public static propTypes: React.ValidationMap<any> = {
        intl: intlShape.isRequired,
    }
    constructor(props: RouteComponentProps<any> & InjectedIntlProps & FormComponentProps) {
        super(props)
        this.state = {
            body: '',
            title: { id: 'forum.posts.new.title' }
        }
    }
    public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault()
        const { intl, history, match } = this.props
        const id = match.params.id

        httpPost(
            (id ? `/forum/posts/${id}` : '/forum/posts'),
            {
                body: this.state.body,
                mediaType: MediaType.HTML,
            },
        ).then((_) => {
            message.success(intl.formatMessage({ id: "flashes.success" }))
            history.push('/forum/posts')
        }).catch(message.error)
    }

    public handleBodyChange = (v: string) => {
        this.setState({ body: v })
    }

    public componentDidMount() {
        const { form, match } = this.props
        const id = match.params.id
        if (id) {
            httpGet(`/forum/posts/${id}`).then((rst) => {
                form.setFieldsValue({
                    title: rst.title,
                })
                this.setState({
                    body: rst.body,
                    title: {
                        id: "forum.posts.edit.title",
                        values: {
                            id,
                        },
                    }
                })
            }).catch(message.error)
        }
    }

    public render() {
        return (<Authorized>
            <Layout title={this.state.title}>
                <Form onSubmit={this.handleSubmit}>
                    <Quill value={this.state.body} onChange={this.handleBodyChange} />
                    <Submit />
                </Form>
            </Layout>
        </Authorized >)
    }
}


const mapStateToProps = ({ }: IApplicationState) => ({})


const mapDispatchToProps = () => ({
})

export default withRouter(connect(
    mapStateToProps,
    mapDispatchToProps
)(injectIntl(Form.create()(Widget))))
