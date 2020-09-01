import React, { useEffect } from "react";
import { FormattedMessage } from "react-intl";
import { useSelector, useDispatch } from "react-redux";
import { Stack, IStackItemStyles } from "office-ui-fabric-react";

import { get as httpGet } from "../utils/request";
import { refresh, IState as IApplicationState, ISite } from "../actions";

interface IProps {
  refresh: typeof refresh;
  siteInfo: ISite;
}

const styles: IStackItemStyles = {
  root: { paddingTop: 20 },
};

const Component = () => {
  const siteInfo = useSelector((state: IApplicationState) => state.siteInfo);
  const dispatch = useDispatch();
  useEffect(() => {
    httpGet("/about").then((rst) => {
      dispatch(
        refresh({
          version: rst.version,
          title: rst.name,
          uptime: rst.uptime,
        })
      );
    });
  }, [dispatch]);

  return (
    <Stack.Item styles={styles} align="center">
      <span>
        &copy;
        <FormattedMessage id="site.copyright" />
      </span>
      &nbsp;
      {siteInfo.version}
      &nbsp;
      {siteInfo.uptime}
    </Stack.Item>
  );
};

export default Component;
