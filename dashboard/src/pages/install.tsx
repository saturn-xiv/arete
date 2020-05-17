import React, { useState } from 'react';
import { useIntl } from 'umi';
import { TextField } from 'office-ui-fabric-react/lib/TextField';
import { Helmet } from 'react-helmet';

import Form from '@/components/Form';

export default () => {
  const intl = useIntl();
  const title = intl.formatMessage({ id: 'install.title' });

  const [username, setUsername] = useState('');
  const [nickname, setNickname] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [passwordConfirmation, setPasswordConfirmation] = useState('');

  const onReset = () => {
    setUsername('');
    setNickname('');
    setEmail('');
    setPassword('');
    setPasswordConfirmation('');
  };
  const onSubmit = () => {
    console.log('on submit', nickname);
  };
  return (
    <>
      <Helmet>
        <title>{title}</title>
      </Helmet>
      <Form onSubmit={onSubmit} onReset={onReset} title={title}>
        <TextField
          value={username}
          onChange={event =>
            setUsername((event.target as HTMLInputElement).value)
          }
          label={intl.formatMessage({ id: 'form.fields.username' })}
        />
        <TextField
          value={nickname}
          onChange={event =>
            setNickname((event.target as HTMLInputElement).value)
          }
          label={intl.formatMessage({ id: 'form.fields.nickname' })}
        />
        <TextField
          value={email}
          onChange={event => setEmail((event.target as HTMLInputElement).value)}
          label={intl.formatMessage({ id: 'form.fields.email' })}
        />
        <TextField
          value={password}
          onChange={event =>
            setPassword((event.target as HTMLInputElement).value)
          }
          label={intl.formatMessage({ id: 'form.fields.password' })}
        />
        <TextField
          value={passwordConfirmation}
          onChange={event =>
            setPasswordConfirmation((event.target as HTMLInputElement).value)
          }
          label={intl.formatMessage({
            id: 'form.fields.password-confirmation',
          })}
        />
      </Form>
    </>
  );
};
