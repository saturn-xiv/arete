import * as React from 'react'
import { FormattedMessage } from 'react-intl'

class Widget extends React.Component {
    public render() {
        return (<div><FormattedMessage id="nut.home.title" /></div>)
    }
}

export default Widget