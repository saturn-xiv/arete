const KEY = 'token';

export const get = () => {
  return sessionStorage.getItem(KEY);
};

export const set = (token: string) => {
  sessionStorage.setItem(KEY, token);
};

export const remove = () => {
  sessionStorage.removeItem(KEY);
};
