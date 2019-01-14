import { Col, Row } from 'antd'
import * as React from 'react'
import { FormattedMessage } from 'react-intl'

import { ILabel } from '..'
import Head from '../Head'

interface IProps {
  title: ILabel,
  children: React.ReactNode,
}

class Widget extends React.Component<IProps> {
  public render() {
    const { children, title } = this.props

    return (<Row>
      <Col sm={{ span: 24 }} md={{ offset: 3, span: 12 }}>
        <FormattedMessage tagName="h1" {...title} />
      </Col>
      <Col sm={{ span: 24 }} md={{ offset: 1, span: 12 }}>
        {children}
        <Head title={title} />
      </Col >
    </Row >)
  }
}

export default Widget
