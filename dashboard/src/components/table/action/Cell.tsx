import { Button, Popconfirm } from 'antd'
import * as React from 'react'
import { FormattedMessage } from 'react-intl'
import { RouteComponentProps, withRouter } from 'react-router'

import { ILabel } from '../..'

interface IProps {
  confirmRemove: ILabel,
  onRemove: () => void,
  toEdit?: string,
  children?: React.ReactNode,
}

class Widget extends React.Component<RouteComponentProps<any> & IProps> {
  public render() {
    const { children, history, toEdit } = this.props
    return (<Button.Group size="small">
      {toEdit && (<Button onClick={() => history.push(toEdit)} icon="edit" type="dashed" />)}
      {children}
      <Popconfirm onConfirm={this.props.onRemove} title={<FormattedMessage {...this.props.confirmRemove} />}>
        <Button icon="delete" type="danger" />
      </Popconfirm>
    </Button.Group>)
  }
}

export default withRouter(Widget)
