import { combineReducers } from "redux";

import siteInfo from "./site-info";
import currentUser from "./current-user";
import sideBar from "./side-bar";

export default combineReducers({
  sideBar,
  siteInfo,
  currentUser,
});
