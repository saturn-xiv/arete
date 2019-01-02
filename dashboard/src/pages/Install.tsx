import { PrimaryButton } from 'office-ui-fabric-react/lib/Button'
import { TextField } from 'office-ui-fabric-react/lib/TextField'
import * as React from 'react'
import { FormattedMessage } from 'react-intl'

class Widget extends React.Component {
    public render() {
        return (<div className="ms-Grid-row">
            <div className="ms-Grid-col ms-sm10 ms-smPush1 ms-md4 ms-mdPush4">
                <FormattedMessage id="install.title" tagName="h2" />
                <TextField label="Nickname" required={true} />
                <TextField label="Realname" required={true} />
                <TextField label="Email" required={true} />
                <TextField label="Password" type="password" required={true} />
                <TextField label="Password confirmation" type="password" required={true} />
                <br />
                <PrimaryButton><FormattedMessage id="buttons.submit" /></PrimaryButton>
            </div>
        </div>)
    }
}

export default Widget