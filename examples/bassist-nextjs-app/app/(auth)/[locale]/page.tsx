"use client";

import { useExtracted } from "@/i18n/client";
import { createClient } from "@/lib/supabase/client";

export default function AuthPage() {
  const t = useExtracted("auth.login");
  const supabase = createClient();

  return <div>{t("Login")}</div>;
}
