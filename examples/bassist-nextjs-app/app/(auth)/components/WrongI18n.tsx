"use client";

// VIOLATION: Client component using server function
import { getExtracted } from "@/i18n/server";

export function ClientComponent() {
  const t = getExtracted("error");
  return <div>{t("This is wrong")}</div>;
}
