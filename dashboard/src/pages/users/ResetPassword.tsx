import { PrimaryButton } from 'office-ui-fabric-react/lib/Button'
import { MessageBar, MessageBarType } from 'office-ui-fabric-react/lib/MessageBar'
import { TextField } from 'office-ui-fabric-react/lib/TextField'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

import { IMessageBar } from '../../components'
import Layout from '../../components/NonSignIn'
import { httpPost } from '../../utils/request'

interface IFormState {
    password: string,
    passwordConfirmation: string,
    bar?: IMessageBar,
}

class Widget extends React.Component<InjectedIntlProps, IFormState> {

    public static propTypes: React.ValidationMap<any> = {
        intl: intlShape.isRequired,
    }

    constructor(props: InjectedIntlProps) {
        super(props)
        this.state = {
            password: '',
            passwordConfirmation: '',
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

        httpPost('/users/reset-password', this.state).then((rst) => {
            this.setState({
                bar: {
                    content: formatMessage({ id: 'users.change-password.success' }),
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
            <FormattedMessage id="users.reset-password.title" tagName="h2" />
            {this.state.bar && (<MessageBar
                messageBarType={this.state.bar.type} onDismiss={this.handleDismiss}
                isMultiline={false}
                dismissButtonAriaLabel="Close">
                {this.state.bar.content}
            </MessageBar>)}
            <form onSubmit={this.handleSubmit}>
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