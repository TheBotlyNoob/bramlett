import type { Metadata } from "next";
import { Inter } from "next/font/google";
import ApolloProvider from "@/components/apollo-provider";
import "@/styles/globals.css";
import { ConfigProvider, theme } from "antd";

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
    return (
        <html lang="en">
            <body className={inter.className}>
                <ConfigProvider
                    theme={{
                        algorithm: theme.darkAlgorithm,
                    }}
                >
                    <ApolloProvider>{children}</ApolloProvider>
                </ConfigProvider>
            </body>
        </html>
    );
}
