import { TextField } from 'office-ui-fabric-react/lib/TextField'
import * as React from 'react'
import { InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

import Form from '../components/Form'

class Widget extends React.Component<InjectedIntlProps> {
    public static propTypes: React.ValidationMap<any> = {
        intl: intlShape.isRequired
    };

    public render() {
        return (<Form title={{ id: 'install.title' }}>
            <TextField label={this.props.intl.formatMessage({ id: 'form.labels.nick-name' })} required={true} />
            <TextField label="Realname" required={true} />
            <TextField label="Email" required={true} />
            <TextField label="Password" type="password" required={true} />
            <TextField label="Password confirmation" type="password" required={true} />
        </Form>)
    }
}

export default injectIntl(Widget)