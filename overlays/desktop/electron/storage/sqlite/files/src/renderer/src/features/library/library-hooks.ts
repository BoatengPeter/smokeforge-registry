import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect } from "react";

import type {
  LibraryCreateInput,
  LibraryListInput,
  LibraryUpdateInput,
} from "@shared/ipc/library";
import { invoke } from "@renderer/lib/desktop-api";

const keys = {
  all: ["library"] as const,
  list: (input: LibraryListInput) => ["library", "list", input] as const,
  stats: ["library", "stats"] as const,
};

export function useLibrary(input: LibraryListInput) {
  const queryClient = useQueryClient();

  useEffect(() => {
    const unsubscribe = window.desktopApi.library.onChanged(() => {
      void queryClient.invalidateQueries({ queryKey: keys.all });
    });

    return unsubscribe;
  }, [queryClient]);

  return useQuery({
    queryKey: keys.list(input),
    queryFn: () => invoke(window.desktopApi.library.list(input)),
  });
}

export function useLibraryStats() {
  return useQuery({
    queryKey: keys.stats,
    queryFn: () => invoke(window.desktopApi.library.stats()),
  });
}

export function useCreateLibraryItem() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (input: LibraryCreateInput) => invoke(window.desktopApi.library.create(input)),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: keys.all }),
  });
}

export function useUpdateLibraryItem() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (input: LibraryUpdateInput) => invoke(window.desktopApi.library.update(input)),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: keys.all }),
  });
}

export function useDeleteLibraryItem() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => invoke(window.desktopApi.library.delete(id)),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: keys.all }),
  });
}

export function useRestoreLibraryItem() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (id: string) => invoke(window.desktopApi.library.restore(id)),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: keys.all }),
  });
}
