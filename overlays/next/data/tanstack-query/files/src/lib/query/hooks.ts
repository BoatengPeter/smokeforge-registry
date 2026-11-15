"use client";

import {
  useMutation,
  useQuery,
  useQueryClient,
  type QueryKey,
  type UseMutationOptions,
  type UseQueryOptions,
} from "@tanstack/react-query";
import { apiFetch } from "@/lib/api/client";
import { AppError } from "@/lib/api/app-error";
import { queryKeys } from "@/lib/query/keys";

export type AuthUser = {
  id: string;
  email: string;
  role: string;
  created_at: string;
};

export type LoginInput = {
  email: string;
  password: string;
};

export type RegisterInput = LoginInput;

export function useApiQuery<T>(
  key: QueryKey,
  path: string,
  options?: Omit<UseQueryOptions<T, AppError>, "queryKey" | "queryFn">,
) {
  return useQuery<T, AppError>({
    queryKey: key,
    queryFn: () => apiFetch<T>(path),
    ...options,
  });
}

export function useApiMutation<TInput, TOutput>(
  path: string,
  method: "POST" | "PUT" | "PATCH" | "DELETE" = "POST",
  options?: Omit<
    UseMutationOptions<TOutput, AppError, TInput>,
    "mutationFn"
  >,
) {
  return useMutation<TOutput, AppError, TInput>({
    mutationFn: (input) =>
      apiFetch<TOutput>(path, {
        method,
        body: JSON.stringify(input),
      }),
    ...options,
  });
}

export function useAuthSession() {
  return useApiQuery<AuthUser>(queryKeys.auth.me, "/api/auth/me", {
    retry: false,
  });
}

export function useLogin() {
  const queryClient = useQueryClient();

  return useApiMutation<LoginInput, AuthUser>("/api/auth/login", "POST", {
    onSuccess: (user) => {
      queryClient.setQueryData(queryKeys.auth.me, user);
    },
  });
}

export function useRegister() {
  const queryClient = useQueryClient();

  return useApiMutation<RegisterInput, AuthUser>(
    "/api/auth/register",
    "POST",
    {
      onSuccess: (user) => {
        queryClient.setQueryData(queryKeys.auth.me, user);
      },
    },
  );
}

export function useLogout() {
  const queryClient = useQueryClient();

  return useApiMutation<Record<string, never>, void>(
    "/api/auth/logout",
    "POST",
    {
      onSuccess: () => {
        queryClient.removeQueries({ queryKey: queryKeys.auth.all });
      },
    },
  );
}
