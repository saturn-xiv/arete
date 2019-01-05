import { PrimaryButton } from 'office-ui-fabric-react/lib/Button'
import { MessageBar, MessageBarType } from 'office-ui-fabric-react/lib/MessageBar'
import { TextField } from 'office-ui-fabric-react/lib/TextField'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

import { IMessageBar } from '../../components'
import Layout from '../../components/NonSignIn'
import { httpPost } from '../../utils/request'

interface IFormState {
    nickName: string,
    realName: string,
    email: string,
    bar?: IMessageBar,
    password: string,
    passwordConfirmation: string,
}

class Widget extends React.Component<InjectedIntlProps, IFormState> {

    public static propTypes: React.ValidationMap<any> = {
        intl: intlShape.isRequired,
    }

    constructor(props: InjectedIntlProps) {
        super(props)
        this.state = {
            email: '',
            nickName: '',
            password: '',
            passwordConfirmation: '',
            realName: '',
        }
        this.handleChange = this.handleChange.bind(this)
        this.handleSubmit = this.handleSubmit.bind(this)
        this.handleDismiss = this.handleDismiss.bind(this)
    }

    public handleChange(e: React.ChangeEvent<HTMLInputElement>) {
        const change = {}
        change[e.target.name] = e.target.value
        this.setState(change);
    }


    public handleDismiss() {
        this.setState({ bar: undefined })
    }

    public handleSubmit(e: React.FormEvent) {
        e.preventDefault();

        const { formatMessage } = this.props.intl
        if (this.state.password !== this.state.passwordConfirmation) {
            this.setState({
                bar: {
                    content: formatMessage({ id: 'form.validations.password-confirmation' }),
                    type: MessageBarType.error,
                }
            })
            return
        }

        httpPost('/users/sign-up', this.state).then((rst) => {
            this.setState({
                bar: {
                    content: formatMessage({ id: 'users.confirm.success' }),
                    type: MessageBarType.success,
                }
            })
        }).catch((err) => this.setState({
            bar: {
                content: err,
                type: MessageBarType.error,
            }
        }))

    }

    public render() {
        const { formatMessage } = this.props.intl
        return (<Layout>
            <FormattedMessage id="users.sign-up.title" tagName="h2" />
            {this.state.bar && (<MessageBar
                messageBarType={this.state.bar.type} onDismiss={this.handleDismiss}
                isMultiline={false}
                dismissButtonAriaLabel="Close">
                {this.state.bar.content}
            </MessageBar>)}
            <form onSubmit={this.handleSubmit}>
                <TextField
                    name="nickName"
                    value={this.state.nickName}
                    onChange={this.handleChange}
                    label={formatMessage({ id: 'form.labels.nick-name' })}
                    description={formatMessage({ id: 'form.helps.nick-name' })}
                    required={true} />
                <TextField
                    name="realName"
                    value={this.state.realName}
                    onChange={this.handleChange}
                    label={formatMessage({ id: 'form.labels.username' })}
                    required={true} />
                <TextField
                    name="email"
                    type="email"
                    value={this.state.email}
                    onChange={this.handleChange}
                    label={formatMessage({ id: 'form.labels.email' })}
                    required={true} />
                <TextField
                    name="password"
                    value={this.state.password}
                    onChange={this.handleChange}
                    label={formatMessage({ id: 'form.labels.password' })}
                    description={formatMessage({ id: 'form.helps.password' })}
                    type="password"
                    required={true} />
                <TextField
                    name="passwordConfirmation"
                    value={this.state.passwordConfirmation}
                    onChange={this.handleChange}
                    label={formatMessage({ id: 'form.labels.password-confirmation' })}
                    type="password"
                    required={true} />
                <br />
                <PrimaryButton type="submit" text={formatMessage({ id: 'buttons.submit' })} />
            </form>
        </Layout>)
    }
}

export default injectIntl(Widget)