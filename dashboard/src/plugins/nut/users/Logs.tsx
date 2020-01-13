import React, { Component } from "react";
import {
  injectIntl,
  FormattedMessage,
  WrappedComponentProps
} from "react-intl";
import { Row, Table, Col, message } from "antd";
import { ColumnProps } from "antd/es/table";

import Layout from "../../../layouts/dashboard";
import { show as moment } from "../../../utils/moment";
import { get as httpGet } from "../../../utils/request";

interface ILog {
  id: number;
  ip: string;
  message: string;
  createdAt: Date;
}

interface IProps {}
interface IState {
  items: ILog[];
  columns: ColumnProps<ILog>[];
}

class Widget extends Component<WrappedComponentProps & IProps, IState> {
  constructor(props: WrappedComponentProps & IProps) {
    super(props);
    this.state = {
      items: [],
      columns: [
        {
          title: <FormattedMessage id="attributes.created-at" />,
          render: (text: any, record: ILog) => moment(record.createdAt),
          width: 320,
          key: "createdAt"
        },
        {
          title: <FormattedMessage id="attributes.message" />,
          dataIndex: "message",
          key: "message"
        }
      ]
    };
  }
  componentDidMount() {
    httpGet("/users/logs?page=1&size=1024")
      .then(rst => {
        this.setState({ items: rst.items });
      })
      .catch(message.error);
  }
  public render() {
    const { intl } = this.props;
    const title = { id: "nut.logs.title" };
    return (
      <Layout title={intl.formatMessage(title)}>
        <Row>
          <Col
            sm={{
              span: 24
            }}
            md={{
              span: 12,
              offset: 1
            }}
          >
            <h1>
              <FormattedMessage {...title} />
            </h1>
          </Col>
          <Col
            sm={{
              span: 24
            }}
            md={{
              span: 22,
              offset: 1
            }}
          >
            <Table<ILog>
              rowKey="id"
              bordered={true}
              dataSource={this.state.items}
              columns={this.state.columns}
            />
          </Col>
        </Row>
      </Layout>
    );
  }
}

export default injectIntl(Widget);
