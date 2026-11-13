"use client";

import Link from "next/link";
import { AnimatePresence, motion, useReducedMotion, type Variants } from "motion/react";
import { X } from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { site } from "@/lib/site";

const primary = [
  { label: "Work", href: "#work" },
  { label: "About", href: "#studio" },
  { label: "Services", href: "#services" },
  { label: "Contact", href: "#contact" },
];

const services = [
  "Full-stack apps",
  "Developer tools",
  "Design systems",
  "AI workflows",
];

const curtain: Variants = {
  hidden: { clipPath: "inset(0 0 100% 0)" },
  visible: {
    clipPath: "inset(0 0 0% 0)",
    transition: { duration: 0.62, ease: [0.76, 0, 0.24, 1] },
  },
  exit: {
    clipPath: "inset(0 0 100% 0)",
    transition: { duration: 0.38, ease: [0.76, 0, 0.24, 1] },
  },
};

const panel: Variants = {
  hidden: {},
  visible: {
    transition: { delayChildren: 0.18, staggerChildren: 0.065 },
  },
  exit: {
    transition: { staggerChildren: 0.025, staggerDirection: -1 },
  },
};

const item: Variants = {
  hidden: { opacity: 0, y: 28 },
  visible: {
    opacity: 1,
    y: 0,
    transition: { duration: 0.58, ease: [0.19, 1, 0.22, 1] },
  },
  exit: {
    opacity: 0,
    y: 12,
    transition: { duration: 0.2, ease: "easeIn" },
  },
};

export function SiteMenu() {
  const [open, setOpen] = useState(false);
  const reduceMotion = useReducedMotion();
  const closeRef = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    if (!open) {
      return;
    }

    const previous = document.body.style.overflow;
    document.body.style.overflow = "hidden";
    closeRef.current?.focus();

    function onKeyDown(event: KeyboardEvent) {
      if (event.key === "Escape") {
        setOpen(false);
      }
    }

    window.addEventListener("keydown", onKeyDown);

    return () => {
      document.body.style.overflow = previous;
      window.removeEventListener("keydown", onKeyDown);
    };
  }, [open]);

  function close() {
    setOpen(false);
  }

  return (
    <>
      <button
        aria-controls="site-menu"
        aria-expanded={open}
        aria-label="Open menu"
        className="flex cursor-pointer items-center gap-3"
        onClick={() => setOpen(true)}
        type="button"
      >
        <span aria-hidden="true" className="flex w-6 flex-col gap-1.5">
          <span className="h-px w-full bg-cream/80" />
          <span className="h-px w-full bg-cream/80" />
          <span className="h-px w-full bg-cream/80" />
        </span>
        <span className="hidden text-sm text-cream/75 sm:inline">Menu</span>
      </button>

      <AnimatePresence>
        {open ? (
          <motion.div
            animate={reduceMotion ? { opacity: 1 } : "visible"}
            aria-label="Site menu"
            aria-modal="true"
            className="fixed inset-0 z-[100] overflow-y-auto bg-ink text-cream"
            exit={reduceMotion ? { opacity: 0 } : "exit"}
            id="site-menu"
            initial={reduceMotion ? { opacity: 0 } : "hidden"}
            role="dialog"
            variants={reduceMotion ? undefined : curtain}
          >
            <motion.div
              animate="visible"
              className="mx-auto flex min-h-dvh w-full max-w-[90rem] flex-col px-5 py-5 sm:px-8 lg:px-12"
              exit="exit"
              initial="hidden"
              variants={reduceMotion ? undefined : panel}
            >
              <motion.div
                className="grid grid-cols-[1fr_auto_1fr] items-center gap-4"
                variants={reduceMotion ? undefined : item}
              >
                <button
                  aria-label="Close menu"
                  className="justify-self-start"
                  onClick={close}
                  ref={closeRef}
                  type="button"
                >
                  <X aria-hidden="true" className="size-8" strokeWidth={1.25} />
                </button>
                <span className="grid size-16 place-items-center rounded-full border border-cream/50 font-display text-xl font-bold uppercase">
                  {site.name.charAt(0)}
                </span>
                <Link
                  className="justify-self-end rounded-full border border-cream/50 px-5 py-2 text-sm font-semibold transition-colors hover:bg-cream hover:text-charcoal"
                  href="#contact"
                  onClick={close}
                >
                  Work with me
                </Link>
              </motion.div>

              <div className="mt-20 grid gap-12 sm:grid-cols-2 lg:grid-cols-4 lg:gap-10">
                <motion.div variants={reduceMotion ? undefined : item}>
                  <p className="text-xs font-bold uppercase tracking-[0.18em] text-cream/50">
                    Explore
                  </p>
                  <nav aria-label="Menu primary" className="mt-6 flex flex-col gap-4">
                    {primary.map((item) => (
                      <Link
                        className="font-display text-3xl font-bold uppercase tracking-[-0.04em] transition-colors hover:text-tide"
                        href={item.href}
                        key={item.label}
                        onClick={close}
                      >
                        {item.label}
                      </Link>
                    ))}
                  </nav>
                </motion.div>

                <motion.div variants={reduceMotion ? undefined : item}>
                  <p className="text-xs font-bold uppercase tracking-[0.18em] text-cream/50">
                    Services
                  </p>
                  <div className="mt-6 flex flex-col gap-4 text-sm text-cream/70">
                    {services.map((service) => (
                      <Link
                        className="transition-colors hover:text-cream"
                        href="#services"
                        key={service}
                        onClick={close}
                      >
                        {service}
                      </Link>
                    ))}
                  </div>
                </motion.div>

                <motion.div variants={reduceMotion ? undefined : item}>
                  <p className="text-xs font-bold uppercase tracking-[0.18em] text-cream/50">
                    More
                  </p>
                  <div className="mt-6 flex flex-col gap-4 text-sm text-cream/70">
                    <Link className="hover:text-cream" href="https://github.com/BoatengPeter" onClick={close}>
                      GitHub
                    </Link>
                    <Link className="hover:text-cream" href="mailto:boatengkonadupeter@gmail.com" onClick={close}>
                      Email
                    </Link>
                    <Link className="hover:text-cream" href="/api/health" onClick={close}>
                      Health
                    </Link>
                    {/* smokeforge:anchor auth-menu */}
                  </div>
                </motion.div>

                <motion.div variants={reduceMotion ? undefined : item}>
                  <p className="text-xs font-bold uppercase tracking-[0.18em] text-cream/50">
                    Availability
                  </p>
                  <p className="mt-6 max-w-xs text-sm leading-6 text-cream/70">
                    Remote, available for select product engineering and developer
                    tooling work.
                  </p>
                  <Link
                    className="mt-6 inline-block border-b border-cream/40 pb-1 text-sm font-semibold transition-colors hover:border-tide hover:text-tide"
                    href="mailto:boatengkonadupeter@gmail.com"
                    onClick={close}
                  >
                    Start a conversation
                  </Link>
                </motion.div>
              </div>

              <motion.div
                className="mt-auto grid gap-6 border-t border-cream/15 pt-6 text-sm text-cream/55 sm:grid-cols-3"
                variants={reduceMotion ? undefined : item}
              >
                <div>
                  <p className="text-cream/35">Contact</p>
                  <p className="mt-2">boatengkonadupeter@gmail.com</p>
                </div>
                <div>
                  <p className="text-cream/35">Based</p>
                  <p className="mt-2">Remote, available for select work</p>
                </div>
                <div>
                  <p className="text-cream/35">Practice</p>
                  <p className="mt-2">{site.name}</p>
                </div>
              </motion.div>
            </motion.div>
          </motion.div>
        ) : null}
      </AnimatePresence>
    </>
  );
}
