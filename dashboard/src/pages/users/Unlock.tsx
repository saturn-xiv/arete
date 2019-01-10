import * as React from 'react'

import EmailForm from '../../components/users/EmailForm'

class Widget extends React.Component {
  public render() {
    return (<EmailForm action="unlock" />)
  }
}

export default Widget
