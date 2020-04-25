import { MessageDescriptor } from "react-intl";

import nut from "./nut";
import album from "./album";
import cbeta from "./cbeta";
import forum from "./forum";
import ops_mail from "./ops/mail";
import ops_vpn from "./ops/vpn";
import ops_monitor from "./ops/monitor";
import survey from "./survey";
import vip from "./vip";

export interface IMenu {
  path: string;
  title: MessageDescriptor;
  component: any;
  children?: IMenu[];
  hidden?: boolean;
  authority?: string[];
}

const menus = new Array<IMenu>()
  .concat(album)
  .concat(cbeta)
  .concat(forum)
  .concat(ops_mail)
  .concat(ops_vpn)
  .concat(ops_monitor)
  .concat(survey)
  .concat(vip)
  .concat(nut);

export default menus;
