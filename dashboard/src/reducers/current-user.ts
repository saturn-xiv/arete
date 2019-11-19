import { ActionTypes, IUser, USER_SIGN_IN, USER_SIGN_OUT } from "../actions";

const initialState: IUser = {};

export default (state = initialState, action: ActionTypes): IUser => {
  switch (action.type) {
    case USER_SIGN_IN:
      return Object.assign({}, state, action.payload);
    case USER_SIGN_OUT:
      return {};
    default:
      return state;
  }
};
