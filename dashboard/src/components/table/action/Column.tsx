import { Button } from 'antd'
import * as React from 'react'
import { FormattedMessage } from 'react-intl'
import { RouteComponentProps, withRouter } from 'react-router'

interface IProps {
  to?: string,
}

class Widget extends React.Component<RouteComponentProps<any> & IProps> {
  public render() {
    const { history, to } = this.props
    return (<>
      <FormattedMessage id="buttons.action" /> &nbsp;
      {to && (<Button onClick={() => history.push(to)} size="small" icon="plus" type="primary" />)}
    </>)
  }
}

export default withRouter(Widget)
