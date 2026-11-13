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
          background: "#fbf8f3",
          color: "#313436",
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
            borderBottom: "1px solid #e7e0d5",
            display: "flex",
            fontSize: 24,
            letterSpacing: "0.18em",
            paddingBottom: 24,
            width: "100%",
          }}
        >
          smokeforge
        </div>
        <div style={{ display: "flex", flexDirection: "column", gap: 24 }}>
          <div
            style={{
              fontFamily: "Arial, sans-serif",
              fontSize: 92,
              lineHeight: 1.05,
              maxWidth: 900,
            }}
          >
            {site.name}
          </div>
          <div
            style={{
              color: "#6f6b64",
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
