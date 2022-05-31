import React, { ReactNode } from "react";
import Head from "next/head";

type Props = {
  children?: ReactNode;
  title?: string;
  className?: string;
};

const Layout = ({ children, title = "Jokehub", className = "" }: Props) => {
  return (
    <>
      <Head>
        <title>{title}</title>
        <meta charSet="utf-8" />
        <meta name="viewport" content="initial-scale=1.0, width=device-width" />
      </Head>

      <div className={"grid grid-cols-12 " + className}>{children}</div>
    </>
  );
};

export default Layout;
