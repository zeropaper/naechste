// VIOLATION: API route not in api/ directory
export async function GET() {
  return Response.json({ message: "Hello" });
}
