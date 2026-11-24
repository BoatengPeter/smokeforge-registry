import { useTranslation } from "react-i18next";

import { ValueStepper } from "@/components/ui/value-stepper";
import { stepZoom, zoom, zoomLabel } from "@shared/zoom";

/** − 100% + for the interface size, shared by Settings and the welcome tour. */
export function SizeStepper({ current, onChange }: { current: number; onChange: (value: number) => void }) {
  const { t } = useTranslation();
  const smaller = stepZoom(current, -zoom.step);
  const larger = stepZoom(current, zoom.step);

  return (
    <ValueStepper
      canDecrease={smaller !== current}
      canIncrease={larger !== current}
      decreaseLabel={t("settings.sizeSmaller")}
      increaseLabel={t("settings.sizeLarger")}
      onDecrease={() => onChange(smaller)}
      onIncrease={() => onChange(larger)}
      value={zoomLabel(current)}
    />
  );
}
