import { log } from "horizon:extension/logger";
// import { readCollection } from "horizon:extension/blobs"
import { Document } from "horizon:extension/network";

export async function handle(input) {
  const z = new Document();
  console.log(z.readKey(""));

  return "string";
}
