import nut from "./nut";
import forum from "./forum";

export interface IRoute {
  path: string;
  component: any;
}

const routes = new Array<IRoute>().concat(forum).concat(nut);

export default routes;
