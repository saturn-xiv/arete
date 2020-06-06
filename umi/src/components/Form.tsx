import React, { ReactNode, FormEventHandler, MouseEventHandler } from 'react';
import { useIntl } from 'umi';
import { Text } from 'office-ui-fabric-react/lib/Text';
import {
  Button,
  DefaultButton,
  PrimaryButton,
  Stack,
  IStackTokens,
} from 'office-ui-fabric-react';

export const buttonStackTokens: IStackTokens = { childrenGap: 40 };
export const fieldStackTokens: IStackTokens = { childrenGap: 12 };

export const trim = (o: any): any =>
  Object.keys(o).reduce((acc: any, key) => {
    const val = o[key];
    acc[key] = key.startsWith('password') ? val : val.trim();
    return acc;
  }, {});

export const USERNAME = {
  presence: true,
  length: {
    minimum: 2,
    maximum: 32,
  },
};

export const EMAIL = {
  presence: true,
  email: true,
  length: {
    maximum: 255,
  },
};

export const NICKNAME = {
  presence: true,
  format: {
    pattern: '^[a-zA-Z0-9_-]{2,32}$',
    flags: 'i',
  },
};

export const PASSWORD = {
  presence: true,
  length: {
    minimum: 6,
    maximum: 32,
  },
};

export interface IProps {
  title: string;
  children: ReactNode;
  onReset: MouseEventHandler<Button>;
  onSubmit: FormEventHandler;
}

export default ({ title, onReset, onSubmit, children }: IProps) => {
  const intl = useIntl();
  return (
    <form onSubmit={onSubmit}>
      <Stack tokens={fieldStackTokens}>
        <Text variant="xxLarge" nowrap block>
          {title}
        </Text>
        {children}
        <Stack horizontal tokens={buttonStackTokens}>
          <DefaultButton
            onClick={onReset}
            text={intl.formatMessage({ id: 'form.buttons.reset' })}
          />
          <PrimaryButton
            type="submit"
            text={intl.formatMessage({ id: 'form.buttons.submit' })}
          />
        </Stack>
      </Stack>
    </form>
  );
};
