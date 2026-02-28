import { useEffect, useState } from "react";

export function useCurrentPath() {

  const [path, setPath] = useState(window.location.pathname)

  useEffect(() => {
    setPath(window.location.pathname)
  }, [window.location])

  return {
    path,
    pathWithoutSlash: path.split('/')
  }
}
