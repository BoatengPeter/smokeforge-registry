import { useTranslation } from "react-i18next";
import { Link } from "react-router";

import { Empty, EmptyDescription, EmptyHeader, EmptyTitle } from "@/components/ui/empty";

export function NotFoundPage() {
  const { t } = useTranslation();

  return (
    <div className="grid h-full place-items-center p-6">
      <Empty className="border">
        <EmptyHeader>
          <EmptyTitle>{t("notFound.title")}</EmptyTitle>
          <EmptyDescription>{t("notFound.body")}</EmptyDescription>
          <Link className="text-sm text-primary underline underline-offset-4" to="/">{t("notFound.home")}</Link>
        </EmptyHeader>
      </Empty>
    </div>
  );
}
