import { formatDistanceToNow } from "date-fns";
import { Play, RotateCcw, Square } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Progress } from "@/components/ui/progress";
import { Separator } from "@/components/ui/separator";

import { useActivity } from "./activity-context";

export function ActivityPage() {
  const { t } = useTranslation();
  const { state, start, cancel, retry } = useActivity();
  const running = state?.status === "running";

  return (
    <div className="mx-auto grid w-full max-w-4xl gap-5 p-6">
      <Card>
        <CardHeader className="flex-row items-center justify-between">
          <div>
            <CardTitle>{t("activity.steps")}</CardTitle>
            <p className="mt-1 text-sm text-muted-foreground">
              {state?.status === "idle" || !state ? t("activity.noRun") : state.step}
            </p>
          </div>
          <Badge className="capitalize" variant={running ? "default" : "secondary"}>
            {t(`activity.status.${state?.status ?? "idle"}`)}
          </Badge>
        </CardHeader>
        <CardContent className="grid gap-5">
          <div>
            <div className="mb-2 flex items-center justify-between text-xs text-muted-foreground">
              <span>{t("activity.progress")}</span>
              <span>{state?.progress ?? 0}%</span>
            </div>
            <Progress value={state?.progress ?? 0} />
          </div>
          <Separator />
          <div className="flex flex-wrap items-center gap-2">
            <Button disabled={running} onClick={() => void start()}>
              <Play className="size-4" />
              {t("activity.start")}
            </Button>
            <Button disabled={!running} onClick={() => void cancel()} variant="outline">
              <Square className="size-4" />
              {t("activity.cancel")}
            </Button>
            <Button disabled={running} onClick={() => void retry()} variant="ghost">
              <RotateCcw className="size-4" />
              {t("activity.retry")}
            </Button>
          </div>
          {state?.finishedAt ? (
            <p className="text-xs text-muted-foreground">
              {t("activity.lastRun")} {formatDistanceToNow(new Date(state.finishedAt), { addSuffix: true })}
            </p>
          ) : null}
        </CardContent>
      </Card>
    </div>
  );
}
