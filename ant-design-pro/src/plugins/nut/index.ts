import { IRoute } from "..";

const routes: IRoute[] = [
  {
    path: "/users/sign-in",
    component: () => import("./users/SignIn")
  },
  {
    path: "/install",
    component: () => import("./Install")
  }
];

export default routes;
