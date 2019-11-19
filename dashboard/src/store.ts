import { createStore } from "redux";

import reducers from "./reducers";

export type AppState = ReturnType<typeof reducers>;

export default createStore(reducers);
