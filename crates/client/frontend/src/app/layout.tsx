import type { Metadata } from "next";
import { Inter } from "next/font/google";
import ApolloProvider from "@/components/apollo-provider";
import "@/styles/globals.css";

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
    if (typeof window !== "undefined") {
        window.onload = () => {
            document.getElementById("holderStyle")?.remove();
        };
    }

    return (
        <html lang="en">
            <body className={inter.className}>
                <ApolloProvider>{children}</ApolloProvider>
            </body>
        </html>
    );
}
