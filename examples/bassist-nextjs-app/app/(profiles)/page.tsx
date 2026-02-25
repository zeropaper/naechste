// This violates bassist-locale-nesting - page.tsx not in [locale]/
import { createClient } from "@/lib/supabase/server";

export default async function ProfilePage() {
  const supabase = await createClient();

  return <div>Profile Page - WRONG LOCATION</div>;
}
