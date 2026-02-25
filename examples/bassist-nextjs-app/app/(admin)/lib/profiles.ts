// VIOLATION: Service client in production code
import { createTestServiceClient } from "@/tests/utils/supabase-service";

export async function getProfiles() {
  const supabase = createTestServiceClient();
  const { data } = await supabase.from("profiles").select("*");
  return data;
}
