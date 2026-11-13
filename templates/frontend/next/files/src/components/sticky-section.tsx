"use client";

import { useEffect, useRef, useState, type ReactNode } from "react";

type StickySectionProps = {
  children: ReactNode;
  className?: string;
  id?: string;
};

export function StickySection({ children, className, id }: StickySectionProps) {
  const ref = useRef<HTMLElement>(null);
  const [top, setTop] = useState<number | null>(null);

  useEffect(() => {
    const element = ref.current;
    if (!element) {
      return;
    }

    function measure() {
      const section = ref.current;
      if (!section) {
        return;
      }

      setTop(Math.min(0, window.innerHeight - section.offsetHeight));
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
    <section
      className={className}
      id={id}
      ref={ref}
      style={top === null ? undefined : { position: "sticky", top }}
    >
      {children}
    </section>
  );
}
