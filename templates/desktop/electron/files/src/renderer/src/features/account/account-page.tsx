import { useTranslation } from "react-i18next";

// smokeforge:anchor auth-imports
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";

export function AccountPage() {
  const { t } = useTranslation();

  return (
    <div className="mx-auto grid w-full max-w-3xl gap-5 p-6">
      <Card>
        <CardHeader>
          <CardTitle>{t("account.title")}</CardTitle>
          <CardDescription>{t("account.subtitle")}</CardDescription>
        </CardHeader>
        <CardContent>
          <p className="text-sm text-muted-foreground">{t("account.disabledBody")}</p>
          {/* smokeforge:anchor auth-view */}
        </CardContent>
      </Card>
    </div>
  );
}
