import { combineReducers } from "redux";

import siteInfo from "./site-info";
import currentUser from "./current-user";
import messageBar from "./message-bar";

export default combineReducers({
  siteInfo,
  currentUser,
  messageBar
});
