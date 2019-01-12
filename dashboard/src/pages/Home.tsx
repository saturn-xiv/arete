import * as React from 'react'
import { RouteComponentProps, withRouter } from "react-router"

import { get as getToken } from '../utils/token'

class Widget extends React.Component<RouteComponentProps<any>> {
  public componentDidMount() {
    this.props.history.push(getToken() ? '/users/logs' : '/users/sign-in')
  }
  public render() {
    return (<></>)
  }
}

export default withRouter(Widget)
