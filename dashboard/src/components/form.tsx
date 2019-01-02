import * as moment from 'moment'
import { PrimaryButton } from 'office-ui-fabric-react/lib/Button'
import { MessageBar, MessageBarType } from 'office-ui-fabric-react/lib/MessageBar'
import * as React from 'react'

import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

export const validateEmail = (s: string): boolean => /^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$/.test(s)

export const validatePassword = (s: string): boolean => s.length >= 6

export const validateUsername = (s: string): boolean => s.length >= 2 && s.length <= 32

export const validateNickname = (s: string): boolean => s.length >= 2 && s.length <= 32

export const validateRequired = (s: string): boolean => s.length > 0



interface IPanelTitle {
    id: string,
    value?: object,
}

interface IPanelProps {
    children: JSX.Element | JSX.Element[],
    title: IPanelTitle,
    errors: string[],
}

interface IPanelMessageState {
    created: string,
    show: boolean,
}

class FormPanel extends React.Component<IPanelProps & InjectedIntlProps, IPanelMessageState> {
    public static propTypes: React.ValidationMap<any> = {
        intl: intlShape.isRequired
    }

    constructor(props: IPanelProps & InjectedIntlProps) {
        super(props)
        this.state = { created: '', show: false }
        this.handleDismiss = this.handleDismiss.bind(this)
    }

    public componentDidUpdate(prevProps: IPanelProps) {
        if (this.props.errors !== prevProps.errors) {
            this.setState({ created: moment().format('ll LTS'), show: this.props.errors.length > 0 })
        }
    }

    public handleDismiss() {
        this.setState({ show: false })
    }

    public render() {
        return (<div className="ms-Grid-row">
            <div className="ms-Grid-col ms-sm10 ms-smPush1 ms-md4 ms-mdPush4">
                <FormattedMessage {...this.props.title} tagName="h2" />
                {this.props.errors.length > 0 && this.state.show ?
                    (<MessageBar
                        messageBarType={MessageBarType.error} onDismiss={this.handleDismiss}
                        isMultiline={true}
                        dismissButtonAriaLabel="Close">
                        <div>{this.state.created}</div>
                        {this.props.errors.map((it, id) => (<div key={id}>{it}</div>))}
                    </MessageBar>) : (<div />)}
                {this.props.children}
                <br />
                <PrimaryButton type="submit" text={this.props.intl.formatMessage({ id: 'buttons.submit' })} />
            </div>
        </div>)
    }
}

export const Panel = injectIntl(FormPanel)