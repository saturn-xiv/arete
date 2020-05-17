import React, { ReactNode, MouseEventHandler } from 'react';
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

export interface IProps {
  title: string;
  children: ReactNode;
  onReset: MouseEventHandler<Button>;
  onSubmit: MouseEventHandler<Button>;
}

export default ({ title, onReset, onSubmit, children }: IProps) => {
  const intl = useIntl();
  return (
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
          onClick={onSubmit}
          text={intl.formatMessage({ id: 'form.buttons.submit' })}
        />
      </Stack>
    </Stack>
  );
};
