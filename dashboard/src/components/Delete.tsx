import { Button, Popconfirm } from 'antd'
import * as React from 'react'
import { FormattedMessage } from 'react-intl'

import { ILabel } from '.'

interface IProps<T> {
  id: T,
  title: ILabel,
  onRemove: (id: T) => void,
}

class Widget<T> extends React.Component<IProps<T>> {
  public render() {
    return (<Popconfirm onConfirm={() => this.props.onRemove(this.props.id)} title={<FormattedMessage {...this.props.title} />}>
      <Button shape="circle" size="small" icon="delete" type="danger" />
    </Popconfirm>)
  }
}

export default Widget
