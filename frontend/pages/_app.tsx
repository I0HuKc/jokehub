import "../styles/globals.css";
import { NextUIProvider, createTheme } from "@nextui-org/react";
import type { AppProps } from "next/app";

function MyApp({ Component, pageProps }: AppProps) {
  const myDarkTheme = createTheme({
    type: "light",
    theme: {
      colors: {},
      space: {},
      fonts: {},
    },
  });

  return (
    <NextUIProvider theme={myDarkTheme}>
      <Component {...pageProps} />
    </NextUIProvider>
  );
}

export default MyApp;
