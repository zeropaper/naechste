import { getExtracted } from "@/i18n/server";

export default async function AdminLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const t = await getExtracted("admin.layout");

  return (
    <div>
      <h1>{t("Admin Dashboard")}</h1>
      {children}
    </div>
  );
}
