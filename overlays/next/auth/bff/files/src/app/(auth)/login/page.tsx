import { LoginForm } from "@/components/auth/login-form";

export default function LoginPage() {
  return (
    <main className="mx-auto flex min-h-dvh max-w-md flex-col justify-center px-6">
      <h1 className="text-2xl font-semibold">Sign in</h1>
      <p className="mt-2 text-sm text-muted-foreground">
        Your session is stored in HttpOnly cookies by the Next.js BFF.
      </p>
      <div className="mt-8">
        <LoginForm />
      </div>
    </main>
  );
}
