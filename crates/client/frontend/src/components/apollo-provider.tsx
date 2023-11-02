"use client";

import createApolloClient from "@/lib/apollo-client";
import { ApolloProvider as RealProvider } from "@apollo/client";

export default function ApolloProvider({
    children,
}: {
    children: React.ReactNode;
}) {
    const client = createApolloClient();

    return <RealProvider client={client}>{children}</RealProvider>;
}
