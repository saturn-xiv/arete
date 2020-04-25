import React from "react";
import { Provider } from "react-redux";
import { createStore } from "redux";
import { IntlProvider } from "react-intl";

import "./main.css";
import reducers from "./reducers";
import { detect as detectLocale } from "./locales";
import Router from "./Router";

const store = createStore(reducers);
const intl = detectLocale();

const Component: React.FC = () => {
  return (
    <Provider store={store}>
      <IntlProvider locale={intl.locale} messages={intl.messages}>
        <Router basename="/my" />
      </IntlProvider>
    </Provider>
  );
};

export default Component;
