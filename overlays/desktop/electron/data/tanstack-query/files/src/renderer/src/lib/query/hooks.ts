import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

import type { SystemInfo } from "@shared/ipc/system";
import { invoke } from "@renderer/lib/desktop-api";
import { queryKeys } from "./keys";

export function useSystemVersion() {
  return useQuery<SystemInfo>({
    queryKey: queryKeys.systemVersion,
    queryFn: () => invoke(window.desktopApi.system.getVersion()),
    retry: false,
  });
}

export function useSystemPing() {
  return useMutation({
    mutationFn: () => invoke(window.desktopApi.system.ping()),
  });
}

export function useRefreshSystemVersion() {
  const queryClient = useQueryClient();

  return () => queryClient.invalidateQueries({ queryKey: queryKeys.system });
}
