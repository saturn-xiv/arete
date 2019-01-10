import HeaderSearch from 'ant-design-pro/lib/HeaderSearch'
import { Icon, Layout, Menu, message } from 'antd'
import * as React from 'react'
import { FormattedMessage } from 'react-intl'
import { connect } from 'react-redux'
import { Dispatch } from 'redux'

import { ISiteState, IUserState, refresh as refreshSiteInfo } from '../actions'
import { IApplicationState } from '../reducers'
import { httpGet } from '../utils/request'
import Footer from './Footer'


const { Header, Sider, Content } = Layout

interface IProps {
  children: React.ReactNode,
  user: IUserState,
  site: ISiteState,
  refresh: typeof refreshSiteInfo,
}

interface IState {
  collapsed: boolean,
}

interface IMenu {
  key: string,
  label: React.ReactNode,
  icon: string,
  children: IMenuItem[],
}
interface IMenuItem {
  key: string,
  label: React.ReactNode,
}

interface INavItem {
  children: React.ReactNode,
  key: string,
}

function headerBar(user: IUserState): INavItem[] {
  const items = [
    {
      children: (<Icon type="home" />),
      key: 'home',
    },
    {
      children: (<HeaderSearch />),
      key: "search",
    },
    {
      children: (<Icon type="reload" />),
      key: "reload",
    }
  ]
  items.push({
    children: (<Icon type="question-circle-o" />),
    key: "doc",
  })
  if (user.uid) {
    items.push({
      children: (<Icon type="logout" />),
      key: "sign-out",
    })
  } else {
    items.push({
      children: (<Icon type="login" />),
      key: "sign-in",
    })
  }
  return items.reverse()
}

function siderBar(user: IUserState): IMenu[] {
  return []
}


class Widget extends React.Component<IProps, IState> {
  constructor(props: IProps) {
    super(props)
    this.state = {
      collapsed: false,
    }
  }
  public componentDidMount() {
    httpGet(`/about`).then((rst) => {
      this.props.refresh(rst)
    }).catch(message.error)
    // TODO check sign-in
  }
  public render() {
    const sider = siderBar(this.props.user).map((it) => (<Menu.SubMenu
      key={it.key}
      title={(<span><Icon type={it.icon} />{it.label}</span>)}>
      {it.children.map((jt) => (<Menu.Item key={jt.key}>{jt.label}</Menu.Item>))}
    </Menu.SubMenu>))
    const header = headerBar(this.props.user).map((it) => (<Menu.Item className="pull-right" key={it.key}>{it.children}</Menu.Item>))
    const languages = this.props.site.languages.map((it) => (<Menu.Item key={`lang-${it}`}>
      <FormattedMessage id={`languages.${it}`} />
    </Menu.Item>))
    return (<Layout>
      <Sider breakpoint="lg" collapsedWidth="0" trigger={null} collapsible={true} collapsed={this.state.collapsed}>
        <div className="sider-logo" />
        <Menu theme="dark" mode="inline" defaultSelectedKeys={[]}>
          {sider}
        </Menu>
      </Sider>
      <Layout>
        <Header className="header-bar">
          <Menu mode="horizontal">
            <Menu.Item key='toggle'>
              <Icon className="trigger" type={this.state.collapsed ? 'menu-unfold' : 'menu-fold'} />
            </Menu.Item>
            {header}
            <Menu.SubMenu className="pull-right" key="switch-languages" title={<Icon type="global" />}>
              {languages}
            </Menu.SubMenu>
          </Menu>
        </Header>
        <Content className="root-content">
          {this.props.children}
        </Content>
        <Footer />
      </Layout>
    </Layout>)
  }
}

const mapStateToProps = ({ site, user }: IApplicationState) => ({
  site,
  user,
})


const mapDispatchToProps = (dispatch: Dispatch) => ({
  refresh: (info: ISiteState) => dispatch(refreshSiteInfo(info))
})

export default connect(
  mapStateToProps,
  mapDispatchToProps
)(Widget)
