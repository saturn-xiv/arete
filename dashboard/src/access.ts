export default function(initialState: any) {
  console.log('TODO, parse token', initialState);

  return {
    canReadFoo: true,
    isSignIn: false,
    // canUpdateFoo: role === 'admin',
    // canDeleteFoo: foo => {
    //   return foo.ownerId === userId;
    // },
  };
}
