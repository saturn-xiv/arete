import React from 'react';
import { useAccess, Access } from 'umi';

export default () => {
  const access = useAccess();
  return (
    <Access
      accessible={access.isSignIn}
      fallback={<div>Can not update foo.</div>}
    >
      <div>
        <h1>logs</h1>
      </div>
    </Access>
  );
};
