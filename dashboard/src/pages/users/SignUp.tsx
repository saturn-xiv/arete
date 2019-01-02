import { TextField } from 'office-ui-fabric-react/lib/TextField'
import * as React from 'react'

import Form from '../../components/Form'

class Widget extends React.Component {
    public render() {
        return (<Form title={{ id: 'users.sign-up.title' }}>
            <TextField label="Nickname" required={true} />
            <TextField label="Realname" required={true} />
            <TextField label="Email" required={true} />
            <TextField label="Password" type="password" required={true} />
            <TextField label="Password confirmation" type="password" required={true} />
        </Form>)
    }
}

export default Widget