import validate_js from "validate.js";
import { MessageBarType } from "office-ui-fabric-react";

export interface IMessage {
  type: MessageBarType;
  body: string[];
}

export function validate(form: any, constraints: any): string[] | undefined {
  var items = (Object.values(
    validate_js(form, constraints)
  ) as string[][]).reduce((acc, it) => acc.concat(it), new Array<string>());
  return items.length > 0 ? items : undefined;
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
