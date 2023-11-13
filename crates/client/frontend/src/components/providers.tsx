"use client";

import createApolloClient from "@/lib/apollo-client";
import { ApolloProvider } from "@apollo/client";
import { NextUIProvider } from "@nextui-org/react";

export default function Providers({ children }: { children: React.ReactNode }) {
    const client = createApolloClient();

    return (
        <NextUIProvider>
            <ApolloProvider client={client}>{children}</ApolloProvider>
        </NextUIProvider>
    );
}
