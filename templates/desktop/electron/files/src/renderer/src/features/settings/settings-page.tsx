import { useTranslation } from "react-i18next";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Switch } from "@/components/ui/switch";
import { SizeStepper } from "@renderer/components/size-stepper";
import { useWelcomeTour } from "@renderer/features/welcome/welcome-context";
import { useAppSettings } from "@renderer/lib/app-settings";
import { DesktopApiError, invoke } from "@renderer/lib/desktop-api";
import { useSystemVersion } from "@renderer/lib/query/hooks";
import { useUpdates } from "@renderer/features/updates/updates-context";

export function SettingsPage() {
  const { t } = useTranslation();
  const { settings, update } = useAppSettings();
  const { data: info } = useSystemVersion();
  const updates = useUpdates();
  const tour = useWelcomeTour();

  if (!settings) return <p className="p-6 text-sm text-muted-foreground">{t("common.loading")}</p>;

  return (
    <div className="mx-auto grid w-full max-w-3xl gap-5 p-6">
      <Card>
        <CardHeader>
          <CardTitle>{t("settings.general")}</CardTitle>
          <CardDescription>{t("settings.subtitle")}</CardDescription>
        </CardHeader>
        <CardContent className="grid gap-5">
          <div className="grid gap-2 sm:grid-cols-[160px_1fr] sm:items-center">
            <Label>{t("settings.theme")}</Label>
            <Select
              onValueChange={(value) => void update({ theme: value as "system" | "light" | "dark" })}
              value={settings.theme}
            >
              <SelectTrigger className="w-52"><SelectValue /></SelectTrigger>
              <SelectContent>
                <SelectItem value="system">{t("settings.themeSystem")}</SelectItem>
                <SelectItem value="light">{t("settings.themeLight")}</SelectItem>
                <SelectItem value="dark">{t("settings.themeDark")}</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="grid gap-2 sm:grid-cols-[160px_1fr] sm:items-center">
            <Label>{t("settings.language")}</Label>
            <Select onValueChange={(value) => void update({ language: value })} value={settings.language}>
              <SelectTrigger className="w-52"><SelectValue /></SelectTrigger>
              <SelectContent>
                <SelectItem value="en">{t("settings.english")}</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="grid gap-2 sm:grid-cols-[160px_1fr] sm:items-center">
            <div>
              <Label>{t("settings.size")}</Label>
              <p className="mt-1 text-xs text-muted-foreground">{t("settings.sizeDescription")}</p>
            </div>
            <div>
              <SizeStepper current={settings.zoom} onChange={(zoom) => void update({ zoom })} />
            </div>
          </div>
          <div className="flex items-center justify-between gap-4">
            <div>
              <Label htmlFor="launch-at-login">{t("settings.launch")}</Label>
              <p className="mt-1 text-xs text-muted-foreground">{t("settings.launchDescription")}</p>
            </div>
            <Switch
              checked={settings.launchAtLogin}
              id="launch-at-login"
              onCheckedChange={(checked) => void update({ launchAtLogin: checked })}
            />
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>{t("settings.help")}</CardTitle>
          <CardDescription>{t("settings.tourDescription")}</CardDescription>
        </CardHeader>
        <CardContent>
          <Button onClick={tour.show} variant="outline">
            {t("settings.tourOpen")}
          </Button>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>{t("settings.data")}</CardTitle>
          <CardDescription>{t("settings.dataDescription")}</CardDescription>
        </CardHeader>
        <CardContent className="grid gap-3">
          <div className="grid gap-1 text-xs">
            <span className="text-muted-foreground">{t("settings.dataPath")}</span>
            <code className="rounded-md bg-muted px-2 py-1.5">{info?.userDataPath ?? "—"}</code>
          </div>
          <div className="flex flex-wrap gap-2">
            <Button
              onClick={() => {
                void invoke(window.desktopApi.library.exportAll()).then((result) => {
                  if (result) toast.success(t("common.exported", { count: result.count }));
                }).catch((error: unknown) => toast.error(error instanceof DesktopApiError ? error.message : t("common.error")));
              }}
              variant="outline"
            >
              {t("settings.export")}
            </Button>
            <Button
              onClick={() => {
                void invoke(window.desktopApi.library.importAll()).then((result) => {
                  if (result) toast.success(t("common.imported", { count: result.imported }));
                }).catch((error: unknown) => toast.error(error instanceof DesktopApiError ? error.message : t("common.error")));
              }}
              variant="outline"
            >
              {t("settings.import")}
            </Button>
            <Button
              onClick={() => {
                void invoke(window.desktopApi.library.backup()).then((result) => {
                  if (result) toast.success(t("common.databaseBackedUp"));
                }).catch((error: unknown) => toast.error(error instanceof DesktopApiError ? error.message : t("common.error")));
              }}
              variant="outline"
            >
              {t("settings.backup")}
            </Button>
          </div>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>{t("settings.updates")}</CardTitle>
          <CardDescription>{t("settings.updatePackaged")}</CardDescription>
        </CardHeader>
        <CardContent className="grid gap-4">
          <div className="grid gap-2 sm:grid-cols-[160px_1fr] sm:items-center">
            <Label>{t("settings.channel")}</Label>
            <Select
              onValueChange={(value) => void update({ updateChannel: value as "stable" | "beta" })}
              value={settings.updateChannel}
            >
              <SelectTrigger className="w-52"><SelectValue /></SelectTrigger>
              <SelectContent>
                <SelectItem value="stable">{t("settings.stable")}</SelectItem>
                <SelectItem value="beta">{t("settings.beta")}</SelectItem>
              </SelectContent>
            </Select>
          </div>
          <div className="flex items-center justify-between gap-4">
            <Label htmlFor="auto-check-updates">{t("settings.checkAutomatically")}</Label>
            <Switch
              checked={settings.autoCheckUpdates}
              id="auto-check-updates"
              onCheckedChange={(checked) => void update({ autoCheckUpdates: checked })}
            />
          </div>
          <div className="flex flex-wrap items-center gap-2">
            <Button
              className="w-fit"
              disabled={updates.state?.status === "checking" || updates.state?.status === "downloading"}
              onClick={() => void updates.check()}
              variant="outline"
            >
              {t("settings.checkNow")}
            </Button>
            {updates.state?.status === "available" ? (
              <Button onClick={() => void updates.download()}>{t("settings.downloadUpdate")}</Button>
            ) : null}
            {updates.state?.status === "downloaded" ? (
              <Button onClick={() => void updates.install()}>{t("settings.restartUpdate")}</Button>
            ) : null}
          </div>
          <p className="text-xs text-muted-foreground">
            {updates.state?.message || t("settings.updateConfig")}
            {updates.state?.progress ? ` ${updates.state.progress}%` : ""}
          </p>
          {updates.state?.releaseNotes ? (
            <pre className="max-h-32 overflow-auto whitespace-pre-wrap rounded-md bg-muted p-2 text-xs">{updates.state.releaseNotes}</pre>
          ) : null}
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>{t("settings.advanced")}</CardTitle>
          <CardDescription>{t("settings.advancedDescription")}</CardDescription>
        </CardHeader>
        <CardContent className="grid gap-4">
          <div className="flex items-center justify-between gap-4">
            <div>
              <Label htmlFor="scheduled-maintenance">{t("common.dailyMaintenance")}</Label>
              <p className="mt-1 text-xs text-muted-foreground">{t("common.dailyMaintenanceDescription")}</p>
            </div>
            <Switch
              checked={settings.scheduledMaintenance}
              id="scheduled-maintenance"
              onCheckedChange={(checked) => {
                void invoke(window.desktopApi.library.setMaintenance(checked))
                  .then(() => update({ scheduledMaintenance: checked }))
                  .catch(() => toast.error(t("common.error")));
              }}
            />
          </div>
          <div className="grid gap-2 sm:grid-cols-[160px_1fr] sm:items-center">
            <Label>{t("settings.logLevel")}</Label>
            <Select
              onValueChange={(value) => void update({ logLevel: value as "debug" | "info" | "warn" | "error" })}
              value={settings.logLevel}
            >
              <SelectTrigger className="w-52"><SelectValue /></SelectTrigger>
              <SelectContent>
                {["debug", "info", "warn", "error"].map((level) => (
                  <SelectItem key={level} value={level}>{level}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
