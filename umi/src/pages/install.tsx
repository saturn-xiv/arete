import React, { FormEvent, useState } from 'react';
import { useIntl } from 'umi';
import { TextField } from 'office-ui-fabric-react/lib/TextField';
import { Helmet } from 'react-helmet';
import validate from 'validate.js';

import Form, {
  USERNAME,
  trim,
  PASSWORD,
  NICKNAME,
  EMAIL,
} from '@/components/Form';

interface IFormData {
  email: string;
  username: string;
  nickname: string;
  password: string;
  passwordConfirmation: string;
}

interface IFormError {
  email?: string[];
  username?: string[];
  nickname?: string[];
  password?: string[];
  passwordConfirmation?: string[];
}

const constraints = {
  email: EMAIL,
  username: USERNAME,
  nickname: NICKNAME,
  password: PASSWORD,
  passwordConfirmation: PASSWORD,
};

export default () => {
  const intl = useIntl();
  const title = intl.formatMessage({ id: 'install.title' });

  const [formError, setFormError] = useState<IFormError>();
  const [formData, setFormData] = useState<IFormData>({
    email: '',
    username: '',
    nickname: '',
    password: '',
    passwordConfirmation: '',
  });

  const onReset = () => {
    setFormData({
      username: '',
      email: '',
      nickname: '',
      password: '',
      passwordConfirmation: '',
    });
  };

  const onSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (!formError) {
      console.log('submit', trim(formData));
    }
  };

  const onValidate = (data: IFormData) => {
    let err = validate(trim(data), constraints);

    if (data.password !== data.passwordConfirmation) {
      err = Object.assign({}, err, {
        passwordConfirmation: [
          intl.formatMessage({
            id: 'form.validate-messages.password-confirmation',
          }),
        ],
      });
    }

    setFormError(err);
  };

  const onChange = (
    event: FormEvent<HTMLInputElement | HTMLTextAreaElement>,
    id: string,
  ) => {
    const data = Object.assign({}, formData, {
      [id]: (event.target as HTMLInputElement).value,
    });
    setFormData(data);
    onValidate(data);
  };

  return (
    <>
      <Helmet>
        <title>{title}</title>
      </Helmet>
      <Form onSubmit={onSubmit} onReset={onReset} title={title}>
        <TextField
          value={formData.username}
          required
          onChange={event => onChange(event, 'username')}
          errorMessage={formError?.username?.join('')}
          label={intl.formatMessage({ id: 'form.fields.username' })}
        />
        <TextField
          value={formData.nickname}
          required
          onChange={event => onChange(event, 'nickname')}
          errorMessage={formError?.nickname?.join('')}
          label={intl.formatMessage({ id: 'form.fields.nickname' })}
        />
        <TextField
          value={formData.email}
          required
          type="email"
          onChange={event => onChange(event, 'email')}
          errorMessage={formError?.email?.join('')}
          label={intl.formatMessage({ id: 'form.fields.email' })}
        />
        <TextField
          value={formData.password}
          required
          type="password"
          onChange={event => onChange(event, 'password')}
          errorMessage={formError?.password?.join('')}
          label={intl.formatMessage({ id: 'form.fields.password' })}
        />
        <TextField
          value={formData.passwordConfirmation}
          required
          type="password"
          onChange={event => onChange(event, 'passwordConfirmation')}
          errorMessage={formError?.passwordConfirmation?.join('')}
          label={intl.formatMessage({
            id: 'form.fields.password-confirmation',
          })}
        />
      </Form>
    </>
  );
};
