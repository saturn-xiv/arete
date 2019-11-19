import { MessageBarType } from "office-ui-fabric-react";

export const SITE_REFRESH = "site.refresh";
export const USER_SIGN_IN = "user.sign-in";
export const USER_SIGN_OUT = "user.sign-out";
export const MESSAGE_BAR = "message-bar";

export interface IMessageBar {
  type?: MessageBarType;
  body?: string;
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

export interface IMessageBarAction {
  type: typeof MESSAGE_BAR;
  payload: IMessageBar;
}

export type ActionTypes =
  | ISiteRefreshAction
  | IUserSignInAction
  | IUserSignOutAction
  | IMessageBarAction;

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

export const showMessage = (msg: IMessageBar): ActionTypes => {
  return {
    type: MESSAGE_BAR,
    payload: msg
  };
};
