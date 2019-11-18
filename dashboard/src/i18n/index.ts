import * as Cookies from "js-cookie";
import "moment/locale/zh-cn";
import "moment/locale/zh-tw";

import enUs from "./en-US";
import zhHans from "./zh-Hans";
import zhHant from "./zh-Hant";

const KEY = "locale";
const ENGLISH = "en-US";

export const set = (l: string) => {
  Cookies.set(KEY, l, {
    expires: Math.pow(2, 16),
    path: "/"
  });
  localStorage.setItem(KEY, l);
};

export const get = () => {
  const locale = Cookies.get(KEY) || localStorage.getItem(KEY) || ENGLISH;
  switch (locale) {
    case "zh-Hans":
      return { locale, messages: zhHans };
    case "zh-Hant":
      return { locale, messages: zhHant };
    default:
      return { locale: ENGLISH, messages: enUs };
  }
};
