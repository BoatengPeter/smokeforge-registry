---
version: "alpha"
name: Smoke Forge
description: A warm editorial developer portfolio with documentary imagery, cream ground, charcoal ink, teal accent, and coral signal.
colors:
  primary: "#313436"
  secondary: "#2fabaa"
  tertiary: "#ff7448"
  neutral: "#fbf8f3"
  surface: "#f1ece3"
  accent: "#2fabaa"
  ink: "#272727"
  hero-scrim: "#182120"
  featured-scrim: "#0d1413"
  sand: "#e7e0d5"
  muted: "#6f6b64"
typography:
  h1:
    fontFamily: "Manrope, Arial, sans-serif"
    fontSize: "clamp(4.5rem, 12vw, 10rem)"
    fontWeight: 800
    lineHeight: 0.84
    letterSpacing: "-0.07em"
  h2:
    fontFamily: "Manrope, Arial, sans-serif"
    fontSize: "clamp(2rem, 5vw, 4.75rem)"
    fontWeight: 700
    lineHeight: 0.98
    letterSpacing: "-0.05em"
  body-md:
    fontFamily: "Manrope, Arial, sans-serif"
    fontSize: "1rem"
    fontWeight: 400
    lineHeight: 1.7
    letterSpacing: "0em"
  label:
    fontFamily: "Manrope, Arial, sans-serif"
    fontSize: "0.72rem"
    fontWeight: 700
    lineHeight: 1
    letterSpacing: "0.18em"
rounded:
  card: "28px"
  pill: "999px"
  input: "12px"
spacing:
  section: "clamp(5rem, 10vw, 9rem)"
  gutter: "clamp(1.25rem, 4vw, 3rem)"
components:
  button-primary:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.neutral}"
    rounded: "{rounded.pill}"
    padding: "14px 24px"
  button-inverse:
    backgroundColor: "{colors.neutral}"
    textColor: "{colors.primary}"
    rounded: "{rounded.pill}"
    padding: "14px 24px"
  card:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.primary}"
    rounded: "{rounded.card}"
    padding: "24px"

---

# Design System: Smoke Forge

## Overview

**Creative North Star: "The Neighborhood Table"**

The system should feel like work made by a person who cares about craft: warm
cream paper, charcoal ink, and a room full of natural light. It is editorial
without being precious, confident without becoming corporate, and rooted in a
place without turning locality into decoration.

The first screen is led by one documentary image and a large, lowercase wordmark.
Sections alternate between generous cream fields and deep charcoal blocks.
Teal marks the connective tissue—labels, links, and small confirmations—while
coral appears sparingly as a human signal. Layout is spacious, but the page
never feels empty: imagery, rules, and type carry the rhythm.

- **Density:** 4/10 — Airy editorial
- **Variance:** 5/10 — Asymmetric, image-led sections
- **Motion:** 2/10 — Quiet transitions only

- **Style:** Warm editorial, community-rooted, modern craft
- **Keywords:** cream, charcoal, teal, coral, documentary, geometric sans, local, human, calm, confident
- **Era:** Contemporary independent developer
- **Light/Dark:** ✓ Light / ✗ No dark mode

**Key Characteristics:**
- Cream ground, charcoal ink, pigment-like teal and coral accents.
- Geometric sans typography with tight display tracking.
- Full-bleed documentary imagery, never abstract tech stock.
- Pill actions, editorial cards, hairline rules, and flat tonal depth.
- One strong idea per section; no dashboard or SaaS-card habits.

## Colors

The palette is warm paper and ink with two saturated accents used like printed
spot color.

### Primary
- **Charcoal Ink** (#313436): Body text, primary buttons, dark sections, and
  the footer. The default ink is warm charcoal, never pure black.

### Secondary
- **Tide Teal** (#2fabaa): Eyebrows, inline links, focus rings, small marks,
  and status chips. It is the connective accent, never a glow.

### Tertiary
- **Signal Coral** (#ff7448): One human highlight per composition: a rule,
  arrow, number, or short emphasis. Do not turn it into a second primary.

### Neutral
- **Paper Cream** (#fbf8f3): Page ground, inverse text, and the calm field
  behind editorial sections.
- **Deep Ink** (#272727): Footer ground and full-bleed dark fields.
- **Hero Scrim** (#182120): Translucent layer over the hero image for legibility.
- **Featured Scrim** (#0d1413): Translucent layer over the featured image.
- **Warm Sand** (#e7e0d5): Dividers, quiet backgrounds, and image frames.
- **Paper Mist** (#f1ece3): Card fills and quiet surfaces.
- **Muted Bark** (#6f6b64): Secondary copy on cream.

### Named Rules

**The One Signal Rule.** Coral appears on no more than one small moment in a
section. Its rarity is what makes it feel human.

**The Warm Ink Rule.** Never use `#000` for text or surfaces. Use charcoal or
deep ink so the page keeps its paper warmth.

## Typography

**Display Font:** Manrope (with Arial, sans-serif fallback)
**Body Font:** Manrope (with Arial, sans-serif fallback)
**Label Font:** Manrope, uppercase with wide tracking
**Scale:** H1 `clamp(4.5rem, 12vw, 10rem)`, H2 `clamp(2rem, 5vw, 4.75rem)`,
body `1rem/1.7`, label `0.72rem/0.18em`.

**Character:** A geometric grotesque in the spirit of Metropolis: open,
confident, and friendly at large sizes. Display type is heavy, tight, and
lowercase for the wordmark. Body copy stays quiet and readable.

### Hierarchy
- **Display** (800, clamp 4.5rem–10rem, 0.84): Hero wordmark and the largest
  statement. Lowercase, tight tracking, no decorative italic.
- **Headline** (700, clamp 2rem–4.75rem, 0.98): Section statements and feature
  titles. Keep lines short and intentional.
- **Body** (400, 1rem, 1.7): Supporting copy. Maximum measure around 65
  characters; never set long paragraphs across the full page.
- **Label** (700, 0.72rem, 0.18em tracking): Eyebrows, section numbers, and
  navigation utility labels. Uppercase only here.

## Layout

The page uses a centered `max-width: 90rem` frame with a fluid gutter of
`clamp(1.25rem, 4vw, 3rem)`. The hero wordmark is centered on the charcoal
canvas. Its photo card overlaps the wordmark at the top and bleeds into the
cream section below, with rounded corners against the cream. Sections breathe
at `clamp(5rem, 10vw, 9rem)` vertical rhythm. Major section transitions use
rounded 50px corners. The hero + intro block and the dark studio section scroll normally, then pin
only when their bottoms reach the viewport, allowing the next section to slide
over them. Keep the studio sticky behavior inside its bounded dark-to-featured
wrapper; never let either sticky block extend under the next-steps or footer
areas.

Use a 12-column editorial grid on large screens: headline columns, image
columns, and asymmetric whitespace. Collapse to one column below 768px, but keep
the same reading order. Cards may become horizontal image/text pairs on tablet
and stack on mobile. Never create horizontal scrolling.

## Elevation & Depth

The system is flat and tonal. Depth comes from image value, charcoal blocks,
hairline rules, and overlapping type—not from shadows. Cards use a one-pixel
warm border or a quiet fill. Do not add glassmorphism, neon glows, or floating
shadow piles.

## Shapes

- **Pill actions:** All primary and secondary buttons use a fully rounded pill.
- **Editorial cards:** 28px radius, with image corners matching the card.
- **Inputs:** 12px radius, warm sand border, charcoal text.
- **Images:** Full-bleed in the hero; 4:5 or 3:2 in editorial cards; never put
  every image in a rounded floating tile with a shadow.

## Components

- **Navigation:** Transparent over the hero with cream text; solid cream with
  charcoal text after scroll or on interior pages. Wordmark is lowercase.
- **Buttons:** Primary charcoal on cream, inverse cream on dark imagery. Labels
  are sentence case, never shouty uppercase.
- **Editorial cards:** Image first, then a small label, short headline, and one
  sentence. No icon tile above every heading.
- **Section markers:** Small teal uppercase labels with a coral rule or number
  used sparingly.
- **Footer:** Deep ink field, cream wordmark, quiet link columns, and one small
  "Health" utility link.
- **Motion:** A **scroll reveal** fades and translates major sections up 24px
  over 720ms with an ease-out `cubic-bezier(0.19, 1, 0.22, 1)`. The work header
  and grid use a small **stagger**. Primary buttons have a left-to-right fill
  **hover effect**; work images scale to 1.03 on hover. Reduced motion users get
  the final state immediately. Nothing loops or animates on idle.

## Imagery

Use documentary photography with real people, real hands, and real places.
Prefer Pacific Northwest overcast light, natural skin tones, cream and charcoal
clothing, and small teal/coral details. Avoid generic office stock, purple
technology gradients, and staged handshakes.

The demo photography lives in `public/images/`. Replacement prompts and target
filenames are documented in `public/images/PROMPTS.md`.

## Do's and Don'ts

### Do
- Lead each section with one clear idea and one strong image or statement.
- Use lowercase display type, tight tracking, and generous line breaks.
- Keep teal and coral rare enough that they still feel like signals.
- Use real developer copy and real project evidence when available.
- Verify the page at 360px, 768px, 1024px, and 1440px before shipping.

### Don't
- Don't use pure black, purple gradients, glassmorphism, or generic SaaS cards.
- Don't put a rounded icon tile or an all-caps eyebrow above every heading.
- Don't use numbered markers unless the content is genuinely sequential.
- Don't append an arrow to every link or button.
- Don't use middle-dot meta strings or spaced-em-dash labels as decoration.
- Don't make every section the same card grid.
- Don't use abstract technology imagery where a human moment would be stronger.
- Don't invent clients, testimonials, awards, or performance metrics.

## Use Case

Developer portfolios, independent product sites, and editorial marketing pages
that need a warm, human alternative to generic SaaS design.
