// VIOLATION: Server component using client hook
import { useExtracted } from "@/i18n/client";

export default function ServerComponent() {
  const t = useExtracted("error");
  return <div>{t("This is wrong")}</div>;
}
