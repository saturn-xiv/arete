import validate_js from "validate.js";
import URI from "urijs";

export const DATETIME_FORMAT = "LL LTS";

export const HOME = new URI(window.location.href).origin();

export function validate(form: any, constraints: any): string[] | undefined {
  var rs = validate_js(form, constraints);
  if (rs) {
    return Object.values(rs as string[][]).reduce(
      (acc, it) => acc.concat(it),
      new Array<string>()
    );
  }
}

export const CONSTRAIONTS = {
  email: {
    presence: true,
    email: true
  },
  nickName: {
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
