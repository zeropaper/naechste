// VIOLATION: Cross-domain import from sibling domain's lib/
import { getProfiles } from "@/app/(admin)/lib/profiles";

export async function loadProfiles() {
  return await getProfiles();
}
