import { MinusIcon, PlusIcon } from "lucide-react";

import { Button } from "@/components/ui/button";

export type ValueStepperProps = {
  /** The formatted value: "130%". */
  value: string;
  onDecrease: () => void;
  onIncrease: () => void;
  canDecrease: boolean;
  canIncrease: boolean;
  /** Accessible names: "Make the interface smaller" / "… larger". */
  decreaseLabel: string;
  increaseLabel: string;
};

/** A compact − value + control for small numeric settings (interface size). */
export function ValueStepper({
  value,
  onDecrease,
  onIncrease,
  canDecrease,
  canIncrease,
  decreaseLabel,
  increaseLabel,
}: ValueStepperProps) {
  return (
    <div className="inline-flex items-center gap-0.5 rounded-md border bg-muted p-0.5">
      <Button
        aria-label={decreaseLabel}
        disabled={!canDecrease}
        onClick={onDecrease}
        size="icon-xs"
        title={decreaseLabel}
        variant="ghost"
      >
        <MinusIcon />
      </Button>
      <output aria-live="polite" className="w-14 text-center text-sm tabular-nums">
        {value}
      </output>
      <Button
        aria-label={increaseLabel}
        disabled={!canIncrease}
        onClick={onIncrease}
        size="icon-xs"
        title={increaseLabel}
        variant="ghost"
      >
        <PlusIcon />
      </Button>
    </div>
  );
}
