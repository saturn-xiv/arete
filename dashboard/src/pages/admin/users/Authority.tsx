import { DatePicker, Form, message, Switch } from 'antd'
import { FormComponentProps } from 'antd/lib/form/Form'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'
import { connect } from 'react-redux'
import { RouteComponentProps, withRouter } from 'react-router'

import { ISiteState } from '../../../actions'
import { ILabel } from '../../../components'
import { Authorized, RoleTypes } from '../../../components/authorized'
import { formItemLayout } from '../../../components/form'
import Layout from '../../../components/form/Layout'
import Submit from '../../../components/form/Submit'
import { IApplicationState } from '../../../reducers'
import { httpGet, httpPost } from '../../../utils/request'

const FormItem = Form.Item
const RangePicker = DatePicker.RangePicker

interface IProps {
    site: ISiteState,
}

interface IState {
    admin: boolean,
    survey: boolean,
    vendor: boolean,
    album: boolean,
    schedule: boolean,
    nbf?: string,
    exp?: string,
    title: ILabel,
}

class Widget extends React.Component<RouteComponentProps<any> & InjectedIntlProps & FormComponentProps & IProps, IState> {
    public static propTypes: React.ValidationMap<any> = {
        intl: intlShape.isRequired,
    }
    constructor(props: RouteComponentProps<any> & InjectedIntlProps & FormComponentProps & IProps) {
        super(props)
        this.state = {
            admin: false,
            album: false,
            schedule: false,
            survey: false,
            title: { id: 'nut.admin.users.authority.title', values: { name: null } },
            vendor: false,
        }
    }
    public handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault()
        const { intl, history, match } = this.props
        const id = match.params.id
        const manager = 'manager'
        const policies: Array<Array<string | null>> = []
        if (this.state.admin) {
            policies.push(['admin', null])
        }
        if (this.state.album) {
            policies.push([manager, 'album'])
        }
        if (this.state.schedule) {
            policies.push([manager, 'schedule'])
        }
        if (this.state.survey) {
            policies.push([manager, 'survey'])
        }
        if (this.state.vendor) {
            policies.push([manager, 'vendor'])
        }
        httpPost(
            `/admin/users/${id}/authority`,
            {
                exp: this.state.exp,
                nbf: this.state.nbf,
                policies,
            },
        ).then((_) => {
            message.success(intl.formatMessage({ id: "flashes.success" }))
            history.push('/admin/users')
        }).catch(message.error)

    }


    public componentDidMount() {
        httpGet(`/admin/users/${this.props.match.params.id}/authority`).then((rst) => {
            const manager = 'Manager'
            if (rst.policies.some((it: string[]) => it[0] === 'Admin' && it[1] === null)) {
                this.setState({ admin: true })
            }
            if (rst.policies.some((it: string[]) => it[0] === manager && it[1] === 'vendor')) {
                this.setState({ vendor: true })
            }
            if (rst.policies.some((it: string[]) => it[0] === manager && it[1] === 'album')) {
                this.setState({ album: true })
            }
            if (rst.policies.some((it: string[]) => it[0] === manager && it[1] === 'schedule')) {
                this.setState({ schedule: true })
            }
            if (rst.policies.some((it: string[]) => it[0] === manager && it[1] === 'survey')) {
                this.setState({ survey: true })
            }

            this.setState({
                title: {
                    id: "nut.admin.users.authority.title",
                    values: {
                        name: `${rst.nickName} [${rst.realName}]`,
                    },
                }
            })
        }).catch(message.error)

    }
    public render() {
        return (<Authorized authority={RoleTypes.ADMIN}>
            <Layout title={this.state.title}>
                <Form onSubmit={this.handleSubmit}>
                    <FormItem {...formItemLayout} label={<FormattedMessage id="nut.admin.users.authority.admin" />}>
                        <Switch checked={this.state.admin} onChange={(v) => this.setState({ admin: v })} />
                    </FormItem>
                    <FormItem {...formItemLayout} label={<FormattedMessage id="nut.admin.users.authority.survey" />}>
                        <Switch checked={this.state.survey} onChange={(v) => this.setState({ survey: v })} />
                    </FormItem>
                    <FormItem {...formItemLayout} label={<FormattedMessage id="nut.admin.users.authority.schedule" />}>
                        <Switch checked={this.state.schedule} onChange={(v) => this.setState({ schedule: v })} />
                    </FormItem>
                    <FormItem {...formItemLayout} label={<FormattedMessage id="nut.admin.users.authority.album" />}>
                        <Switch checked={this.state.album} onChange={(v) => this.setState({ album: v })} />
                    </FormItem>
                    <FormItem {...formItemLayout} label={<FormattedMessage id="nut.admin.users.authority.vendor" />}>
                        <Switch checked={this.state.vendor} onChange={(v) => this.setState({ vendor: v })} />
                    </FormItem>

                    <FormItem {...formItemLayout} label={<FormattedMessage id="form.labels.range.date" />}>
                        <RangePicker onChange={(d, s) => this.setState({ nbf: s[0], exp: s[1] })} />
                    </FormItem>

                    <Submit />
                </Form>
            </Layout>
        </Authorized >)
    }
}


const mapStateToProps = ({ site }: IApplicationState) => ({
    site
})


const mapDispatchToProps = () => ({
})

export default withRouter(connect(
    mapStateToProps,
    mapDispatchToProps
)(injectIntl(Form.create()(Widget))))
