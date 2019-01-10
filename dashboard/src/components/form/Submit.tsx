import { Button, Form } from 'antd'
import * as React from 'react'
import { FormattedMessage } from 'react-intl'

import { tailFormItemLayout } from '.'

const FormItem = Form.Item

class Widget extends React.Component {
  public render() {
    const { children } = this.props
    return (<FormItem {...tailFormItemLayout}>
      <Button type="primary" htmlType="submit">
        <FormattedMessage id="buttons.submit" />
      </Button>
      {children}
    </FormItem>)
  }
}

export default Widget
