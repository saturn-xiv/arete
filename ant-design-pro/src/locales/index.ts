import Cookies from "js-cookie";
import antdZhCn from "antd/es/locale/zh_CN";
import antdZhTw from "antd/es/locale/zh_TW";
import antdEnUs from "antd/es/locale/en_US";
import moment from "moment";
import "moment/locale/zh-cn";
import "moment/locale/zh-tw";

import enUS from "./en-US";
import zhHans from "./zh-Hans";
import zhHant from "./zh-Hant";

const KEY = "locale";

export const get = () => {
  return localStorage.getItem(KEY) || Cookies.get(KEY) || "en-US";
};

export const set = (lang: string) => {
  localStorage.setItem(KEY, lang);
  Cookies.set(KEY, lang, {
    expires: 1 << 16,
    path: "/"
  });
  window.location.reload();
};

export const detect = () => {
  const lang = get();
  switch (lang) {
    case "zh-Hans":
      moment.locale("zh-cn");
      return { locale: lang, antd: antdZhCn, messages: zhHans };
    case "zh-Hant":
      moment.locale("zh-tw");
      return { locale: lang, antd: antdZhTw, messages: zhHant };
    default:
      moment.locale();
      return {
        locale: "en-US",
        moment: null,
        antd: antdEnUs,
        messages: enUS
      };
  }
};
