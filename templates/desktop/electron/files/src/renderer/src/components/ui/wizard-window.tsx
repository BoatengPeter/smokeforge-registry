import { ArrowRightIcon, XIcon } from "lucide-react";
import {
  useEffect,
  useId,
  useRef,
  useState,
  type KeyboardEvent,
  type PointerEvent,
  type ReactNode,
} from "react";

import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

import { clampOffset, type Offset } from "./wizard-drag";

/**
 * A floating, draggable multi-step window: first-run tours, import flows.
 *
 * It floats over the live app without a backdrop (`aria-modal="false"`), so
 * the user can drag it aside by its header and watch a setting take effect.
 * Its size is fixed, so pages of different lengths don't make it jump; long
 * pages scroll. The caller owns the step and draws the page.
 *
 * Keys: Escape closes; Enter / → go forward and ← back while the window
 * itself (not one of its controls) has focus.
 */

export type WizardLabels = {
  back: string;
  /** The forward button on this step: "Next", "Get started". */
  next: string;
  /** Accessible name of the × button: "Skip the tour". */
  close: string;
  /** Header hint: "Drag to move". */
  dragHint: string;
  /** Step dots description: "Step 1 of 3". */
  progress: string;
};

export type WizardWindowProps = {
  title: string;
  /** A small outlined tag after the title ("Beta", a version). */
  badge?: string;
  logoSrc?: string;
  /** Zero-based. */
  step: number;
  steps: number;
  labels: WizardLabels;
  onBack: () => void;
  onNext: () => void;
  onClose: () => void;
  children: ReactNode;
};

export function WizardWindow({
  title,
  badge,
  logoSrc,
  step,
  steps,
  labels,
  onBack,
  onNext,
  onClose,
  children,
}: WizardWindowProps) {
  const titleId = useId();
  const windowRef = useRef<HTMLDivElement>(null);
  const grab = useRef<Offset | null>(null);
  const [offset, setOffset] = useState<Offset>({ x: 0, y: 0 });

  // Take focus on open so the keys work; give it back on close.
  useEffect(() => {
    const previous = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    windowRef.current?.focus();

    return () => previous?.focus();
  }, []);

  const onPointerDown = (event: PointerEvent<HTMLDivElement>) => {
    // Buttons in the header keep their own clicks.
    if (event.button !== 0 || (event.target as HTMLElement).closest("button")) return;
    grab.current = { x: event.clientX - offset.x, y: event.clientY - offset.y };
    event.currentTarget.setPointerCapture(event.pointerId);
  };

  const onPointerMove = (event: PointerEvent<HTMLDivElement>) => {
    const start = grab.current;
    const box = windowRef.current;
    if (!start || !box) return;
    setOffset(
      clampOffset(
        { x: event.clientX - start.x, y: event.clientY - start.y },
        { width: window.innerWidth, height: window.innerHeight },
        { width: box.offsetWidth, height: box.offsetHeight },
      ),
    );
  };

  const endDrag = (event: PointerEvent<HTMLDivElement>) => {
    grab.current = null;
    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId);
    }
  };

  const onKeyDown = (event: KeyboardEvent<HTMLDivElement>) => {
    if (event.key === "Escape") {
      event.preventDefault();
      onClose();
      return;
    }
    const onWindow = event.target === event.currentTarget;
    if (!onWindow || event.altKey || event.ctrlKey || event.metaKey || event.shiftKey) return;
    if (event.key === "Enter" || event.key === "ArrowRight") {
      event.preventDefault();
      onNext();
    } else if (event.key === "ArrowLeft" && step > 0) {
      event.preventDefault();
      onBack();
    }
  };

  return (
    <div className="pointer-events-none fixed inset-0 z-40 flex items-center justify-center">
      <div
        aria-labelledby={titleId}
        aria-modal="false"
        className="pointer-events-auto flex h-[min(46rem,calc(100vh-2rem))] w-[min(52rem,calc(100vw-2rem))] flex-col overflow-hidden rounded-xl border bg-popover text-popover-foreground shadow-2xl outline-none"
        onKeyDown={onKeyDown}
        ref={windowRef}
        role="dialog"
        style={{ transform: `translate(${offset.x}px, ${offset.y}px)` }}
        tabIndex={-1}
      >
        <div
          className="flex h-14 shrink-0 cursor-grab touch-none items-center gap-2.5 border-b pr-3 pl-5 select-none active:cursor-grabbing"
          onPointerCancel={endDrag}
          onPointerDown={onPointerDown}
          onPointerMove={onPointerMove}
          onPointerUp={endDrag}
        >
          {logoSrc ? <img alt="" className="size-6 rounded-md" draggable={false} src={logoSrc} /> : null}
          <h2 className="text-[15px] font-semibold" id={titleId}>
            {title}
          </h2>
          {badge ? (
            <span className="rounded-md border px-1.5 text-[11px] font-semibold text-muted-foreground">
              {badge}
            </span>
          ) : null}
          <span className="ml-auto text-xs text-muted-foreground">{labels.dragHint}</span>
          <Button aria-label={labels.close} onClick={onClose} size="icon-sm" title={labels.close} variant="ghost">
            <XIcon />
          </Button>
        </div>

        <div className="scrollbar-thin min-h-0 flex-1 overflow-y-auto">
          <div className="grid gap-5 px-6 pt-6 pb-5">{children}</div>
        </div>

        <div className="flex h-16 shrink-0 items-center gap-2 border-t px-5">
          <StepDots label={labels.progress} step={step} steps={steps} />
          <div className="ml-auto flex items-center gap-2">
            {step > 0 ? (
              <Button onClick={onBack} variant="ghost">
                {labels.back}
              </Button>
            ) : null}
            <Button onClick={onNext}>
              {labels.next}
              <ArrowRightIcon />
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}

function StepDots({ step, steps, label }: { step: number; steps: number; label: string }) {
  return (
    <div aria-label={label} className="flex items-center gap-1 pl-1" role="img">
      {Array.from({ length: steps }, (_, index) => (
        <span
          className={cn(
            "h-1.5 rounded-full transition-[width] duration-150",
            index === step ? "w-4 bg-foreground" : "w-1.5 bg-input",
          )}
          key={`step-${index}`}
        />
      ))}
    </div>
  );
}
