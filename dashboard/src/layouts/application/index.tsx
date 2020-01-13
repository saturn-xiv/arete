import React, { Component, ReactNode } from "react";
import { Row, Col, Layout } from "antd";

import Title from "../Title";
import Footer from "../Footer";

const { Header, Content } = Layout;

interface IProps {
  children: ReactNode;
  title: string;
}

interface IState {}

class Widget extends Component<IProps, IState> {
  public render() {
    const { title, children } = this.props;
    return (
      <Layout>
        <Header />
        <Content>
          <Row>
            <Col
              sm={{
                span: 24
              }}
              md={{
                span: 8,
                offset: 8
              }}
            >
              <h1>{title}</h1>
              {children}
              <Title value={title} />
            </Col>
          </Row>
        </Content>
        <Footer />
      </Layout>
    );
  }
}

export default Widget;
