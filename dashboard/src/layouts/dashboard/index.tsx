import React, { Component } from "react";
import { connect } from "react-redux";
import {
  injectIntl,
  FormattedMessage,
  WrappedComponentProps
} from "react-intl";
import { Icon, Layout, Menu, message, Modal } from "antd";
import { ClickParam, SelectParam } from "antd/lib/menu";
import { RouteComponentProps, withRouter } from "react-router";

import {
  signIn,
  signOut,
  IState as IApplicationState,
  IUser,
  ISideBar,
  openSideBar,
  ISite
} from "../../actions";
import { get as getToken } from "../../utils/token";
import { set as setLocale } from "../../locales";
import { delete_ as httpDelete } from "../../utils/request";
import Footer from "../Footer";
import Title from "../Title";

const { Header, Sider, Content } = Layout;

interface IHeaderBarItem {
  icon: string;
  id: string;
}

interface ISiderBarMenu {
  icon: string;
  label: string;
  to: string;
  items: ISiderBarItem[];
}

interface ISiderBarItem {
  label: string;
  to: string;
}

interface IProps {
  children: React.ReactNode;
  currentUser: IUser;
  sideBar: ISideBar;
  siteInfo: ISite;
  signIn: typeof signIn;
  signOut: typeof signOut;
  openSideBar: typeof openSideBar;
  title: string;
}

interface IState {
  collapsed: boolean;
}

class Widget extends Component<
  RouteComponentProps<any> & WrappedComponentProps & IProps,
  IState
> {
  constructor(
    props: RouteComponentProps<any> & WrappedComponentProps & IProps
  ) {
    super(props);
    this.state = {
      collapsed: false
    };
  }
  handleHeaderBar = (e: ClickParam) => {
    const { history, signOut, siteInfo } = this.props;
    const { formatMessage } = this.props.intl;

    const lang = "lang-";
    if (e.key.startsWith(lang)) {
      setLocale(e.key.substring(lang.length));
      return;
    }

    switch (e.key) {
      case "home":
        window.open("/", "_blank");
        return;
      case "github":
        window.open(siteInfo.homepage, "_blank");
        return;
      case "toggle":
        this.setState({
          collapsed: !this.state.collapsed
        });
        return;
      case "users.sign-out":
        Modal.confirm({
          title: formatMessage({ id: "header.sign-out.confirm" }),
          onOk() {
            httpDelete("/users/sign-out")
              .then(() => {
                history.push("/users/sign-in");
                message.info(formatMessage({ id: "flashes.success" }));
                signOut();
              })
              .catch(message.error);
          }
        });
        return;
      default:
        console.error(e.key);
    }
  };
  siderMenus = (): ISiderBarMenu[] => {
    return [];
  };
  headerMenus = (): IHeaderBarItem[] => {
    return [
      { id: "users.sign-out", icon: "logout" },
      { id: "github", icon: "github" },
      { id: "home", icon: "home" }
    ];
  };
  componentDidMount() {
    const { history, signIn, currentUser } = this.props;
    const token = getToken();
    if (token) {
      if (!currentUser.uid) {
        signIn(token);
      }
    } else {
      history.push("/users/sign-in");
    }
  }
  handlerSiderBarSelect = (it: SelectParam) => {
    this.props.history.push(it.key);
  };
  handlerSiderBarOpenChange = (items: string[]) => {
    this.props.openSideBar(items);
  };
  public render() {
    const { children, title, sideBar, siteInfo, currentUser } = this.props;
    return (
      <Layout>
        <Sider
          breakpoint="lg"
          collapsedWidth="0"
          trigger={null}
          collapsible={true}
          collapsed={this.state.collapsed}
        >
          <div className="sider-logo" />
          <Menu
            theme="dark"
            mode="inline"
            onSelect={this.handlerSiderBarSelect}
            onOpenChange={this.handlerSiderBarOpenChange}
            openKeys={sideBar.menus || []}
          >
            {this.siderMenus().map(it => (
              <Menu.SubMenu
                key={it.label}
                title={
                  <span>
                    <Icon type={it.icon} />
                    <FormattedMessage id={it.label} />
                  </span>
                }
              >
                {it.items.map(jt => (
                  <Menu.Item key={jt.to}>
                    <FormattedMessage id={jt.label} />
                  </Menu.Item>
                ))}
              </Menu.SubMenu>
            ))}
          </Menu>
        </Sider>
        <Layout>
          <Header
            style={{
              background: "#fff",
              padding: 0
            }}
          >
            <Menu onClick={this.handleHeaderBar} mode="horizontal">
              <Menu.Item key="toggle">
                <Icon
                  className="trigger"
                  type={this.state.collapsed ? "menu-unfold" : "menu-fold"}
                />
              </Menu.Item>
              {this.headerMenus().map(it => (
                <Menu.Item
                  style={{
                    float: "right"
                  }}
                  key={it.id}
                >
                  <Icon type={it.icon} />
                </Menu.Item>
              ))}
              <Menu.SubMenu
                style={{
                  float: "right"
                }}
                key="switch-languages"
                title={<Icon type="global" />}
              >
                {siteInfo.languages.map(it => (
                  <Menu.Item key={`lang-${it}`}>
                    <FormattedMessage id={`languages.${it}`} />
                  </Menu.Item>
                ))}
              </Menu.SubMenu>
            </Menu>
          </Header>
          <Content
            style={{
              margin: "24px 16px",
              padding: 24,
              background: "#fff",
              minHeight: 360
            }}
          >
            {children}
            <Title value={title} />
          </Content>
          <Footer />
        </Layout>
      </Layout>
    );
  }
}

export default withRouter(
  connect(
    ({ currentUser, siteInfo, sideBar }: IApplicationState) => ({
      currentUser,
      sideBar,
      siteInfo
    }),
    { signIn, signOut, openSideBar }
  )(injectIntl(Widget))
);
