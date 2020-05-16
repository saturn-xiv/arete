import React from 'react';
import { useIntl } from 'umi';

export default () => {
  const intl = useIntl();
  return (
    <div>
      <h1>{intl.formatMessage({ id: 'home.title' })}</h1>
    </div>
  );
};
