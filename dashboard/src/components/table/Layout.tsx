import { Col, Row, Table } from 'antd'
import { ColumnProps } from 'antd/lib/table'
import * as React from 'react'
import { FormattedMessage } from 'react-intl'

import { ILabel } from '..'
import Head from '../Head'

interface IProps<T> {
  rowKey: string,
  title: ILabel,
  columns: Array<ColumnProps<T>>,
  data: T[],
  expandedRowRender?: (it: T) => React.ReactNode,
}

class Widget<T> extends React.Component<IProps<T>> {
  public render() {
    return (<Row>
      <Col sm={{ span: 24 }} md={{ offset: 1, span: 22 }}>
        <Table
          title={() => (<FormattedMessage {...this.props.title} />)}
          bordered={true}
          columns={this.props.columns}
          rowKey={this.props.rowKey}
          expandedRowRender={this.props.expandedRowRender}
          dataSource={this.props.data} />
        <Head title={this.props.title} />
      </Col >
    </Row >)
  }
}

export default Widget
