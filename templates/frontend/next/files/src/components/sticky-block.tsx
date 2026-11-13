"use client";

import { useEffect, useRef, useState, type ReactNode } from "react";

type StickyBlockProps = {
  children: ReactNode;
  className?: string;
};

export function StickyBlock({ children, className }: StickyBlockProps) {
  const ref = useRef<HTMLDivElement>(null);
  const [top, setTop] = useState<number | null>(null);

  useEffect(() => {
    const element = ref.current;
    if (!element) {
      return;
    }

    function measure() {
      const block = ref.current;
      if (!block) {
        return;
      }

      setTop(Math.min(0, window.innerHeight - block.offsetHeight));
    }

    measure();

    const observer = new ResizeObserver(measure);
    observer.observe(element);
    window.addEventListener("resize", measure);

    return () => {
      observer.disconnect();
      window.removeEventListener("resize", measure);
    };
  }, []);

  return (
    <div
      className={className}
      ref={ref}
      style={top === null ? undefined : { position: "sticky", top }}
    >
      {children}
    </div>
  );
}
