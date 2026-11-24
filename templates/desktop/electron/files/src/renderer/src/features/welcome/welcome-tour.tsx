import { FolderOpenIcon, HardDriveIcon, PlusIcon, RefreshCwIcon, SearchIcon } from "lucide-react";
import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

import { Button } from "@/components/ui/button";
import { FeatureRow } from "@/components/ui/feature-row";
import { Inset } from "@/components/ui/inset";
import { Kbd } from "@/components/ui/kbd";
import { Segmented } from "@/components/ui/segmented";
import { Separator } from "@/components/ui/separator";
import { WizardWindow } from "@/components/ui/wizard-window";
import logo from "@renderer/assets/logo.png";
import { useAppSettings } from "@renderer/lib/app-settings";
import { acceleratorFor, commandIds, formatAccelerator, type CommandId } from "@shared/commands";
import { SizeStepper } from "@renderer/components/size-stepper";
import { zoom } from "@shared/zoom";

import { useWelcomeTour } from "./welcome-context";
import { nextPage, pageIndex, previousPage, welcomePages } from "./welcome-pages";

const isMac = navigator.userAgent.includes("Mac");

/**
 * The first-run welcome tour, drawn in `WizardWindow` from `components/ui`
 * parts. State lives in `welcome-context.tsx`; page order in
 * `welcome-pages.ts`. Mounted once by the app shell.
 */
export function WelcomeTour({ appName, version, dataPath }: { appName: string; version?: string; dataPath?: string }) {
  const { t } = useTranslation();
  const tour = useWelcomeTour();
  const page = tour.page;
  if (!page) return null;

  const next = nextPage(page);
  const previous = previousPage(page);

  return (
    <WizardWindow
      badge={version ? `v${version}` : undefined}
      labels={{
        back: t("welcome.back"),
        next: next ? t("welcome.next") : t("welcome.finish"),
        close: t("welcome.skip"),
        dragHint: t("welcome.dragHint"),
        progress: t("welcome.progress", { step: pageIndex(page) + 1, total: welcomePages.length }),
      }}
      logoSrc={logo}
      onBack={() => previous && tour.go(previous)}
      onClose={tour.close}
      onNext={() => (next ? tour.go(next) : tour.close())}
      step={pageIndex(page)}
      steps={welcomePages.length}
      title={t("welcome.windowTitle", { app: appName })}
    >
      {page === "intro" ? <IntroPage appName={appName} /> : null}
      {page === "shortcuts" ? <ShortcutsPage /> : null}
      {page === "ready" ? <ReadyPage dataPath={dataPath} /> : null}
    </WizardWindow>
  );
}

function PageTitle({ title, children }: { title: string; children: ReactNode }) {
  return (
    <div className="grid gap-2">
      <h3 className="text-xl font-semibold">{title}</h3>
      <p className="text-sm text-muted-foreground">{children}</p>
    </div>
  );
}

/** A fixed-width label column and a control, like the Appearance/Size rows. */
function LabelledRow({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div className="flex items-center gap-3">
      <span className="w-24 shrink-0 text-sm text-muted-foreground">{label}</span>
      {children}
    </div>
  );
}

function IntroPage({ appName }: { appName: string }) {
  const { t } = useTranslation();
  const { settings, update } = useAppSettings();
  const current = settings?.zoom ?? zoom.default;

  return (
    <>
      <PageTitle title={t("welcome.introTitle", { app: appName })}>{t("welcome.introBody")}</PageTitle>
      <FeatureRow icon={<HardDriveIcon />} title={t("welcome.localTitle")}>
        {t("welcome.localBody")}
      </FeatureRow>
      <FeatureRow icon={<SearchIcon />} title={t("welcome.findTitle")}>
        {t("welcome.findBody")}
      </FeatureRow>
      <FeatureRow icon={<RefreshCwIcon />} title={t("welcome.syncTitle")}>
        {t("welcome.syncBody")}
      </FeatureRow>
      <Separator />
      <LabelledRow label={t("welcome.appearance")}>
        <Segmented
          label={t("welcome.appearance")}
          onChange={(theme) => void update({ theme })}
          options={[
            { value: "system", label: t("settings.themeSystem") },
            { value: "light", label: t("settings.themeLight") },
            { value: "dark", label: t("settings.themeDark") },
          ]}
          value={settings?.theme ?? "system"}
        />
      </LabelledRow>
      <LabelledRow label={t("welcome.size")}>
        <SizeStepper current={current} onChange={(value) => void update({ zoom: value })} />
      </LabelledRow>
    </>
  );
}

function Shortcut({ id }: { id: CommandId }) {
  const accelerator = acceleratorFor(id);

  return accelerator ? <Kbd>{formatAccelerator(accelerator, isMac)}</Kbd> : null;
}

function ShortcutsPage() {
  const { t } = useTranslation();
  const rows: [CommandId, string][] = [
    [commandIds.newItem, t("welcome.keyNew")],
    [commandIds.toggleSidebar, t("welcome.keySidebar")],
    [commandIds.settings, t("welcome.keySettings")],
    [commandIds.zoomIn, t("welcome.keyZoomIn")],
    [commandIds.zoomOut, t("welcome.keyZoomOut")],
  ];

  return (
    <>
      <PageTitle title={t("welcome.keysTitle")}>{t("welcome.keysBody")}</PageTitle>
      <Inset variant="callout">
        <div className="flex items-start gap-3">
          <Shortcut id={commandIds.palette} />
          <div className="grid gap-1">
            <p className="text-sm font-semibold">{t("welcome.paletteTitle")}</p>
            <p className="text-sm text-muted-foreground">{t("welcome.paletteBody")}</p>
          </div>
        </div>
      </Inset>
      <dl className="grid gap-2 text-sm">
        {rows.map(([id, label]) => (
          <div className="flex items-center gap-3" key={id}>
            <dt className="w-36 shrink-0">
              <Shortcut id={id} />
            </dt>
            <dd>{label}</dd>
          </div>
        ))}
        <div className="flex items-center gap-3">
          <dt className="w-36 shrink-0">
            <Kbd>Esc</Kbd>
          </dt>
          <dd>{t("welcome.keyEscape")}</dd>
        </div>
      </dl>
    </>
  );
}

function ReadyPage({ dataPath }: { dataPath?: string }) {
  const { t } = useTranslation();
  const tour = useWelcomeTour();
  const navigate = useNavigate();

  return (
    <>
      <PageTitle title={t("welcome.readyTitle")}>{t("welcome.readyBody")}</PageTitle>
      <Inset>
        <p className="text-sm font-semibold">{t("welcome.dataTitle")}</p>
        <p className="text-sm text-muted-foreground">{t("welcome.dataBody")}</p>
        <code className="rounded-md bg-background px-2 py-1.5 text-xs">{dataPath ?? "—"}</code>
        <div>
          <Button
            onClick={() => {
              tour.close();
              navigate("/settings");
            }}
            size="sm"
            variant="outline"
          >
            <FolderOpenIcon />
            {t("welcome.dataSettings")}
          </Button>
        </div>
      </Inset>
      <Inset>
        <p className="text-sm font-semibold">{t("welcome.firstTitle")}</p>
        <p className="text-sm text-muted-foreground">{t("welcome.firstBody")}</p>
        <div>
          <Button
            onClick={() => {
              tour.close();
              navigate("/library", { state: { newItem: true } });
            }}
            size="sm"
            variant="outline"
          >
            <PlusIcon />
            {t("welcome.firstAction")}
          </Button>
        </div>
      </Inset>
      <p className="text-sm text-muted-foreground">{t("welcome.reopen")}</p>
    </>
  );
}
