import React from 'react';
import { useIntl } from 'umi';
import { TextField } from 'office-ui-fabric-react/lib/TextField';
import { Text } from 'office-ui-fabric-react/lib/Text';
import { Helmet } from 'react-helmet';

export default () => {
  const intl = useIntl();
  const title = intl.formatMessage({ id: 'users.sign-up.title' });
  return (
    <>
      <Helmet>
        <title>{title}</title>
      </Helmet>
      <Text variant="xxLarge" nowrap block>
        {title}
      </Text>
      <TextField label="Standard" />
    </>
  );
};
