import { log } from "horizon:extension/logger";
// import { readCollection } from "horizon:extension/blobs"
import { Document } from "horizon:extension/network";

export async function handle(input) {
  const z = new Document();
  const { read, write, documentId } = await z.create();
  console.table({ read, write, documentId });
  let mz = z.load(read);
  console.log(mz);
  const str = "world!";
  const encoder = new TextEncoder();
  const uint8Array = encoder.encode(str);
  z.addKeyValue(mz.documentId, "helo", uint8Array);
  const xs = z.readKey(mz.documentId, "helo");
  const decoder = new TextDecoder();
  const str2 = decoder.decode(xs);
  console.log(str2);

  return "string";
}
