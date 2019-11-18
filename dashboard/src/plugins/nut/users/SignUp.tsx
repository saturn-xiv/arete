import React from "react";
import { FormattedMessage } from "react-intl";

import SharedLinks from "./SharedLinks";

const Widget: React.FC = () => {
  return (
    <div>
      <FormattedMessage id="nut.users.sign-up" />
      <br /> <SharedLinks />
    </div>
  );
};

export default Widget;
