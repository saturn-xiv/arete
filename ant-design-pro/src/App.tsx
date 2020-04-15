import React from "react";
import { Provider } from "react-redux";
import { createStore } from "redux";
import { IntlProvider } from "react-intl";
import { ConfigProvider } from "antd";

import "./main.css";
import reducers from "./reducers";
import { detect as detectLocale } from "./locales";
import Router from "./Router";

const store = createStore(reducers);
const intl = detectLocale();

const App: React.FC = () => {
  return (
    <Provider store={store}>
      <IntlProvider locale={intl.locale} messages={intl.messages}>
        <ConfigProvider locale={intl.antd}>
          <Router basename="/my" />
        </ConfigProvider>
      </IntlProvider>
    </Provider>
  );
};

export default App;
