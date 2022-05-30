import Head from "next/head";
import Layout from "../components/layout";
import Container from "../components/container";
import Intro from "../components/intro";
import Link from "next/link";

type Props = {
  test: [];
};

const Index = ({ test }: Props) => {
  return (
    <>
      <Layout title="Jokehub">
        <Head>
          <title>Jokehub</title>
        </Head>
        <Link href="/login">
          <a>Login</a>
        </Link>
      </Layout>
    </>
  );
};

export default Index;

export const getStaticProps = async () => {
  return {
    props: {},
  };
};
