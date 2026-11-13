"use client";

import Image from "next/image";
import Link from "next/link";
import { ArrowLeft, ArrowRight } from "lucide-react";
import { useRef } from "react";
import { Reveal } from "@/components/reveal";

type Service = {
  title: string;
  description: string;
  image: string;
  alt: string;
};

type ServicesCarouselProps = {
  services: readonly Service[];
};

export function ServicesCarousel({ services }: ServicesCarouselProps) {
  const trackRef = useRef<HTMLDivElement>(null);

  function scroll(direction: -1 | 1) {
    const track = trackRef.current;
    if (!track) {
      return;
    }

    track.scrollBy({
      behavior: "smooth",
      left: direction * track.clientWidth * 0.8,
    });
  }

  return (
    <section
      className="mx-auto w-full max-w-[80rem] px-5 pb-[clamp(5rem,10vw,8rem)] pt-[clamp(5rem,10vw,8rem)] sm:px-8 lg:px-12"
      id="services"
    >
      <Reveal className="flex flex-wrap items-end justify-between gap-6">
        <h2 className="max-w-[12ch] font-display text-[clamp(2.8rem,5.5vw,5.5rem)] font-bold uppercase leading-[0.88] tracking-[-0.055em]">
          Take your next step
        </h2>
        <div className="flex gap-3">
          <button
            aria-label="Previous services"
            className="icon-fill grid size-14 place-items-center rounded-full"
            onClick={() => scroll(-1)}
            type="button"
          >
            <ArrowLeft aria-hidden="true" className="size-5" strokeWidth={1.5} />
          </button>
          <button
            aria-label="Next services"
            className="icon-fill grid size-14 place-items-center rounded-full"
            onClick={() => scroll(1)}
            type="button"
          >
            <ArrowRight aria-hidden="true" className="size-5" strokeWidth={1.5} />
          </button>
        </div>
      </Reveal>

      <div
        className="carousel-scroll mt-16 flex snap-x snap-mandatory gap-5 overflow-x-auto pb-2"
        ref={trackRef}
      >
        {services.map((service) => (
          <Reveal
            className="w-[82%] shrink-0 snap-start sm:w-[48%] lg:w-[31.9%]"
            key={service.title}
          >
            <Link className="group block" href="#contact">
              <div className="relative aspect-[3/2] overflow-hidden rounded-[20px] bg-charcoal">
                <Image
                  alt={service.alt}
                  className="object-cover transition-transform duration-700 ease-out group-hover:scale-[1.04]"
                  fill
                  sizes="(max-width: 640px) 82vw, (max-width: 1024px) 48vw, 32vw"
                  src={service.image}
                />
              </div>
              <div className="mt-6">
                <h3 className="font-display text-2xl font-bold uppercase leading-[0.96] tracking-[-0.04em]">
                  {service.title}
                </h3>
                <p className="mt-3 max-w-sm text-sm leading-6 text-muted-warm">
                  {service.description}
                </p>
                <span className="mt-5 inline-flex items-center gap-2 border-b border-charcoal/30 pb-1 text-sm font-semibold transition-colors group-hover:border-tide group-hover:text-tide">
                  Learn more
                  <ArrowRight aria-hidden="true" className="size-4" strokeWidth={1.5} />
                </span>
              </div>
            </Link>
          </Reveal>
        ))}
      </div>
    </section>
  );
}
