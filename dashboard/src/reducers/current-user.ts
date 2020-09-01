import jwt_decode from "jwt-decode";

import { ActionTypes, IUser, USER_SIGN_IN, USER_SIGN_OUT } from "../actions";
import { set as setToken, remove as removeToken } from "../utils/token";

const initialState: IUser = {};

interface Token {
  uid: string;
  sub: string;
}

export default (state = initialState, action: ActionTypes): IUser => {
  switch (action.type) {
    case USER_SIGN_IN:
      try {
        var tkn: Token = jwt_decode(action.payload);
        setToken(action.payload);
        return Object.assign({}, state, { uid: tkn.uid, name: tkn.sub });
      } catch (e) {
        console.error(e.message);
        removeToken();
        return {};
      }
    case USER_SIGN_OUT:
      removeToken();
      return {};
    default:
      return state;
  }
};
