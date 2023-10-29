"use client";

import { wsLink, initRspc, httpLink } from "@rspc/client";
import { createReactQueryHooks } from "@rspc/react-query";
import { QueryClient } from "@tanstack/react-query";
import type { Procedures } from "./bindings";

export const client = initRspc<Procedures>({
    links: [
        // wsLink({
        //     url: "ws://localhost:4000/rspc/ws"
        // })
        httpLink({
            url: "http://localhost:4000/rspc"
        })
    ]
});

export const queryClient = new QueryClient({
    defaultOptions: {
        queries: {
            retry: false // If you want to retry when requests fail, remove this.
        }
    }
});

export const rspc = createReactQueryHooks<Procedures>();
