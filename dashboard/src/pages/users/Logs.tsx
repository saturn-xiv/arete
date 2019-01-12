import * as React from 'react'
import { FormattedMessage } from 'react-intl'

import { Authorized } from '../../components/authorized'

class Widget extends React.Component {
  public render() {
    return (<Authorized>
      <FormattedMessage id="nut.users.logs.title" />
    </Authorized>)
  }
}

export default Widget
