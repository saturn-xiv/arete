import { ActionTypes, IMessageBar, MESSAGE_BAR } from "../actions";

const initialState: IMessageBar = {};

export default (state = initialState, action: ActionTypes): IMessageBar => {
  switch (action.type) {
    case MESSAGE_BAR:
      return Object.assign({}, state, action.payload);
    default:
      return state;
  }
};
