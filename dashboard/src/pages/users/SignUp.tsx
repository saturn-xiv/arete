import { TextField } from 'office-ui-fabric-react/lib/TextField'
import * as React from 'react'

import { Panel } from '../../components/form'

class Widget extends React.Component {
    public render() {
        return (<Panel errors={[]} title={{ id: 'users.sign-up.title' }}>
            <TextField label="Nickname" required={true} />
            <TextField label="Realname" required={true} />
            <TextField label="Email" required={true} />
            <TextField label="Password" type="password" required={true} />
            <TextField label="Password confirmation" type="password" required={true} />
        </Panel>)
    }
}

export default Widget