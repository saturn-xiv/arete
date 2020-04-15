import nut from "./nut";
import album from "./album";
import cbeta from "./cbeta";
import forum from "./forum";
import ops_mail from "./ops/mail";
import ops_vpn from "./ops/vpn";
import survey from "./survey";
import vip from "./vip";

export interface IRoute {
  path: string;
  component: any;
}

const routes = new Array<IRoute>()
  .concat(album)
  .concat(cbeta)
  .concat(forum)
  .concat(ops_mail)
  .concat(ops_vpn)
  .concat(survey)
  .concat(vip)
  .concat(nut);

export default routes;
