import React from "react";
import { useIntl } from "react-intl";
import { useHistory } from "react-router";
import { Stack, Nav, INavLink, INavLinkGroup } from "office-ui-fabric-react";

import Footer from "../Footer";
import Title from "../Title";
import HeaderBar from "./HeaderBar";

interface IProps {
  children: React.ReactNode;
  title: string;
}

const Component = ({ title, children }: IProps) => {
  const intl = useIntl();
  const history = useHistory();

  const items: INavLinkGroup[] = [];

  items.push({
    name: intl.formatMessage({ id: "nut.network.self.title" }),
    links: [
      {
        name: intl.formatMessage({ id: "nut.network.eth.title" }),
        url: "/network/eth",
      },
      {
        name: intl.formatMessage({ id: "nut.network.wlan.title" }),
        url: "/network/wlan",
      },
      {
        name: intl.formatMessage({ id: "nut.network.ping.title" }),
        url: "/network/ping",
      },
    ],
  });

  const server = [
    {
      name: intl.formatMessage({ id: "nut.ntp.title" }),
      url: "/ntp",
    },
    {
      name: intl.formatMessage({ id: "nut.vpn.title" }),
      url: "/vpn",
    },
  ];

  const settings = [
    {
      name: intl.formatMessage({ id: "nut.attachments.index.title" }),
      url: "/attachments",
    },
  ];

  const tools = [
    {
      name: intl.formatMessage({ id: "nut.users.profile.title" }),
      url: "/users/profile",
    },
    {
      name: intl.formatMessage({ id: "nut.logs.title" }),
      url: "/users/logs",
    },
    {
      name: intl.formatMessage({ id: "nut.status.title" }),
      url: "/status",
    },
  ];

  const feature = process.env.REACT_APP_FEATURE;
  if (feature) {
    if (feature === "shake-alert-actone") {
      server.push({
        name: intl.formatMessage({ id: "shake-alert.server.title" }),
        url: "/shake-alert/server",
      });
    }
    if (feature === "scratch") {
      settings.push({
        name: intl.formatMessage({ id: "scratch.server.title" }),
        url: "/scratch/server",
      });
    }
    if (feature === "mini-web") {
      settings.push({
        name: intl.formatMessage({ id: "mini-web.server.title" }),
        url: "/mini-web/server",
      });
    }
    if (feature.startsWith("syn-apps-revolution-")) {
      settings.push({
        name: intl.formatMessage({ id: "syn-apps.revolution.title" }),
        url: "/syn-apps/revolution",
      });
    }
    if (feature === "singlewire-icmobile-react-v3") {
      settings.push({
        name: intl.formatMessage({
          id: "singlewire.icmobile.react.rss.title",
        }),
        url: "/singlewire/icmobile/react/rss",
      });
    }
    if (
      feature === "eas-react-v2" ||
      feature === "eas-react-v3" ||
      feature === "capsol-react-v2" ||
      feature === "capsol-react-v3"
    ) {
      settings.push({
        name: intl.formatMessage({ id: "eas.react.server.title" }),
        url: "/eas/react/server",
      });
    }
    if (
      feature === "eas-react-v2" ||
      feature === "eas-react-v3" ||
      feature === "capsol-react-v2" ||
      feature === "capsol-react-v3" ||
      feature === "singlewire-icmobile-react-v3"
    ) {
      settings.push({
        name: intl.formatMessage({ id: "eas.token.title" }),
        url: "/eas/token",
      });
      settings.push({
        name: intl.formatMessage({ id: "eas.standard.title" }),
        url: "/eas/react/standard",
      });
      settings.push({
        name: intl.formatMessage({ id: "eas.raw.title" }),
        url: "/eas/react/raw",
      });
    }
    if (
      feature === "singlewire-icmobile-actone" ||
      feature === "eas-actone" ||
      feature === "capsol-actone" ||
      feature === "shake-alert-actone"
    ) {
      settings.push({
        name: intl.formatMessage({ id: "eas.token.title" }),
        url: "/eas/token",
      });
      settings.push({
        name: intl.formatMessage({ id: "eas.raw.title" }),
        url: "/eas/actone/raw",
      });
    }
    if (feature === "singlewire-icmobile-actone") {
      settings.push({
        name: intl.formatMessage({ id: "actone.buttons.title" }),
        url: "/singlewire/icmobile/actone/buttons",
      });
    }
    if (
      feature === "eas-actone" ||
      feature === "capsol-actone" ||
      feature === "shake-alert-actone"
    ) {
      settings.push({
        name: intl.formatMessage({ id: "actone.buttons.title" }),
        url: "/actone/buttons",
      });
      settings.push({
        name: intl.formatMessage({ id: "actone.messages.index.title" }),
        url: "/eas/actone/messages",
      });
      settings.push({
        name: intl.formatMessage({ id: "eas.actone.groups.index.title" }),
        url: "/eas/actone/groups",
      });
      settings.push({
        name: intl.formatMessage({ id: "eas.actone.signs.index.title" }),
        url: "/eas/actone/signs",
      });
      settings.push({
        name: intl.formatMessage({ id: "eas.actone.bind.title" }),
        url: "/eas/actone/bind",
      });
      settings.push({
        name: intl.formatMessage({ id: "eas.actone.publish.title" }),
        url: "/eas/actone/publish",
      });
    }
  }

  items.push({
    name: intl.formatMessage({ id: "sider.server" }),
    links: server,
  });
  items.push({
    name: intl.formatMessage({ id: "sider.tools" }),
    links: tools,
  });
  items.push({
    name: intl.formatMessage({ id: "sider.settings" }),
    links: settings,
  });

  return (
    <Stack horizontal>
      <Stack.Item align="start">
        <Nav
          onLinkClick={(
            ev?: React.MouseEvent<HTMLElement>,
            item?: INavLink
          ) => {
            ev?.preventDefault();
            if (item) {
              history.push(item.url);
            }
          }}
          groups={items}
        />
      </Stack.Item>
      <Stack.Item grow>
        <Stack
          styles={{
            root: { marginLeft: 40 },
          }}
        >
          <Stack.Item>
            <HeaderBar />
          </Stack.Item>
          {children}
          <Footer />
          <Title value={title} />
        </Stack>
      </Stack.Item>
    </Stack>
  );
};

export default Component;
