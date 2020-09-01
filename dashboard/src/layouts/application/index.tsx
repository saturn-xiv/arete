import React, { ReactNode } from "react";
import { Stack, IStackStyles } from "office-ui-fabric-react";

import Title from "../Title";
import Footer from "../Footer";

interface IProps {
  children: ReactNode;
  title: string;
}

const styles: IStackStyles = {
  root: { marginTop: 40 },
};

const Component = ({ children, title }: IProps) => {
  return (
    <Stack styles={styles}>
      {children}
      <Footer />
      <Title value={title} />
    </Stack>
  );
};
export default Component;
