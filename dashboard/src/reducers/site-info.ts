import { ActionTypes, ISite, SITE_REFRESH } from "../actions";

const initialState: ISite = {};

export default (state = initialState, action: ActionTypes): ISite => {
  switch (action.type) {
    case SITE_REFRESH:
      return Object.assign({}, state, action.payload);
    default:
      return state;
  }
};
