import { IPlugin } from "..";

const Plugin: IPlugin = {
  routes: [
    { path: "/", component: () => import("./Home") },
    { path: "/install", component: () => import("./Install") },
    { path: "/users/sign-in", component: () => import("./users/SignIn") },
    { path: "/users/sign-up", component: () => import("./users/SignUp") }
  ]
};

export default Plugin;
