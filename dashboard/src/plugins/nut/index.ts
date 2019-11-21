import { IPlugin } from "..";

const Plugin: IPlugin = {
  routes: [
    { path: "/", component: () => import("./Home") },
    { path: "/install", component: () => import("./Install") },

    { path: "/users/sign-in", component: () => import("./users/SignIn") },
    { path: "/users/sign-up", component: () => import("./users/SignUp") },
    { path: "/users/confirm", component: () => import("./users/Confirm") },
    {
      path: "/users/forgot-password",
      component: () => import("./users/ForgotPassword")
    },
    { path: "/users/unlock", component: () => import("./users/Unlock") }
  ]
};

export default Plugin;
