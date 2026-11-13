import Image from "next/image";
import Link from "next/link";
import { ArrowRight, Play } from "lucide-react";
import { Reveal } from "@/components/reveal";
import { ServicesCarousel } from "@/components/services-carousel";
import { SiteMenu } from "@/components/site-menu";
import { StickyBlock } from "@/components/sticky-block";
import { StickySection } from "@/components/sticky-section";
import { Button } from "@/components/ui/button";
import { site } from "@/lib/site";

const ways = [
  {
    title: "Product engineering",
    description:
      "Full-stack applications, APIs, authentication, databases, and the details that make them dependable.",
    image: "/images/product.jpeg",
    alt: "A developer reviewing a product dashboard on a laptop.",
    href: "#contact",
  },
  {
    title: "Developer tools",
    description:
      "CLIs, templates, registries, and the systems that help other developers move with more confidence.",
    image: "/images/brand.jpeg",
    alt: "Printed systems and color swatches arranged on a wooden table.",
    href: "#contact",
  },
] as const;

const nextSteps = [
  {
    title: "Full-stack apps",
    description: "Product engineering from data model to interface.",
    image: "/images/product.jpeg",
    alt: "A developer reviewing a product dashboard on a laptop.",
  },
  {
    title: "Developer tools",
    description: "CLIs, registries, templates, and better developer workflows.",
    image: "/images/brand.jpeg",
    alt: "Printed systems and color swatches arranged on a wooden table.",
  },
  {
    title: "Design systems",
    description: "Reusable UI foundations that stay coherent as products grow.",
    image: "/images/featured.jpeg",
    alt: "A group gathered around a table in a warm studio.",
  },
  {
    title: "AI workflows",
    description: "Practical AI integrations that fit the work people already do.",
    image: "/images/hero.jpeg",
    alt: "A group of friends laughing together outside a white brick building.",
  },
] as const;

export default function Home() {
  return (
    <main className="min-h-dvh overflow-x-clip bg-background">
      <StickyBlock className="relative z-0">
      <section className="bg-ink text-cream">
        <div className="mx-auto flex w-full max-w-[90rem] flex-col px-5 pt-5 sm:px-8 lg:px-12">
          <header className="grid grid-cols-[1fr_auto_1fr] items-center gap-4">
            <div className="flex items-center gap-5 justify-self-start">
              <SiteMenu />
              <Link className="hidden text-sm text-cream/75 transition-colors hover:text-cream sm:inline" href="#work">
                Work
              </Link>
            </div>

            <Link
              aria-label={`${site.name} home`}
              className="justify-self-center"
              href="/"
            >
              <span className="grid size-16 place-items-center rounded-full border border-cream/50 font-display text-xl font-bold uppercase tracking-[-0.04em] text-cream">
                {site.name.charAt(0)}
              </span>
            </Link>

            <div className="flex flex-wrap items-center justify-end gap-2">
              <nav
                aria-label="Secondary"
                className="hidden items-center gap-6 text-sm text-cream/75 md:flex"
              >
                <Link className="transition-colors hover:text-cream" href="#services">
                  Services
                </Link>
              </nav>
              <Button
                asChild
                className="hidden h-11 rounded-full border-cream/50 bg-transparent px-6 text-sm text-cream hover:bg-cream hover:text-charcoal sm:inline-flex"
                size="lg"
                variant="outline"
              >
                <Link href="#contact">Work with me</Link>
              </Button>
              {/* smokeforge:anchor auth-cta */}
            </div>
          </header>

          <Reveal className="relative pt-6 sm:pt-8">
            <h1 className="relative z-20 break-words text-center font-display text-[clamp(3rem,9.5vw,9.5rem)] font-extrabold uppercase leading-[0.82] tracking-[-0.05em] text-cream">
              {site.name}
            </h1>

            <div className="-mb-16 relative z-10 mx-auto -mt-2 w-full max-w-[80rem] overflow-hidden rounded-[35px] bg-charcoal sm:-mb-20 sm:-mt-6">
              <div className="relative min-h-[76svh] sm:min-h-[78svh]">
                <Image
                  alt="A group of friends laughing together outside a white brick building."
                  className="object-cover"
                  fill
                  priority
                  sizes="100vw"
                  src="/images/hero.jpeg"
                />
                <div aria-hidden="true" className="hero-card-wash absolute inset-0" />
                <div className="relative flex min-h-[76svh] flex-col items-center justify-start px-5 pb-16 pt-[clamp(4rem,9vw,8rem)] text-center sm:min-h-[78svh]">
                  <p className="text-sm font-bold uppercase tracking-[0.18em] text-cream">
                    I build thoughtful web products.
                  </p>
                  <p className="mt-5 max-w-xl text-base leading-7 text-cream/85">
                    {site.description} I care about the details that make a
                    product feel reliable, fast, and human.
                  </p>
                  <Button
                    asChild
                    className="mt-8 rounded-full border-cream/60 bg-transparent text-cream hover:bg-cream hover:text-charcoal"
                    variant="outline"
                  >
                    <Link href="#work">See my work</Link>
                  </Button>
                </div>
              </div>
            </div>
          </Reveal>

        </div>
      </section>

      <section className="bg-background">
        <div className="mx-auto w-full max-w-[90rem] px-5 py-[clamp(5rem,10vw,9rem)] sm:px-8 lg:px-12">
          <div className="grid gap-10 border-t border-charcoal/15 pt-8 lg:grid-cols-[1.25fr_0.75fr] lg:gap-20">
            <Reveal>
              <h2 className="max-w-[15ch] font-display text-[clamp(2.8rem,5.5vw,5.5rem)] font-bold uppercase leading-[0.88] tracking-[-0.055em]">
                I build web products and the tools around them.
              </h2>
            </Reveal>
            <Reveal className="flex flex-col items-start justify-end" delay={0.08}>
              <p className="max-w-sm text-base leading-7 text-muted-warm">
                I work across product engineering and developer tooling, with a
                bias toward clear systems, useful interfaces, and code that
                stays understandable.
              </p>
              <Button
                asChild
                className="mt-7 rounded-full border-charcoal/30 bg-transparent px-6 text-charcoal hover:bg-charcoal hover:text-cream"
                variant="outline"
              >
                <Link href="#studio">More about me</Link>
              </Button>
            </Reveal>
          </div>
        </div>
      </section>
      </StickyBlock>

      <div className="relative z-10">
        <StickySection
          className="z-0 -mt-10 rounded-t-[50px] bg-ink text-cream"
          id="studio"
        >
        <div className="mx-auto w-full max-w-[90rem] px-5 py-[clamp(5rem,10vw,8rem)] sm:px-8 lg:px-12">
          <Reveal className="text-center">
            <h2 className="font-display text-[clamp(2.8rem,5.5vw,5.5rem)] font-bold uppercase leading-[0.88] tracking-[-0.055em]">
              One developer
              <br />
              two modes
            </h2>
          </Reveal>

          <div className="mt-16 grid gap-12 md:grid-cols-2 md:gap-8">
            {ways.map((way) => (
              <Reveal key={way.title}>
                <article>
                  <div className="relative aspect-[4/3] overflow-hidden rounded-[28px] bg-charcoal">
                    <Image
                      alt={way.alt}
                      className="object-cover transition-transform duration-700 ease-out hover:scale-[1.03]"
                      fill
                      sizes="(max-width: 768px) 100vw, 50vw"
                      src={way.image}
                    />
                  </div>
                  <div className="mt-6 border-t border-cream/20 pt-5">
                    <h3 className="font-display text-3xl font-bold uppercase tracking-[-0.04em]">
                      {way.title}
                    </h3>
                    <p className="mt-3 max-w-md text-sm leading-6 text-cream/65">
                      {way.description}
                    </p>
                    <Link
                      className="mt-5 inline-block border-b border-cream/40 pb-1 text-sm font-semibold transition-colors hover:border-tide hover:text-tide"
                      href={way.href}
                    >
                      Learn more
                    </Link>
                  </div>
                </article>
              </Reveal>
            ))}
          </div>

          <Reveal className="mt-24 grid gap-10 border-t border-cream/20 pt-10 lg:grid-cols-[1fr_0.95fr] lg:items-center lg:gap-20">
            <div>
              <ArrowRight
                aria-hidden="true"
                className="size-16 text-cream"
                strokeWidth={1}
              />
              <h3 className="mt-8 font-display text-[clamp(2.4rem,4.5vw,4.5rem)] font-bold uppercase leading-[0.9] tracking-[-0.05em]">
                About me
              </h3>
              <p className="mt-6 text-sm font-semibold uppercase leading-6 tracking-[0.08em] text-cream/80">
                Boateng Peter
                <br />
                Full-stack engineer
              </p>
              <Link
                className="mt-8 inline-block border-b border-cream/40 pb-1 text-sm font-semibold transition-colors hover:border-tide hover:text-tide"
                href="#contact"
              >
                More about my work
              </Link>
            </div>
            <div className="relative aspect-[3/2] overflow-hidden rounded-[28px] bg-charcoal">
              <Image
                alt="A community workshop gathered around a table under warm string lights."
                className="object-cover"
                fill
                sizes="(max-width: 1024px) 100vw, 60vw"
                src="/images/featured.jpeg"
              />
            </div>
          </Reveal>
        </div>
        </StickySection>

        <section className="relative z-10 -mt-10 rounded-t-[50px] bg-background">
        <div className="mx-auto w-full max-w-[90rem] px-5 py-[clamp(5rem,10vw,8rem)] sm:px-8 lg:px-12">
          <Reveal className="flex flex-col items-center text-center">
            <h2 className="font-display text-[clamp(2.8rem,5.5vw,5.5rem)] font-bold uppercase leading-[0.88] tracking-[-0.055em]">
              Watch the featured
              <br />
              work
            </h2>
            <Button
              asChild
              className="mt-7 rounded-full border-charcoal/30 bg-transparent px-6 text-charcoal hover:bg-charcoal hover:text-cream"
              variant="outline"
            >
              <Link href="https://github.com/BoatengPeter">More projects</Link>
            </Button>
          </Reveal>

          <Reveal className="mx-auto mt-16 w-full max-w-[80rem]">
            <Link
              aria-label="View the featured work"
              className="group relative block aspect-[16/9] overflow-hidden rounded-[50px] bg-ink"
              href="#contact"
            >
              <Image
                alt="A community workshop gathered around a table under warm string lights."
                className="object-cover transition-transform duration-1000 ease-out group-hover:scale-[1.02]"
                fill
                sizes="(max-width: 1024px) 100vw, 90rem"
                src="/images/hero.jpeg"
              />
              <div aria-hidden="true" className="featured-wash absolute inset-0" />
              <span className="absolute inset-0 grid place-items-center">
                <span className="grid size-20 place-items-center rounded-full bg-cream text-charcoal transition-transform duration-500 group-hover:scale-110 sm:size-24">
                  <Play aria-hidden="true" className="ml-1 size-7 fill-current" />
                </span>
              </span>
            </Link>
          </Reveal>
        </div>
      </section>
      </div>

      <div className="bg-ink">
        <section className="relative z-20 rounded-b-[50px] bg-background">
          <ServicesCarousel services={nextSteps} />
        </section>

        <footer className="sticky bottom-0 z-0 bg-ink text-cream" id="contact">
          <div className="mx-auto grid w-full max-w-[90rem] gap-12 px-5 py-16 sm:px-8 lg:grid-cols-[1.4fr_1fr_1fr_1fr] lg:gap-10 lg:px-12 lg:py-20">
            <div>
              <Link
                aria-label={`${site.name} home`}
                className="inline-grid size-16 place-items-center rounded-full border border-cream/50 font-display text-xl font-bold uppercase tracking-[-0.04em]"
                href="/"
              >
                {site.name.charAt(0)}
              </Link>
              <p className="mt-6 max-w-xs text-sm font-semibold text-cream">
                {site.name}
              </p>
              <p className="mt-2 max-w-xs text-sm leading-6 text-cream/55">
                Software engineer building thoughtful web products, developer
                tools, and the systems around them.
              </p>
            </div>

            <div>
              <p className="text-sm font-semibold text-cream">Explore</p>
              <nav aria-label="Footer explore" className="mt-5 flex flex-col gap-3 text-sm text-cream/60">
                <Link className="hover:text-cream" href="#work">Work</Link>
                <Link className="hover:text-cream" href="#studio">About</Link>
                <Link className="hover:text-cream" href="#services">Services</Link>
                <Link className="hover:text-cream" href="#contact">Contact</Link>
              </nav>
            </div>

            <div>
              <p className="text-sm font-semibold text-cream">Services</p>
              <div className="mt-5 flex flex-col gap-3 text-sm text-cream/60">
                <Link className="hover:text-cream" href="#services">Full-stack apps</Link>
                <Link className="hover:text-cream" href="#services">Developer tools</Link>
                <Link className="hover:text-cream" href="#services">Design systems</Link>
                <Link className="hover:text-cream" href="#services">AI workflows</Link>
              </div>
            </div>

            <div>
              <p className="text-sm font-semibold text-cream">More</p>
              <div className="mt-5 flex flex-col gap-3 text-sm text-cream/60">
                <Link className="hover:text-cream" href="mailto:boatengkonadupeter@gmail.com">Email</Link>
                <Link className="hover:text-cream" href="https://github.com/BoatengPeter">GitHub</Link>
                <Link className="hover:text-cream" href="/api/health">Health</Link>
                <span>Remote, available for select work</span>
                {/* smokeforge:anchor auth-footer */}
              </div>
            </div>
          </div>

          <div className="mx-auto flex w-full max-w-[90rem] flex-wrap items-center justify-between gap-3 border-t border-cream/15 px-5 py-6 text-xs text-cream/40 sm:px-8 lg:px-12">
            <div className="flex flex-wrap gap-4">
              <span>© {site.name}</span>
              <span>Software engineer</span>
            </div>
            <div className="flex items-center gap-4">
              <span>Remote, available for select work</span>
              <Link className="hover:text-cream" href="mailto:boatengkonadupeter@gmail.com">
                Email
              </Link>
            </div>
          </div>
        </footer>
      </div>
    </main>
  );
}
