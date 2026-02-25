"use client";

import { useExtracted } from "@/i18n/client";

export default function AuthLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const t = useExtracted("auth");

  return <div>{children}</div>;
}
