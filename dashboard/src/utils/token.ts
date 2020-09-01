import jwt_decode from "jwt-decode";

const KEY = "token";

export const get = () => {
  return sessionStorage.getItem(KEY);
};

export const set = (token: string) => {
  sessionStorage.setItem(KEY, token);
};

export const remove = () => {
  sessionStorage.removeItem(KEY);
};

export interface IToken {
  uid: string;
  sub: string;
}

export const parse = (token: string): IToken | undefined => {
  try {
    var tkn: IToken = jwt_decode(token);
    return tkn;
  } catch (e) {
    console.error(e.message);
  }
};
