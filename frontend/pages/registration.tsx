import axios, { AxiosError } from "axios";

import React from "react";
import Link from "next/link";
import Layout from "../components/layout";

const RegistrationPage = () => {
  interface LoginForm {
    username?: string;
    password?: string;
  }

  // interface ErrBlock {
  //   display?: boolean;
  //   err?: HubError;
  // }

  // const [state, setState] = React.useState<LoginForm>();
  // const [err_block, togErrBlock] = React.useState<ErrBlock>();

  // const handleChange = (event: React.FormEvent<HTMLInputElement>) => {
  //   setState({
  //     ...state,
  //     [event.currentTarget.name]: event.currentTarget.value,
  //   });
  // };

  // const submit = async (event: React.FormEvent<HTMLFormElement>) => {
  //   event.preventDefault();

  //   let result = await axios
  //     .post<Auth>("/api/v1/login", state)
  //     .then(async (resp) => {
  //       console.log(resp.data.access_token);
  //     })
  //     .catch((err: AxiosError<HubError>) => {
  //       togErrBlock({ display: true, err: err.response?.data });
  //     });
  // };

  return (
    <>
      <Layout title="Registration" className="h-screen">
        <div className="col-start-5 col-span-4 flex flex-col gap-y-14 justify-center items-center">
          {/* Заголовок страницы */}
          <div className="flex flex-col items-center gap-y-3">
            <h2 className="font-extrabold text-5xl text-stone-800">
              Create an account
            </h2>

            <p className="text-base text-stone-400">
              Enter your details to proceed further
            </p>
          </div>

          {/* Форма регистрации */}
          <form method="post" className="flex flex-col gap-y-8 w-full">
            <div className="flex flex-col gap-y-4 w-full">
              <input
                id="input_username"
                name="username"
                type="text"
                autoComplete="off"
                required
                placeholder="Username"
                className="appearance-none rounded-noneZ levfk w-full text-sm px-4 py-5 border border-stone-300 placeholder-stone-400 text-stone-800 rounded-lg focus:outline-none focus:ring-perfo focus:border-perfo focus:z-10 transition duration-300 ease-in-out"
              />

              <input
                id="input_password"
                name="password"
                type="password"
                autoComplete="off"
                required
                placeholder="Password"
                className="appearance-none rounded-none w-full text-sm px-4 py-5 border border-stone-300 placeholder-stone-400 text-stone-800 rounded-lg focus:outline-none focus:ring-perfo focus:border-perfo focus:z-10 transition duration-300 ease-in-out"
              />

              <input
                id="input_repeat_password"
                name="repeat_password"
                type="password"
                autoComplete="off"
                required
                placeholder="Repeat password"
                className="appearance-none rounded-none w-full text-sm px-4 py-5 border border-stone-300 placeholder-stone-400 text-stone-800 rounded-lg focus:outline-none focus:ring-perfo focus:border-perfo focus:z-10 transition duration-300 ease-in-out"
              />
            </div>

            <button className="bg-perfo py-5 w-full font-medium text-white rounded-lg">Create</button>
          </form>
        </div>
      </Layout>
    </>
  );
};

export default RegistrationPage;
