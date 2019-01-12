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
    return (<Row>
      <Col sm={{ span: 24 }} md={{ offset: 3, span: 12 }}>
        <FormattedMessage tagName="h1" id={this.props.title.id} values={this.props.title.values} />
      </Col>
      <Col sm={{ span: 24 }} md={{ offset: 1, span: 12 }}>
        {this.props.children}
        <Head title={this.props.title} />
      </Col >
    </Row >)
  }
}

export default Widget
