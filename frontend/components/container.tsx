import { ReactNode, FunctionComponent } from "react";

type Props = {
  children?: ReactNode;
};

const Container: FunctionComponent = ({ children }: Props) => {
  return (
    <>
      <div className="grid grid-cols-12 gap-5">{children}</div>
    </>
  );
};

export default Container;
