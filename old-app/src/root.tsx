import {
  Meta,
  Links,
  Outlet,
  Scripts,
  ScrollRestoration,
  isRouteErrorResponse,
} from "react-router";
import type { Route } from "./+types/root";
import asmLogo from './svg/ams-icon.svg'
import { ChakraProvider, createSystem, defaultConfig } from "@chakra-ui/react";
import { Toaster } from "./utility";
import { IS_INSIDE_TAURI } from "./constants";
import { QueryClientProvider } from "@tanstack/react-query";
import { queryClient } from "./commands";
import "react-datepicker/dist/react-datepicker.css"
import "root.css"

const system = createSystem(defaultConfig);

export default function Root() {
  return (
    <QueryClientProvider client={queryClient}>
      <ChakraProvider value={system}>
        <Toaster />
        <Outlet />
      </ChakraProvider>
    </QueryClientProvider>
  )
}

// The Layout component is a special export for the root route.
// It acts as your document's "app shell" for all route components, HydrateFallback, and ErrorBoundary
// For more information, see https://reactrouter.com/explanation/special-files#layout-export
export function Layout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <link rel="icon" href={asmLogo} type="image/svg+xml" />
        <title>AMS - Ampas Management System</title>
        <Meta />
        <Links />
      </head>
      <body>
        {children}
        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
}

// The top most error boundary for the app, rendered when your app throws an error
// For more information, see https://reactrouter.com/start/framework/route-module#errorboundary
export function ErrorBoundary({ error }: Route.ErrorBoundaryProps) {
  let message = "Oops!";
  let details = "An unexpected error occurred. Please Report to the Developer";
  let stack: string | undefined;

  if (isRouteErrorResponse(error)) {
    message = error.status === 404 ? "404" : "Error";
    details =
      error.status === 404
        ? "The requested page could not be found."
        : error.statusText || details;
  } else if (
    // import.meta.env.error && 
    error instanceof Error) {
    details = error.message;
    stack = error.stack;
  }

  return (
    <main id="error-page">
      <h1>❌{message}</h1>
      <p>{IS_INSIDE_TAURI ? "INSIDE_TAURI" : "INSIDE_VITE"}</p>
      <p>{details}</p>
      {stack && (
        <pre>
          <code>{stack}</code>
        </pre>
      )}
    </main>
  );
}
