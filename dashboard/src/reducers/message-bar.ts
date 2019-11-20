import { ActionTypes, SHOW_MESSAGE_BAR, HIDE_MESSAGE_BAR } from "../actions";
import { IMessageBar } from "../components/MessageBar";

const initialState: IMessageBar = { type: undefined, messages: undefined };

export default (state = initialState, action: ActionTypes): IMessageBar => {
  switch (action.type) {
    case SHOW_MESSAGE_BAR:
      return Object.assign({}, state, action.payload);
    case HIDE_MESSAGE_BAR:
      return {};
    default:
      return state;
  }
};
