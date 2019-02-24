import HeaderSearch from 'ant-design-pro/lib/HeaderSearch'
import { Icon, Layout, Menu, message, Modal, Tooltip } from 'antd'
import { ClickParam } from 'antd/lib/menu'
import * as React from 'react'
import { FormattedMessage, InjectedIntlProps, injectIntl, intlShape } from 'react-intl'
import { connect } from 'react-redux'
import { RouteComponentProps, withRouter } from 'react-router'
import { Dispatch } from 'redux'

import { ISiteState, IUserState, siteRefresh, userSignIn, userSignOut } from '../actions'
// import * as logo from '../assets/cloud.svg'
import { havePermission, RoleTypes } from '../components/authorized'
import { set as setLocale } from '../intl'
import { IApplicationState } from '../reducers'
import { graphql, httpDelete } from '../utils/request'
import { get as getToken } from '../utils/token'
import Footer from './Footer'

const { Header, Sider, Content } = Layout

interface IAbout { apiVersion: string, currentUser?: ICurrentUser, availableLanguage: string[] }
interface ICurrentUser { logo: string, realName: string }

interface IProps {
  children: React.ReactNode,
  user: IUserState,
  site: ISiteState,
  refresh: typeof siteRefresh,
  signIn: typeof userSignIn,
  signOut: typeof userSignOut,
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
  children: React.ReactNode,
}

function headerBar(user: IUserState): IMenuItem[] {
  const items = [{
    children: (<Icon type="home" />),
    key: 'home',
  }]
  if (user.uid) {
    items.push({
      children: (<Tooltip title={<FormattedMessage id="layout.header.dashboard" />}><Icon type="dashboard" /></Tooltip>),
      key: "to-/users/logs",
    })
  }
  items.push({
    children: (<Tooltip title={<FormattedMessage id="layout.header.search" />}><HeaderSearch /></Tooltip>),
    key: "search",
  })
  items.push({
    children: (<Tooltip title={<FormattedMessage id="layout.header.reload" />}><Icon type="reload" /></Tooltip>),
    key: "reload",
  })
  items.push({
    children: (<Tooltip title={<FormattedMessage id="layout.header.doc" />}><Icon type="question-circle-o" /></Tooltip>),
    key: "doc",
  })
  if (user.uid) {
    items.push({
      children: (<Tooltip title={<FormattedMessage id="nut.users.sign-out.title" />}><Icon type="logout" /></Tooltip>),
      key: "sign-out",
    })
  } else {
    items.push({
      children: (<Tooltip title={<FormattedMessage id="nut.users.sign-in.title" />}><Icon type="login" /></Tooltip>),
      key: "to-/users/sign-in",
    })
  }
  return items.reverse()
}

function siderBar(user: IUserState): IMenu[] {
  const items: IMenu[] = []
  if (!user.uid) {
    return items
  }

  items.push({
    children: [
      { children: (<FormattedMessage id="nut.attachments.index.title" />), key: "to-/attachments" },
      { children: (<FormattedMessage id="nut.users.logs.title" />), key: "to-/users/logs" },
      { children: (<FormattedMessage id="nut.users.change-password.title" />), key: "to-/users/change-password" },
      { children: (<FormattedMessage id="nut.users.profile.title" />), key: "to-/users/profile" },
    ],
    icon: 'user',
    key: 'personal',
    label: (<FormattedMessage id="nut.dashboard.personal.title" />),
  })

  if (havePermission(user, RoleTypes.ADMIN)) {
    items.push({
      children: [
        { children: (<FormattedMessage id="nut.admin.site.status.title" />), key: "to-/admin/site/status" },
        { children: (<FormattedMessage id="nut.admin.site.info.title" />), key: "to-/admin/site/info" },
        { children: (<FormattedMessage id="nut.admin.site.author.title" />), key: "to-/admin/site/author" },
        { children: (<FormattedMessage id="nut.admin.site.seo.title" />), key: "to-/admin/site/seo" },
        { children: (<FormattedMessage id="nut.admin.site.smtp.title" />), key: "to-/admin/site/smtp" },
        { children: (<FormattedMessage id="nut.admin.locales.index.title" />), key: "to-/admin/locales" },
        { children: (<FormattedMessage id="nut.admin.tags.index.title" />), key: "to-/admin/tags" },
        { children: (<FormattedMessage id="nut.admin.categories.index.title" />), key: "to-/admin/categories" },
        { children: (<FormattedMessage id="nut.admin.links.index.title" />), key: "to-/admin/links" },
        { children: (<FormattedMessage id="nut.admin.cards.index.title" />), key: "to-/admin/cards" },
        { children: (<FormattedMessage id="nut.admin.leave-words.index.title" />), key: "to-/admin/leave-words" },
        { children: (<FormattedMessage id="nut.admin.friend-links.index.title" />), key: "to-/admin/friend-links" },
        { children: (<FormattedMessage id="nut.admin.users.index.title" />), key: "to-/admin/users" },
        { children: (<FormattedMessage id="nut.admin.votes.index.title" />), key: "to-/admin/votes" },
      ],
      icon: 'setting',
      key: 'site',
      label: (<FormattedMessage id="nut.dashboard.site.title" />),
    })
  }

  if (process.env.REACT_APP_FEATURE_ALBUM) {
    items.push({
      children: [],
      icon: 'picture',
      key: 'album',
      label: (<FormattedMessage id="album.dashboard.title" />),
    })
  }

  if (process.env.REACT_APP_FEATURE_FORUM) {
    items.push({
      children: [
        { children: (<FormattedMessage id="forum.topics.index.title" />), key: "to-/forum/topics" },
        { children: (<FormattedMessage id="forum.posts.index.title" />), key: "to-/forum/posts" },
      ],
      icon: 'snippets',
      key: 'forum',
      label: (<FormattedMessage id="forum.dashboard.title" />),
    })
  }

  if (process.env.REACT_APP_FEATURE_SURVEY) {
    items.push({
      children: [],
      icon: 'form',
      key: 'survey',
      label: (<FormattedMessage id="survey.dashboard.title" />),
    })
  }

  if (process.env.REACT_APP_FEATURE_CBETA) {
    items.push({
      children: [],
      icon: 'read',
      key: 'cbeta',
      label: (<FormattedMessage id="cbeta.dashboard.title" />),
    })
  }

  if (process.env.REACT_APP_FEATURE_SCHEDULE) {
    items.push({
      children: [],
      icon: 'schedule',
      key: 'schedule',
      label: (<FormattedMessage id="schedule.dashboard.title" />),
    })
  }
  if (process.env.REACT_APP_FEATURE_SHOPPING) {
    items.push({
      children: [],
      icon: 'shopping-cart',
      key: 'shopping',
      label: (<FormattedMessage id="shopping.dashboard.title" />),
    })
  }

  if (process.env.REACT_APP_FEATURE_CALENDAR) {
    items.push({
      children: [],
      icon: 'calendar',
      key: 'calendar',
      label: (<FormattedMessage id="calendar.dashboard.title" />),
    })
  }

  if (havePermission(user, RoleTypes.ADMIN)) {

    if (process.env.REACT_APP_FEATURE_DONATE) {
      items.push({
        children: [],
        icon: 'money-collect',
        key: 'donate',
        label: (<FormattedMessage id="donate.dashboard.title" />),
      })
    }

    if (process.env.REACT_APP_FEATURE_OPS_VPN) {
      items.push({
        children: [],
        icon: 'cloud',
        key: 'ops.vpn',
        label: (<FormattedMessage id="ops.vpn.dashboard.title" />),
      })
    }

    if (process.env.REACT_APP_FEATURE_OPS_EMAIL) {
      items.push({
        children: [],
        icon: 'mail',
        key: 'ops.email',
        label: (<FormattedMessage id="ops.email.dashboard.title" />),
      })
    }

    if (process.env.REACT_APP_FEATURE_SURVEY
      || process.env.REACT_APP_CALENDAR
      || process.env.REACT_APP_FEATURE_SCHEDULE
      || process.env.REACT_APP_FEATURE_DONATE
      || process.env.REACT_APP_FEATURE_SHOPPING
    ) {
      items.push({
        children: [],
        icon: 'idcard',
        key: 'vip',
        label: (<FormattedMessage id="vip.dashboard.title" />),
      })
    }

  }

  return items
}


class Widget extends React.Component<RouteComponentProps<any> & InjectedIntlProps & IProps, IState> {
  public static propTypes: React.ValidationMap<any> = {
    intl: intlShape.isRequired,
  }
  constructor(props: RouteComponentProps<any> & InjectedIntlProps & IProps) {
    super(props)
    this.state = {
      collapsed: false,
    }
  }
  public handleMenuItem = (e: ClickParam) => {
    const { history, refresh, site, intl, signOut } = this.props
    const key = e.key

    const to = 'to-'
    if (key.startsWith(to)) {
      history.push(key.substring(to.length))
      return
    }

    const lang = 'lang-'
    if (key.startsWith(lang)) {
      setLocale(key.substring(lang.length))
      window.location.reload()
      return
    }

    switch (key) {
      case 'home':
        window.open('/', '_blank')
        return
      case 'doc':
        window.open('https://github.com/saturn-xiv/arete/issues', '_blank')
        return
      case 'reload':
        window.location.reload()
        return
      case 'toggle':
        this.setState({
          collapsed: !this.state.collapsed
        })
        return
      case 'search':
        return
      case 'sign-out':
        Modal.confirm({
          title: intl.formatMessage({ id: 'nut.users.sign-out.sure' }),
          onOk() {
            httpDelete('/users/sign-out')
              .then(() => message.success(intl.formatMessage({ id: 'flashes.success' })))
              .catch(message.error)
            signOut()
            refresh(Object.assign({}, site, { who: null }))
            history.push('/users/sign-in')
          }
        });
        return
      default:
        window.console.log(key)
    }
  }
  public componentDidMount() {
    graphql({
      query: `{
  apiVersion,
  currentUser{logo, realName},
  availableLanguage
}`}, (rst: IAbout) => {
        this.props.refresh({
          languages: rst.availableLanguage,
          version: rst.apiVersion,
          who: rst.currentUser,
        })
      })


    const token = getToken()
    if (token) {
      this.props.signIn(token)
    }
  }
  public render() {
    const { who } = this.props.site
    return (<Layout>
      <Sider breakpoint="lg" collapsedWidth="0" trigger={null} collapsible={true} collapsed={this.state.collapsed}>
        {who && (<div className="header-logo">
          <a href="/" target="_blank">
            <img src={who.logo} />
            <h1>{who.realName}</h1>
          </a>
        </div>)}
        <Menu onClick={this.handleMenuItem} theme="dark" mode="inline" defaultSelectedKeys={[]}>
          {
            siderBar(this.props.user).map((it) => (<Menu.SubMenu
              key={it.key}
              title={(<span><Icon type={it.icon} />{it.label}</span>)}>
              {it.children.map((jt) => (<Menu.Item key={jt.key}>{jt.children}</Menu.Item>))}
            </Menu.SubMenu>))
          }
        </Menu>
      </Sider>
      <Layout>
        <Header className="header-bar">
          <Menu onClick={this.handleMenuItem} mode="horizontal">
            <Menu.Item key='toggle'>
              <Icon className="trigger" type={this.state.collapsed ? 'menu-unfold' : 'menu-fold'} />
            </Menu.Item>
            <Menu.Item key='hi'>
              <span />
            </Menu.Item>
            {
              headerBar(this.props.user).map((it) => (<Menu.Item
                className="pull-right"
                key={it.key}>
                {it.children}
              </Menu.Item>))
            }
            <Menu.SubMenu className="pull-right" key="switch-languages" title={<Icon type="global" />}>
              {
                this.props.site.languages.map((it) => (<Menu.Item key={`lang-${it}`}>
                  <FormattedMessage id={`languages.${it}`} />
                </Menu.Item>))
              }
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
  refresh: (info: ISiteState) => dispatch(siteRefresh(info)),
  signIn: (token: string) => dispatch(userSignIn(token)),
  signOut: () => dispatch(userSignOut()),
})

export default withRouter(connect(
  mapStateToProps,
  mapDispatchToProps
)(injectIntl(Widget)))
