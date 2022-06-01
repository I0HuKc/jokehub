import axios, { AxiosError } from "axios";

import React from "react";
import Link from "next/link";
import Layout from "../components/layout";
import { Auth } from "../types/account";
import { HubError } from "../types/base";

const LoginPage = () => {
  interface LoginForm {
    username?: string;
    password?: string;
  }

  interface ErrBlock {
    display?: boolean;
    err?: HubError;
  }

  const [state, setState] = React.useState<LoginForm>();
  const [err_block, togErrBlock] = React.useState<ErrBlock>();

  const handleChange = (event: React.FormEvent<HTMLInputElement>) => {
    setState({
      ...state,
      [event.currentTarget.name]: event.currentTarget.value,
    });
  };

  const submit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    let result = await axios
      .post<Auth>("/api/v1/login", state)
      .then(async (resp) => {
        console.log(resp.data.access_token);
      })
      .catch((err: AxiosError<HubError>) => {
        togErrBlock({ display: true, err: err.response?.data });
      });
  };

  return (
    <>
      <Layout title="Login" className="h-screen">
        <div className="col-span-3 bg-amber-50 flex flex-col hidden xl:block">
          <div className="grid grid-cols-10 mt-14 mb-8">
            <div className="relative col-start-2 col-span-8">
              <div className="flex flex-col gap-y-6">
                <div className="w-full">
                  <Link href="/">
                    <img
                      src="img/jokehub_logo.svg"
                      alt=""
                      className="h-10 select-none"
                    />
                  </Link>
                </div>

                <h3 className="text-amber-900 font-semibold text-4xl opacity-60 select-none">
                  Discover the world's top Designers & Creatives.
                </h3>
              </div>

              <div className="flex flex-col items-center justify-center mt-32">
                <img src="img/beer.svg" alt="" className="w-full select-none" />
              </div>
            </div>

            <div className="absolute bottom-0 mb-6 mx-10">
              <p className="text-sm text-amber-900">
                Joke by <button className="underline">shavedkiwi</button>
              </p>
            </div>
          </div>
        </div>

        <div className="col-span-12 xl:col-span-9 grid grid-cols-12 xl:grid-cols-11">
          <div className="col-start-3 md:col-start-5 col-span-8 md:col-span-3 flex flex-col justify-center mb-10">
            <div className="flex flex-col items-center my-10 w-full">
              <div className="flex flex-col gap-y-5 w-full">
                <div className="flex flex-col gap-y-2.5 items-start">
                  <h2 className="font-semibold text-2xl">Login</h2>
                  <p className="text-sm text-gray-500 leading-5">
                    Hey, enter your details to get sing in to your account
                  </p>
                </div>

                <form
                  className="flex flex-col items-center gap-y-4 w-full"
                  method="post"
                  onSubmit={submit}
                >
                  <div className="flex flex-col items-center gap-y-2.5 w-full">
                    <div className="relative w-full">
                      <input
                        id="input_username"
                        name="username"
                        type="text"
                        autoComplete="off"
                        required
                        onChange={handleChange}
                        className="appearance-none rounded-none relative block w-full px-3.5 py-3 border placeholder-gray-400 text-gray-900 rounded-md focus:outline-none focus:ring-amber-400 focus:border-amber-400 focus:z-10 sm:text-sm transition duration-300 ease-in-out"
                        placeholder="Username"
                      />
                    </div>

                    <div className="flex flex-col gap-y-1 relative w-full">
                      <input
                        id="input_password"
                        name="password"
                        type="password"
                        autoComplete="off"
                        required
                        onChange={handleChange}
                        maxLength={20}
                        minLength={8}
                        className="appearance-none rounded-none relative block w-full px-3.5 py-3 border placeholder-gray-400 text-gray-900 rounded-md focus:outline-none focus:ring-amber-400 focus:border-amber-400 focus:z-10 sm:text-sm transition duration-300 ease-in-out"
                        placeholder="Password"
                      />
                      <div className="flex flex-row justify-end w-full">
                        <Link href="/forgot">
                          <p className="text-xs text-blue-600 select-none cursor-pointer">
                            Forgot password?
                          </p>
                        </Link>
                      </div>
                    </div>
                  </div>

                  {err_block?.display ? (
                    <div className="bg-red-50 border border-red-100 rounded-md w-full px-5 py-3">
                      <p className="text-sm font-medium text-red-900"> {err_block.err?.error}</p>
                      <p className="text-sm text-red-700"> {err_block.err?.details[0]}</p>
                    </div>
                  ) : null}

                  <div className="flex flex-col items-center gap-y-2 w-full">
                    <button
                      type="submit"
                      className="bg-amber-400 w-full rounded-md py-3 text-sm font-medium text-gray-700 select-none hover:bg-opacity-80 transition duration-300 ease-in-out"
                    >
                      Login
                    </button>
                    <Link href="/registration">
                      <button
                        type="submit"
                        className="border-2 border-amber-400 w-full rounded-md py-3 text-sm font-medium text-gray-700 select-none hover:opacity-70 transition duration-300 ease-in-out"
                      >
                        Registration
                      </button>
                    </Link>
                  </div>
                </form>
              </div>
            </div>
          </div>
        </div>
      </Layout>
    </>
  );
};

export default LoginPage;
