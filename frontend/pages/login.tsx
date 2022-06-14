import axios, { AxiosError } from "axios";

import React from "react";
import Link from "next/link";
import Layout from "../components/layout";
import { HubError } from "../types/base";
import { Loading } from "@nextui-org/react";
import { Auth } from "../types/account";

const LoginPage = () => {
  interface LoginForm {
    username?: string;
    password?: string;
  }

  interface ContentState {
    hack_time?: string;
    form_disable?: boolean;
    err?: string;
  }

  const [log_state, setState] = React.useState<LoginForm>();
  const [content_state, setContetnState] = React.useState<ContentState>();

  const handleChange = (event: React.FormEvent<HTMLInputElement>) => {
    setState({
      ...log_state,
      [event.currentTarget.name]: event.currentTarget.value,
    });
  };

  const submit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    let result = await axios
      .post<Auth>("/api/v1/login", log_state)
      .then(async (resp) => {
        console.log(resp.data.access_token);

        setContetnState({ err: undefined });
      })
      .catch((err: AxiosError<HubError>) => {
        setContetnState({ err: err.response?.data.error });
      });
  };

  return (
    <>
      <Layout title="Login" className="h-screen">
        <div className="col-start-5 col-span-4 flex flex-col gap-y-14 justify-center items-center mb-14">
          {/* Заголовок страницы */}
          <div className="flex flex-col items-center gap-y-3">
            <img
              src="img/beer_logo.png"
              className="h-16 select-none"
              alt="JokeHub"
            />

            <h2 className="font-sans font-extrabold text-5xl text-stone-800">
              Welcome back
            </h2>

            <p className="font-sans text-base text-stone-400">
              Enter your details to log in
            </p>
          </div>

          {/* Форма регистрации */}
          <form
            className="flex flex-col gap-y-8 w-full"
            method="post"
            onSubmit={submit}
          >
            <div className="flex flex-col gap-y-4 w-full">
              <input
                id="input_username"
                name="username"
                type="text"
                autoComplete="off"
                required
                placeholder="Username"
                onChange={handleChange}
                disabled={
                  content_state?.form_disable != undefined
                    ? content_state.form_disable
                    : false
                }
                className="appearance-none rounded-none levfk w-full text-sm px-4 py-5 border border-stone-300 placeholder-stone-400 font-sans text-stone-800 rounded-lg focus:outline-none focus:ring-perfo focus:border-amber-500 focus:z-10 disabled:bg-stone-100 disabled:text-opacity-50 disabled:cursor-not-allowed transition duration-300 ease-in-out select-none"
              />

              <input
                id="input_password"
                name="password"
                type="password"
                autoComplete="off"
                required
                placeholder="Password"
                minLength={8}
                maxLength={20}
                onChange={handleChange}
                disabled={
                  content_state?.form_disable != undefined
                    ? content_state.form_disable
                    : false
                }
                className="appearance-none rounded-none w-full text-sm px-4 py-5 border border-stone-300 placeholder-stone-400 font-sans text-stone-800 rounded-lg focus:outline-none focus:ring-perfo focus:border-amber-500 focus:z-10 disabled:bg-stone-100 disabled:text-opacity-50 disabled:cursor-not-allowed transition duration-300 ease-in-out select-none"
              />
            </div>

            <div className="flex flex-col items-center gap-y-2.5">
              <button
                type="submit"
                disabled={
                  content_state?.form_disable != undefined
                    ? content_state.form_disable
                    : false
                }
                className="bg-yellow-400 py-5 w-full rounded-lg select-none hover:bg-amber-400 disabled:bg-opacity-20 disabled:cursor-not-allowed transition duration-300 ease-in-out"
              >
                {content_state?.form_disable != true ||
                content_state?.form_disable == undefined ? (
                  <span className="font-sans font-medium text-[#280700]">
                    Log in
                  </span>
                ) : (
                  <Loading type="points" size="md" color="warning" />
                )}
              </button>
            </div>

            {content_state?.err != undefined ? (
              <div className="bg-red-100 rounded-md border border-red-300 p-4 w-full">
                <p className="font-sans text-sm text-red-700">
                  {content_state?.err}
                </p>
              </div>
            ) : null}
          </form>
        </div>

        <div className="absolute bottom-0 w-full flex flex-row gap-x-2 justify-center items-center bg-stone-100 h-20 select-none z-0">
          <p className="font-sans text-sm text-stone-400">
            Already have an account?
          </p>
          <Link href="/registration">
            <span className="font-sans text-sm text-stone-400 font-medium hover:text-[#280700] cursor-pointer transition duration-300 ease-in-out">
              Registration
            </span>
          </Link>
        </div>
      </Layout>
    </>
  );
};

export default LoginPage;
