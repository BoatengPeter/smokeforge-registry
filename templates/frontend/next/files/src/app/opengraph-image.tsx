import { ImageResponse } from "next/og";
import { site } from "@/lib/site";

export const alt = site.name;
export const size = {
  width: 1200,
  height: 630,
};
export const contentType = "image/png";

export default function OpenGraphImage() {
  return new ImageResponse(
    (
      <div
        style={{
          alignItems: "flex-start",
          background: "#f7f5ef",
          color: "#1b1b1f",
          display: "flex",
          flexDirection: "column",
          height: "100%",
          justifyContent: "space-between",
          padding: "72px",
          width: "100%",
        }}
      >
        <div
          style={{
            borderBottom: "1px solid #cfc9bd",
            display: "flex",
            fontSize: 24,
            letterSpacing: "0.02em",
            paddingBottom: 24,
            width: "100%",
          }}
        >
          smokeforge
        </div>
        <div style={{ display: "flex", flexDirection: "column", gap: 24 }}>
          <div
            style={{
              fontFamily: "Georgia, serif",
              fontSize: 84,
              lineHeight: 1.05,
              maxWidth: 900,
            }}
          >
            {site.name}
          </div>
          <div
            style={{
              color: "#5f5a52",
              fontSize: 30,
              lineHeight: 1.4,
              maxWidth: 900,
            }}
          >
            {site.description}
          </div>
        </div>
      </div>
    ),
    size,
  );
}
