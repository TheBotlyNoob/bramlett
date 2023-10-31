import type { Metadata } from "next";
import { Inter } from "next/font/google";
import { ApolloProvider } from "@apollo/client";
import createApolloClient from "@/lib/apollo-client";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
    title: "Bramlett's Game Launcher",
    description: "A totally awesome game launcher.",
};

export default function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    const client = createApolloClient();

    return (
        <html lang="en">
            <body className={inter.className}>
                <ApolloProvider client={client}>{children}</ApolloProvider>
            </body>
        </html>
    );
}
