import { serve } from "https://deno.land/std@0.188.0/http/server.ts";
import { serveFile } from "https://deno.land/std@0.188.0/http/file_server.ts";

const handler = async (request: Request): Promise<Response> => {
    return await serveFile(request, `${Deno.cwd()}` + new URL(request.url).pathname);
};

console.log("Server listening on http://localhost:8080");
await serve(handler, { port: 8080 });