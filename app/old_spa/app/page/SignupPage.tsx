import { useFetcher } from "react-router";
import type { Route } from "../+types/root";
import "./SignupPage.css"
import { simpleFormGenerator } from "../component/SimpleFormGenerator";

export async function clientAction({
  params,
  request
}: Route.ActionArgs) {
  const formData = await request.formData()
  formData.forEach((item, key) => console.log(`key: ${key}, value: ${item}`))
  console.log("form data: ", formData)
  console.log("params: ", params)
}

const signupModel = {
  username: "",
  email: "",
  password: "",
}

export default function SignupPage() {

  const fetcher = useFetcher()

  const internalSignup = (
    <fetcher.Form
      method="POST"
    >
      {simpleFormGenerator(signupModel, {
        email: {
          type: "email"
        },
        password: {
          type: "password"
        }
      })}


      <p>
        <button
          type="submit"
        >
          Sign Up
        </button>
      </p>
    </fetcher.Form>
  )

  const externalSignup = (
    <>
      <button>
        Gmail
      </button>

      <button>
        Github
      </button>

      <button>
        Microsoft
      </button>
    </>
  );

  return (
    <>
      <h2>🔑 Signup Page</h2>

      <div
        className="signup-container"
      >
        <div>
          {internalSignup}
        </div>

        <div
          className="external-signup"
        >
          {externalSignup}
        </div>
      </div>

    </>
  )
}
