import { useFetcher } from "react-router";
import type { Route } from "../+types/root";
import { simpleFormGenerator } from "../component/SimpleFormGenerator";
import "./SigninPage.css"
import { externalAuthenticationUrlBuilder, ExternalLoginProvider } from "../api/constant";

export async function clientAction({
  params,
  request
}: Route.ActionArgs) {
}

const signinModel = {
  email: "",
}

export default function SignupPage() {

  const fetcher = useFetcher()

  const handleLogin = async (provider: ExternalLoginProvider) => {
    window.location.href = externalAuthenticationUrlBuilder(provider)
  }


  const internalSignin = (
    <fetcher.Form
      method="POST"
    >
      {simpleFormGenerator(signinModel, {
        email: {
          type: "email"
        },
      })}


      <p>
        <button
          type="submit"
        >
          Sign In
        </button>
      </p>
    </fetcher.Form>
  )

  const externalSignin = (
    <>
      <button
        onClick={() => handleLogin(ExternalLoginProvider.Google)}
      >
        Gmail
      </button>

      <button
        onClick={() => handleLogin(ExternalLoginProvider.Github)}
      >
        Github
      </button>

      <button>
        ✏️ Microsoft (TODO)
      </button>
    </>
  );

  return (
    <>
      <h2>🔏 Signin Page</h2>

      <div
        className="signin-container"
      >
        {/* <div> */}
        {/*   {internalSignin} */}
        {/* </div> */}

        <div
          className="external-signin"
        >
          {externalSignin}
        </div>
      </div>

    </>
  )
}
