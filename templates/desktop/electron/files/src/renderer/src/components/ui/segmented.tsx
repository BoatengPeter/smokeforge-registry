import { cn } from "@/lib/utils";

export type SegmentedOption<T extends string> = { value: T; label: string };

/**
 * 2–4 mutually exclusive options shown side by side (System / Light / Dark).
 * A radio group for assistive tech; use a Select when there are more options.
 */
export function Segmented<T extends string>({
  label,
  value,
  options,
  onChange,
}: {
  /** Accessible name of the group. */
  label: string;
  value: T;
  options: SegmentedOption<T>[];
  onChange: (value: T) => void;
}) {
  return (
    <div aria-label={label} className="inline-flex gap-0.5 rounded-md border bg-muted p-0.5" role="radiogroup">
      {options.map((option) => {
        const selected = option.value === value;

        return (
          <button
            aria-checked={selected}
            className={cn(
              "h-7 rounded-[5px] px-3 text-sm transition-colors focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none",
              selected
                ? "border bg-background text-foreground shadow-xs"
                : "text-muted-foreground hover:text-foreground",
            )}
            key={option.value}
            onClick={() => onChange(option.value)}
            role="radio"
            type="button"
          >
            {option.label}
          </button>
        );
      })}
    </div>
  );
}
