import { IMessageBar } from "./components/MessageBar";

export const SITE_REFRESH = "site.refresh";
export const USER_SIGN_IN = "user.sign-in";
export const USER_SIGN_OUT = "user.sign-out";
export const SHOW_MESSAGE_BAR = "message-bar.show";
export const HIDE_MESSAGE_BAR = "message-bar.hide";

export interface IState {
  siteInfo: ISite;
  currentUser: IUser;
  messageBar: IMessageBar;
}

export interface ISite {
  title?: string;
  subhead?: string;
  copyright?: string;
  version?: string;
}

export interface IRole {}

export interface IUser {
  uid?: string;
  real_name?: string;
  nick_name?: string;
  email?: string;
  roles?: IRole[];
}

export interface ISiteRefreshAction {
  type: typeof SITE_REFRESH;
  payload: ISite;
}

export interface IUserSignInAction {
  type: typeof USER_SIGN_IN;
  payload: IUser;
}

export interface IUserSignOutAction {
  type: typeof USER_SIGN_OUT;
}

export interface IMessageBarShowAction {
  type: typeof SHOW_MESSAGE_BAR;
  payload: IMessageBar;
}

export interface IMessageBarHideAction {
  type: typeof HIDE_MESSAGE_BAR;
}
export type ActionTypes =
  | ISiteRefreshAction
  | IUserSignInAction
  | IUserSignOutAction
  | IMessageBarHideAction
  | IMessageBarShowAction;

export const refresh = (info: ISite): ActionTypes => {
  return {
    type: SITE_REFRESH,
    payload: info
  };
};

export const signIn = (user: IUser): ActionTypes => {
  return {
    type: USER_SIGN_IN,
    payload: user
  };
};

export const signOut = (): ActionTypes => {
  return {
    type: USER_SIGN_OUT
  };
};

export const showMessageBar = (msg: IMessageBar): ActionTypes => {
  return {
    type: SHOW_MESSAGE_BAR,
    payload: msg
  };
};

export const hideMessageBar = (): ActionTypes => {
  return {
    type: HIDE_MESSAGE_BAR
  };
};
