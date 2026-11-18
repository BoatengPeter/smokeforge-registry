import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import { Button } from "@/components/ui/button";
import type { OAuthState } from "@shared/ipc/oauth";
import { DesktopApiError, invoke } from "@renderer/lib/desktop-api";

export function OAuthView() {
  const { t } = useTranslation();
  const [state, setState] = useState<OAuthState | null>(null);
  const [error, setError] = useState("");

  useEffect(() => {
    void invoke(window.desktopApi.oauth.status()).then(setState).catch(() => undefined);
    return window.desktopApi.oauth.onEvent(setState);
  }, []);

  const run = async (action: "login" | "logout") => {
    setError("");
    try {
      setState(await invoke(action === "login" ? window.desktopApi.oauth.login() : window.desktopApi.oauth.logout()));
    } catch (reason) {
      setError(reason instanceof DesktopApiError ? reason.message : t("common.error"));
    }
  };

  return (
    <div className="mt-4 grid gap-3">
      <p className="text-sm">
        {state?.status === "signed-in"
          ? "Signed in with the system browser."
          : state?.status === "pending"
            ? state.message
            : "Use your identity provider to sign in."}
      </p>
      {error ? <p className="text-sm text-destructive" role="alert">{error}</p> : null}
      {state?.status === "signed-in" ? (
        <Button className="w-fit" onClick={() => void run("logout")} variant="outline">Sign out</Button>
      ) : (
        <Button className="w-fit" disabled={state?.status === "pending"} onClick={() => void run("login")}>Sign in</Button>
      )}
    </div>
  );
}
