import { get as getToken } from "./token";

export const backend = (u: string): string => `/api${u}`;

export const options = (method: string, body?: any): RequestInit => {
  var headers: HeadersInit = new Headers();
  headers.set("Authorization", `Bearer ${getToken()}`);
  // https://github.github.io/fetch/#options
  if (method === "POST") {
    headers.set("Content-Type", "application/json; charset=utf-8");
  }
  var it: RequestInit = {
    method: method,
    // mode: 'cors',
    credentials: "include",
    headers,
  };
  if (body) {
    it.body = JSON.stringify(body);
  }
  return it;
};

export const parse = (res: any) => {
  return res.ok
    ? res.json()
    : res.text().then((err: any) => {
        throw err;
      });
};

export const download = (path: string, name: string) => {
  return fetch(backend(path), options("GET"))
    .then((response) => response.blob())
    .then((blob) => {
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = name;
      const clickHandler = () => {
        setTimeout(() => {
          URL.revokeObjectURL(url);
        }, 150);
      };
      a.addEventListener("click", clickHandler, false);
      a.click();
    });
};

export const get = (path: string) =>
  fetch(backend(path), options("GET")).then(parse);

export const delete_ = (path: string) =>
  fetch(backend(path), options("DELETE")).then(parse);

export const post = (path: string, body: any) =>
  fetch(backend(path), options("POST", body)).then(parse);

export const patch = (path: string, body: any) =>
  fetch(backend(path), options("PATCH", body)).then(parse);

export const put = (path: string, body: any) =>
  fetch(backend(path), options("PUT", body)).then(parse);
