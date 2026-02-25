"use client";

// VIOLATION: Client component importing from server
import { createClient } from "@/lib/supabase/server";

export function WrongClientImport() {
  const supabase = createClient();
  return <div>Wrong Import</div>;
}
