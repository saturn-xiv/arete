import { Form, Input, message } from 'antd'
import { FormComponentProps } from 'antd/lib/form/Form'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'
import { connect } from 'react-redux'
import { RouteComponentProps, withRouter } from 'react-router'

import { ILabel, MediaType } from '../../../components'
import { Authorized } from '../../../components/authorized'
import { formItemLayout } from '../../../components/form'
import Layout from '../../../components/form/Layout'
import Quill from '../../../components/form/Quill'
import Submit from '../../../components/form/Submit'
import { IApplicationState } from '../../../reducers'
import { httpGet, httpPost } from '../../../utils/request'

const FormItem = Form.Item

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
            title: { id: 'forum.topics.new.title' }
        }
    }
    public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault()
        const { form, intl, history, match } = this.props
        const id = match.params.id
        form.validateFields((err, values) => {
            if (!err) {
                httpPost(
                    (id ? `/forum/topics/${id}` : '/forum/topics'),
                    Object.assign({}, values, {
                        body: this.state.body,
                        categories: [],
                        mediaType: MediaType.HTML,
                        tags: [],
                    }),
                ).then((_) => {
                    message.success(intl.formatMessage({ id: "flashes.success" }))
                    history.push('/forum/topics')
                }).catch(message.error)
            }
        })
    }

    public handleBodyChange = (v: string) => {
        this.setState({ body: v })
    }

    public componentDidMount() {
        const { form, match } = this.props
        const id = match.params.id
        if (id) {
            httpGet(`/forum/topics/${id}`).then((rst) => {
                form.setFieldsValue({
                    title: rst.title,
                })
                this.setState({
                    body: rst.body,
                    title: {
                        id: "forum.topics.edit.title",
                        values: {
                            title: rst.title,
                        },
                    }
                })
            }).catch(message.error)
        }
    }
    public render() {
        const { formatMessage } = this.props.intl
        const { getFieldDecorator } = this.props.form

        return (<Authorized>
            <Layout title={this.state.title}>
                <Form onSubmit={this.handleSubmit}>
                    <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.title" />}>
                        {
                            getFieldDecorator('title', {
                                rules: [
                                    {
                                        message: formatMessage({ id: "form.validations.required" }),
                                        required: true,
                                    }
                                ]
                            })(<Input />)
                        }
                    </FormItem>
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
