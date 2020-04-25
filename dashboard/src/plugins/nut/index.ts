import { IMenu } from "..";

const items: IMenu[] = [
  {
    path: "/",
    component: () => import("./Home"),
    title: { id: "nut.home.title" },
    hidden: true,
  },
  {
    path: "/install",
    component: () => import("./Install"),
    title: { id: "nut.install.title" },
    hidden: true,
  },
  {
    path: "/users/sign-in",
    component: () => import("./users/SignIn"),
    title: { id: "nut.users.sign-in.title" },
    hidden: true,
  },
  {
    path: "/users/sign-up",
    component: () => import("./users/SignUp"),
    title: { id: "nut.users.sign-up.title" },
    hidden: true,
  },
];

export default items;
