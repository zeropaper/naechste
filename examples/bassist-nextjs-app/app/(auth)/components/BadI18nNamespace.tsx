// VIOLATION: Missing namespace in i18n key
"use client";

import { useExtracted } from "@/i18n/client";

export function BadNamespace() {
  const t = useExtracted("login"); // Should be "auth.login"
  return <div>{t("Login")}</div>;
}
