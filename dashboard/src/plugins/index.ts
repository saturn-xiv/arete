import nut from "./nut";

export interface IRoute {
  path: string;
  component: any;
}

export interface IPlugin {
  routes: IRoute[];
}

const plugins: IPlugin[] = [nut];

export default plugins;
