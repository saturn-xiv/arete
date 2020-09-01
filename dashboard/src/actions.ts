export const USER_SIGN_IN = "user.sign-in";
export const USER_SIGN_OUT = "user.sign-out";
export const SITE_REFRESH = "site.refresh";
export const SIDE_BAR_OPEN = "side-bar.open";

export interface IState {
  siteInfo: ISite;
  currentUser: IUser;
  sideBar: ISideBar;
}
export interface ISite {
  title?: string;
  subhead?: string;
  copyright?: string;
  languages?: string[];
  version?: string;
  uptime?: string;
}

export interface ISideBar {
  current?: string;
  menus?: string[];
}
export interface IUser {
  uid?: string;
  name?: string;
}

export interface ISiteRefreshAction {
  type: typeof SITE_REFRESH;
  payload: ISite;
}

export interface IUserSignInAction {
  type: typeof USER_SIGN_IN;
  payload: string;
}

export interface IUserSignOutAction {
  type: typeof USER_SIGN_OUT;
}

export interface ISideBarOpenAction {
  type: typeof SIDE_BAR_OPEN;
  payload: string[];
}

export type ActionTypes =
  | ISiteRefreshAction
  | IUserSignInAction
  | IUserSignOutAction
  | ISideBarOpenAction;

export const refresh = (payload: ISite): ActionTypes => {
  return {
    type: SITE_REFRESH,
    payload,
  };
};

export const signIn = (payload: string): ActionTypes => {
  return {
    type: USER_SIGN_IN,
    payload,
  };
};

export const signOut = (): ActionTypes => {
  return {
    type: USER_SIGN_OUT,
  };
};

export const openSideBar = (payload: string[]): ActionTypes => {
  return {
    type: SIDE_BAR_OPEN,
    payload,
  };
};
