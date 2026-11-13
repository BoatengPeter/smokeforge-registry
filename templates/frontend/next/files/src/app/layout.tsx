import type { Metadata } from "next";
import { Manrope } from "next/font/google";
import type { ReactNode } from "react";
import "./globals.css";
import { site } from "@/lib/site";
// smokeforge:anchor analytics-imports
import { Providers } from "@/components/providers";

const manrope = Manrope({
  subsets: ["latin"],
  variable: "--font-manrope",
  display: "swap",
});

export const metadata: Metadata = {
  metadataBase: new URL(site.url),
  title: {
    default: site.name,
    template: `%s | ${site.name}`,
  },
  description: site.description,
  openGraph: {
    type: "website",
    url: site.url,
    title: site.name,
    description: site.description,
    siteName: site.name,
    images: [
      {
        url: "/opengraph-image",
        width: 1200,
        height: 630,
        alt: site.name,
      },
    ],
  },
  twitter: {
    card: "summary_large_image",
    title: site.name,
    description: site.description,
    images: ["/opengraph-image"],
  },
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="en">
      <body
        className={`${manrope.variable} min-h-dvh bg-background font-sans text-foreground antialiased`}
      >
        <Providers>{children}</Providers>
        {/* smokeforge:anchor analytics */}
      </body>
    </html>
  );
}
