import * as React from 'react'

import EmailForm from './EmailForm'

class Widget extends React.Component {
  public render() {
    return (<EmailForm action="forgot-password" />)
  }
}

export default Widget
