import { PrimaryButton } from 'office-ui-fabric-react/lib/Button'
import { MessageBar, MessageBarType } from 'office-ui-fabric-react/lib/MessageBar'
import { TextField } from 'office-ui-fabric-react/lib/TextField'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

import { IMessageBar } from '../../components'
import Layout from '../../components/NonSignIn'
import { httpPost } from '../../utils/request'

interface IFormState {
    email: string,
    bar?: IMessageBar,
}

interface IFormProps {
    action: string,
}

class Widget extends React.Component<InjectedIntlProps & IFormProps, IFormState> {

    public static propTypes: React.ValidationMap<any> = {
        intl: intlShape.isRequired,
    }

    constructor(props: InjectedIntlProps & IFormProps) {
        super(props)
        this.state = {
            email: '',
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

        const { action } = this.props
        const { formatMessage } = this.props.intl

        httpPost(`/users/${action}`, this.state).then((rst) => {
            this.setState({
                bar: {
                    content: formatMessage({ id: `users.${action}.success` }),
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
            <FormattedMessage id={`users.${this.props.action}.title`} tagName="h2" />
            {this.state.bar && (<MessageBar
                messageBarType={this.state.bar.type} onDismiss={this.handleDismiss}
                isMultiline={false}
                dismissButtonAriaLabel="Close">
                {this.state.bar.content}
            </MessageBar>)}
            <form onSubmit={this.handleSubmit}>
                <TextField
                    name="email"
                    type="email"
                    value={this.state.email}
                    onChange={this.handleChange}
                    label={formatMessage({ id: 'form.labels.email' })}
                    required={true} />
                <br />
                <PrimaryButton type="submit" text={formatMessage({ id: 'buttons.submit' })} />
            </form>
        </Layout>)
    }
}

export default injectIntl(Widget)