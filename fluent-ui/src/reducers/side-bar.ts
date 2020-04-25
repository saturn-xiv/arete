import { ActionTypes, ISideBar, SIDE_BAR_OPEN } from "../actions";

const initialState: ISideBar = {};

export default (state = initialState, action: ActionTypes): ISideBar => {
  switch (action.type) {
    case SIDE_BAR_OPEN:
      return Object.assign({}, state, { menus: action.payload });
    default:
      return state;
  }
};
