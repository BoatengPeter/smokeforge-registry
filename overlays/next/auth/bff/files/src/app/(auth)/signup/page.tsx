import { SignupForm } from "@/components/auth/signup-form";

export default function SignupPage() {
  return (
    <main className="mx-auto flex min-h-dvh max-w-md flex-col justify-center px-6">
      <h1 className="text-2xl font-semibold">Create account</h1>
      <p className="mt-2 text-sm text-muted-foreground">
        Accounts are created by the API; the BFF keeps tokens out of browser JavaScript.
      </p>
      <div className="mt-8">
        <SignupForm />
      </div>
    </main>
  );
}
