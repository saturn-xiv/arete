import React from "react";

import SharedLinks from "./SharedLinks";

const Widget: React.FC = () => {
  return (
    <SharedLinks title={{ id: "nut.users.sign-in.title" }}>
      Sign In
      <br />
    </SharedLinks>
  );
};

export default Widget;
