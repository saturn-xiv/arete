import React from "react";

import SharedLinks from "./SharedLinks";

const Widget: React.FC = () => {
  return (
    <SharedLinks title={{ id: "nut.users.sign-up.title" }}>
      sign up
      <br />
    </SharedLinks>
  );
};

export default Widget;
