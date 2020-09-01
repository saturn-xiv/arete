import React from "react";
import { useSelector } from "react-redux";
import { Helmet } from "react-helmet";

import { IState as IApplicationState } from "../actions";

interface IProps {
  value: string;
}

const Component = ({ value }: IProps) => {
  const siteInfo = useSelector((state: IApplicationState) => state.siteInfo);
  const title = `${value}|${siteInfo.title || ""}|`;

  return (
    <Helmet>
      <title>{title}</title>
    </Helmet>
  );
};

export default Component;
