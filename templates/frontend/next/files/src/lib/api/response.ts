import { NextResponse } from "next/server";
import { failure, success } from "@/lib/api/envelope";

export function jsonOk<T>(data: T, status = 200): NextResponse {
  return NextResponse.json(success(data), { status });
}

export function jsonCreated<T>(data: T): NextResponse {
  return jsonOk(data, 201);
}

export function jsonError(
  code: string,
  message: string,
  status = 400,
  fields?: Record<string, string[]>,
): NextResponse {
  return NextResponse.json(failure(code, message, fields), { status });
}
