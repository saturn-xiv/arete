import validate_js from "validate.js";

export function validate(form: any, constraints: any): string[] {
  var items = Object.values(validate_js(form, constraints)) as string[][];
  return items.reduce((acc, it) => acc.concat(it), new Array<string>());
}

export const CONSTRAIONTS = {
  email: {
    presence: true,
    email: true
  },
  nickname: {
    presence: true,
    length: {
      minimum: 2,
      maximum: 32
    }
  },
  realName: {
    presence: true,
    length: {
      minimum: 2,
      maximum: 32
    }
  },
  password: {
    presence: true,
    length: {
      minimum: 6,
      maximum: 32
    }
  },
  passwordConfirmation: {
    presence: true,
    equality: "password"
  }
};
