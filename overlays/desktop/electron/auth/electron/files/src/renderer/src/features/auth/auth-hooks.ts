import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import type { AuthUser, LoginInput, RegisterInput } from "@shared/ipc/auth";
import { invoke } from "@renderer/lib/desktop-api";
import { queryKeys } from "@renderer/lib/query/keys";

export function useAuthSession() {
  return useQuery<AuthUser | null>({
    queryKey: queryKeys.auth.me,
    queryFn: () => invoke(window.desktopApi.auth.me()),
    retry: false,
  });
}

export function useLogin() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (input: LoginInput) => invoke(window.desktopApi.auth.login(input)),
    onSuccess: (user) => queryClient.setQueryData(queryKeys.auth.me, user),
  });
}

export function useRegister() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (input: RegisterInput) => invoke(window.desktopApi.auth.register(input)),
    onSuccess: (user) => queryClient.setQueryData(queryKeys.auth.me, user),
  });
}

export function useLogout() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: () => invoke(window.desktopApi.auth.logout()),
    onSuccess: () => {
      queryClient.removeQueries({ queryKey: queryKeys.auth.all });
    },
  });
}
