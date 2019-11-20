import { get as getToken } from "./token";

export const PAGE_SIZE = 20;

export const backend = (u: string) => `/api${u}`;

export function options(method: string): RequestInit {
  return {
    credentials: "include",
    headers: {
      Accept: "application/json",
      Authorization: `Bearer ${getToken()}`,
      "Content-Type": "application/json; charset=utf-8"
    },
    method
  };
}

export const get = (path: string) =>
  fetch(backend(path), options("GET")).then(res =>
    res.status === 200
      ? res.json()
      : res.text().then(err => {
          throw err;
        })
  );

export const delete_ = (path: string) =>
  fetch(backend(path), options("DELETE")).then(res =>
    res.status === 200
      ? res.json()
      : res.text().then(err => {
          throw err;
        })
  );

// https://github.github.io/fetch/#options
export const post = (path: string, body: any) => {
  const data = options("POST");
  data.body = JSON.stringify(body);
  return fetch(backend(path), data).then(res =>
    res.status === 200
      ? res.json()
      : res.text().then(err => {
          throw err;
        })
  );
};

export const patch = (path: string, body: any) => {
  const data = options("PATCH");
  data.body = JSON.stringify(body);
  return fetch(backend(path), data).then(res =>
    res.status === 200
      ? res.json()
      : res.text().then(err => {
          throw err;
        })
  );
};

export const put = (path: string, body: any) => {
  const data = options("PUT");
  data.body = JSON.stringify(body);
  return fetch(backend(path), data).then(res =>
    res.status === 200
      ? res.json()
      : res.text().then(err => {
          throw err;
        })
  );
};
