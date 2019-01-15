import { Button, Col, Collapse, List, message, Popconfirm, Row, Table } from 'antd'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'

import { Authorized, RoleTypes } from '../../../components/authorized'
import Head from '../../../components/Head'
import { httpDelete, httpGet, httpPost } from '../../../utils/request'

const Panel = Collapse.Panel

interface IPostgreSql {
  status: object,
  databases: IPostgreSqlDatabase[],
}

interface IPostgreSqlDatabase {
  name: string,
  size: number;
}

interface INetwork {
  name: string,
  mac?: string,
  ip4?: string,
  ip6?: string,
}


interface IState {
  network: INetwork[],
  postgresql: IPostgreSql,
  redis: string,
  os: object,
}

class Widget extends React.Component<InjectedIntlProps, IState> {
  public static propTypes: React.ValidationMap<InjectedIntlProps> = {
    intl: intlShape.isRequired,
  }
  constructor(props: any) {
    super(props)
    this.state = {
      network: [],
      os: {},
      postgresql: { status: {}, databases: [] },
      redis: '',
    }
  }

  public handleClearCache = () => {
    httpDelete(`/admin/site/clear-cache`).then((rst) => {
      message.success(rst)
    }).catch(message.error)
  }

  public handleSendTestEmail = () => {
    const { formatMessage } = this.props.intl
    httpPost(`/admin/site/send-test-email`, {}).then((_) => {
      message.success(formatMessage({ id: 'nut.admin.site.status.manage.send-test-email.success' }))
    }).catch(message.error)
  }

  public componentDidMount() {
    httpGet(`/admin/site/status`).then((rst) => {
      this.setState(rst)
    }).catch(message.error)
  }
  public render() {
    const { os, network, postgresql, redis } = this.state

    return (<Authorized authority={RoleTypes.ADMIN}>
      <Row>
        <Col sm={{ span: 24 }} md={{ offset: 1, span: 22 }}>
          <Collapse>
            <Panel key="os" header={(<FormattedMessage id="nut.admin.site.status.os" />)}>
              <List
                size="small"
                bordered={true}
                dataSource={Object.keys(os).map((k: string) => `${k}: ${os[k]}`)}
                renderItem={(it: string) => (<List.Item>{it}</List.Item>)}
              />
            </Panel>
            <Panel key="network" header={(<FormattedMessage id="nut.admin.site.status.network" />)}>
              <Table
                title={() => (<FormattedMessage id="nut.admin.site.status.network" />)}
                bordered={true}
                size="small"
                columns={[{
                  dataIndex: 'name',
                  key: 'name',
                  title: (<FormattedMessage id="form.labels.name" />),
                }, {
                  dataIndex: 'ip4',
                  key: 'ip4',
                  title: 'IPV4',
                }, {
                  dataIndex: 'ip6',
                  key: 'ip6',
                  title: 'IPV6',
                }, {
                  dataIndex: 'mac',
                  key: 'mac',
                  title: 'MAC',
                }]}
                rowKey="name"
                dataSource={network} />
            </Panel>
            <Panel key="postgresql" header={(<FormattedMessage id="nut.admin.site.status.postgresql" />)}>
              <List
                size="small"
                bordered={true}
                dataSource={Object.keys(postgresql.status).map((k: string) => `${k}: ${postgresql.status[k]}`)}
                renderItem={(it: string) => (<List.Item>{it}</List.Item>)}
              />
              <br />
              <Table
                bordered={true}
                size="small"
                columns={[{
                  dataIndex: 'name',
                  key: 'name',
                  title: 'Name',
                }, {
                  dataIndex: 'size',
                  key: 'size',
                  render: (it: number) => `${it}MB`,
                  title: 'Size',
                }]}
                rowKey="name"
                dataSource={postgresql.databases} />
            </Panel>
            <Panel key="redis" header={(<FormattedMessage id="nut.admin.site.status.redis" />)}>
              <List
                size="small"
                bordered={true}
                dataSource={redis.split("\n").filter((it) => it.trim().length > 0)}
                renderItem={(it: string) => (<List.Item>{it}</List.Item>)}
              />
            </Panel>
            <Panel key="manage" header={(<FormattedMessage id="nut.admin.site.status.manage.title" />)}>
              <Button.Group size="small">
                <Popconfirm onConfirm={this.handleClearCache} title={<FormattedMessage id="nut.admin.site.status.manage.clear-cache.confirm" />}>
                  <Button type="danger" icon="delete">
                    <FormattedMessage id="nut.admin.site.status.manage.clear-cache.label" />
                  </Button>
                </Popconfirm>
                <Popconfirm onConfirm={this.handleSendTestEmail} title={<FormattedMessage id="nut.admin.site.status.manage.send-test-email.confirm" />}>
                  <Button type="dashed" icon="mail">
                    <FormattedMessage id="nut.admin.site.status.manage.send-test-email.label" />
                  </Button>
                </Popconfirm>
              </Button.Group>
            </Panel>
          </Collapse>
        </Col>
      </Row >
      <Head title={{ id: 'nut.admin.site.status.title' }} />
    </Authorized>)
  }
}

export default injectIntl(Widget)
