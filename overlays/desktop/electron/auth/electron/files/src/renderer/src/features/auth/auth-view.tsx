import { useState, type FormEvent } from "react";

import { useAuthSession, useLogin, useLogout, useRegister } from "./auth-hooks";

export function AuthView() {
  const session = useAuthSession();
  const login = useLogin();
  const register = useRegister();
  const logout = useLogout();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [mode, setMode] = useState<"login" | "register">("login");

  if (session.isLoading) {
    return <p className="text-sm text-slate-400">Checking session...</p>;
  }

  if (session.data) {
    return (
      <section className="rounded-xl border border-white/10 bg-white/5 p-6" aria-label="Account">
        <p className="text-xs uppercase tracking-wide text-slate-400">Signed in</p>
        <p className="mt-2 text-lg font-medium text-white">{session.data.email}</p>
        <button
          type="button"
          className="mt-4 rounded-md border border-white/15 px-3 py-1.5 text-sm text-white"
          disabled={logout.isPending}
          onClick={() => logout.mutate()}
        >
          {logout.isPending ? "Signing out..." : "Sign out"}
        </button>
      </section>
    );
  }

  function submit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const mutation = mode === "login" ? login : register;
    mutation.mutate({ email, password });
  }

  return (
    <section className="rounded-xl border border-white/10 bg-white/5 p-6" aria-label="Authentication">
      <h2 className="text-xl font-medium text-white">
        {mode === "login" ? "Sign in" : "Create account"}
      </h2>
      <form className="mt-4 space-y-3" onSubmit={submit}>
        <input
          className="w-full rounded-md border border-white/15 bg-slate-950 px-3 py-2 text-white"
          type="email"
          required
          placeholder="Email"
          value={email}
          onChange={(event) => setEmail(event.target.value)}
        />
        <input
          className="w-full rounded-md border border-white/15 bg-slate-950 px-3 py-2 text-white"
          type="password"
          required
          minLength={mode === "register" ? 12 : 1}
          placeholder="Password"
          value={password}
          onChange={(event) => setPassword(event.target.value)}
        />
        {login.error || register.error ? (
          <p role="alert" className="text-sm text-red-200">
            {(login.error ?? register.error)?.message}
          </p>
        ) : null}
        <button
          type="submit"
          className="w-full rounded-md bg-cyan-300 px-3 py-2 font-medium text-slate-950 disabled:opacity-50"
          disabled={login.isPending || register.isPending}
        >
          {mode === "login" ? "Sign in" : "Create account"}
        </button>
      </form>
      <button
        type="button"
        className="mt-3 text-sm text-cyan-200 underline"
        onClick={() => setMode(mode === "login" ? "register" : "login")}
      >
        {mode === "login" ? "Need an account?" : "Already have an account?"}
      </button>
    </section>
  );
}
