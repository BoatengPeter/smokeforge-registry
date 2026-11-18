import { zodResolver } from "@hookform/resolvers/zod";
import { useEffect } from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { z } from "zod";

import { Button } from "@/components/ui/button";
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import type { LibraryItem, LibraryStatus } from "@shared/ipc/library";
import { DesktopApiError } from "@renderer/lib/desktop-api";

import { useCreateLibraryItem, useUpdateLibraryItem } from "./library-hooks";

const formSchema = z.object({
  title: z.string().trim().min(1, "A title is required.").max(160),
  description: z.string().trim().max(5000),
  status: z.enum(["draft", "active", "archived"]),
  tags: z.string().max(400),
});

type FormValues = z.infer<typeof formSchema>;

function valuesFor(item: LibraryItem | null): FormValues {
  return {
    title: item?.title ?? "",
    description: item?.description ?? "",
    status: item?.status ?? "draft",
    tags: item?.tags.join(", ") ?? "",
  };
}

export function LibraryForm({
  item,
  onDone,
}: {
  item: LibraryItem | null;
  onDone: (item: LibraryItem) => void;
}) {
  const { t } = useTranslation();
  const create = useCreateLibraryItem();
  const update = useUpdateLibraryItem();
  const form = useForm<FormValues>({
    resolver: zodResolver(formSchema),
    defaultValues: valuesFor(item),
  });
  const pending = create.isPending || update.isPending;
  const error = create.error ?? update.error;

  useEffect(() => {
    form.reset(valuesFor(item));
  }, [form, item]);

  const submit = form.handleSubmit(async (values) => {
    const tags = values.tags
      .split(",")
      .map((tag) => tag.trim())
      .filter(Boolean)
      .slice(0, 12);
    const input = { ...values, tags };

    const saved = item
      ? await update.mutateAsync({ id: item.id, ...input })
      : await create.mutateAsync(input);

    if (saved) onDone(saved);
  });

  return (
    <Form {...form}>
      <form className="grid gap-4" onSubmit={submit}>
        <FormField
          control={form.control}
          name="title"
          render={({ field }) => (
            <FormItem>
              <FormLabel>{t("library.titleField")}</FormLabel>
              <FormControl>
                <Input autoFocus placeholder={t("library.titlePlaceholder")} {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="description"
          render={({ field }) => (
            <FormItem>
              <FormLabel>{t("library.descriptionField")}</FormLabel>
              <FormControl>
                <Textarea placeholder={t("library.descriptionPlaceholder")} {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="status"
          render={({ field }) => (
            <FormItem>
              <FormLabel>{t("library.statusField")}</FormLabel>
              <Select onValueChange={field.onChange} value={field.value}>
                <FormControl>
                  <SelectTrigger>
                    <SelectValue />
                  </SelectTrigger>
                </FormControl>
                <SelectContent>
                  {(["draft", "active", "archived"] as LibraryStatus[]).map((status) => (
                    <SelectItem key={status} value={status}>
                      {t(`library.${status}`)}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
              <FormMessage />
            </FormItem>
          )}
        />
        <FormField
          control={form.control}
          name="tags"
          render={({ field }) => (
            <FormItem>
              <FormLabel>{t("library.tagsField")}</FormLabel>
              <FormControl>
                <Input placeholder={t("library.tagsPlaceholder")} {...field} />
              </FormControl>
              <FormMessage />
            </FormItem>
          )}
        />
        {error ? (
          <p className="text-sm text-destructive" role="alert">
            {error instanceof DesktopApiError ? error.message : t("common.error")}
          </p>
        ) : null}
        <Button disabled={pending} type="submit">
          {pending ? t("common.saving") : t("common.save")}
        </Button>
      </form>
    </Form>
  );
}
