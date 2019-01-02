import { TextField } from 'office-ui-fabric-react/lib/TextField'
import * as React from 'react'
import { InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

import { Panel, validateEmail, validatePassword, validateUsername } from '../components/form'
import { httpPost } from '../utils/request'

interface IFormState {
    realName: string,
    email: string,
    errors: string[],
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
            errors: [],
            password: '',
            passwordConfirmation: '',
            realName: '',
        }
        this.handleChange = this.handleChange.bind(this)
        this.handleSubmit = this.handleSubmit.bind(this)
    }

    public handleChange(e: React.ChangeEvent<HTMLInputElement>) {
        const change = {}
        change[e.target.name] = e.target.value
        this.setState(change);
    }

    public handleSubmit(e: React.FormEvent) {
        e.preventDefault();
        const errors = []
        if (!validateEmail(this.state.email)) {
            errors.push(this.props.intl.formatMessage({ id: 'form.validations.email' }))
        }
        if (!validateUsername(this.state.realName)) {
            errors.push(this.props.intl.formatMessage({ id: 'form.validations.username' }))
        }
        if (!validatePassword(this.state.password)) {
            errors.push(this.props.intl.formatMessage({ id: 'form.validations.password' }))
        }
        if (this.state.password !== this.state.passwordConfirmation) {
            errors.push(this.props.intl.formatMessage({ id: 'form.validations.password-confirmation' }))
        }


        if (errors.length === 0) {
            httpPost('/install', this.state).then((rst) => { alert(rst) }).catch((err) => this.setState({ errors: [err] }))
        } {
            this.setState({ errors })
        }
    }

    public render() {
        return (<form onSubmit={this.handleSubmit}>
            <Panel errors={this.state.errors} title={{ id: 'install.title' }}>
                <TextField
                    name="realName"
                    value={this.state.realName}
                    onChange={this.handleChange}
                    label={this.props.intl.formatMessage({ id: 'form.labels.username' })}
                    required={true} />
                <TextField
                    name="email"
                    type="email"
                    value={this.state.email}
                    onChange={this.handleChange}
                    label={this.props.intl.formatMessage({ id: 'form.labels.email' })}
                    required={true} />
                <TextField
                    name="password"
                    value={this.state.password}
                    onChange={this.handleChange}
                    label={this.props.intl.formatMessage({ id: 'form.labels.password' })}
                    type="password"
                    required={true} />
                <TextField
                    name="passwordConfirmation"
                    value={this.state.passwordConfirmation}
                    onChange={this.handleChange}
                    label={this.props.intl.formatMessage({ id: 'form.labels.password-confirmation' })}
                    type="password"
                    required={true} />
            </Panel>
        </form>)
    }
}

export default injectIntl(Widget)