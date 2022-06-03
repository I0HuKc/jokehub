import axios, { AxiosError } from "axios";
import React from "react";
import { Loading } from "@nextui-org/react";
import { HubError } from "../types/base";
import { Strength } from "../types/account";
import { GoogleReCaptchaProvider } from "react-google-recaptcha-v3";
import Layout from "../components/layout";
import IconLightBlub from "../icons/light_bulb";

const RegistrationPage = () => {
  interface RegForm {
    username?: string;
    password?: string;
    repeat_password?: string;
  }

  interface ContentState {
    hack_time?: string;
    form_disable?: boolean;
  }

  // interface ErrBlock {
  //   display?: boolean;
  //   err?: HubError;
  // }

  const [reg_state, setRegState] = React.useState<RegForm>();
  const [content_state, setContetnState] = React.useState<ContentState>();
  // const [err_block, togErrBlock] = React.useState<ErrBlock>();

  const handleChange = (event: React.FormEvent<HTMLInputElement>) => {
    setRegState({
      ...reg_state,
      [event.currentTarget.name]: event.currentTarget.value,
    });

    if (
      event.currentTarget.name == "password" &&
      reg_state?.password != undefined &&
      reg_state?.password.length > 0
    ) {
      password_strength();
    }
  };

  const submit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    console.log(process.env.RECAPTCHA_SECRET);

    if (reg_state?.password == reg_state?.repeat_password) {
      setContetnState({ form_disable: true });

      console.log("sd");

      // let result = await axios
      //   .post("/api/v1/registration", reg_state)
      //   .then(async (resp) => {
      //     setContetnState({ form_disable: false });
      //     console.log(resp.data.access_token);
      //   })
      //   .catch((err: AxiosError<HubError>) => {
      //     // togErrBlock({ display: true, err: err.response?.data });
      //   });

      return;
    }
  };

  const password_strength = async () => {
    let result = await axios
      .post<Strength>("/api/v1/registration/password-strength", reg_state)
      .then(async (resp) => {
        setContetnState({ hack_time: resp.data.throttling_10_second });
      })
      .catch((err: AxiosError<HubError>) => {
        console.log(err);
        // togErrBlock({ display: true, err: err.response?.data });
      });
  };

  return (
    <>
      <GoogleReCaptchaProvider reCaptchaKey={process.env.RECAPTCHA_SECRET}>
        <Layout title="Registration" className="h-screen">
          <div className="col-start-5 col-span-4 flex flex-col gap-y-14 justify-center items-center">
            {/* Заголовок страницы */}
            <div className="flex flex-col items-center gap-y-3">
              <img
                src="img/beer_logo.png"
                className="h-16 select-none"
                alt="JokeHub"
              />

              <h2 className="font-sans font-extrabold text-5xl text-stone-800">
                Create an account
              </h2>

              <p className="font-sans text-base text-stone-400">
                Enter your details to proceed further
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

                <div className="flex flex-col gap-y-2 relative w-full">
                  <input
                    id="input_password"
                    name="password"
                    type="password"
                    autoComplete="off"
                    required
                    placeholder="Password"
                    onChange={handleChange}
                    disabled={
                      content_state?.form_disable != undefined
                        ? content_state.form_disable
                        : false
                    }
                    className="appearance-none rounded-none w-full text-sm px-4 py-5 border border-stone-300 placeholder-stone-400 font-sans text-stone-800 rounded-lg focus:outline-none focus:ring-perfo focus:border-amber-500 focus:z-10 disabled:bg-stone-100 disabled:text-opacity-50 disabled:cursor-not-allowed transition duration-300 ease-in-out select-none"
                  />

                  {content_state?.hack_time != undefined &&
                  reg_state?.password != undefined &&
                  reg_state?.password.length != 0 ? (
                    <div className="flex flex-row items-center gap-x-2 select-none">
                      <IconLightBlub className="h-4 stroke-stone-500" />
                      <p className="text-xs text-stone-400 font-sans">
                        Time to hack:{" "}
                        <span className="font-medium">
                          {content_state?.hack_time}
                        </span>
                      </p>
                    </div>
                  ) : null}
                </div>

                <input
                  id="input_repeat_password"
                  name="repeat_password"
                  type="password"
                  autoComplete="off"
                  required
                  placeholder="Repeat password"
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
                      Create
                    </span>
                  ) : (
                    <Loading type="points" size="md" color="warning" />
                  )}
                </button>

                <p className="font-sans text-xs text-stone-400">
                  By signing up you agree to our API Terms of Service and
                  Privacy Policy
                </p>
              </div>
            </form>
          </div>
        </Layout>
      </GoogleReCaptchaProvider>
    </>
  );
};

export default RegistrationPage;
