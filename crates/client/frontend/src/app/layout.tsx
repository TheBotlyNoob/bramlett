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
            <style
                id="holderStyle"
                dangerouslySetInnerHTML={{
                    __html: `
                            /* https://github.com/ant-design/ant-design/issues/16037#issuecomment-483140458 */
                            /* Not only antd, but also any other style if you want to use ssr. */
                            *, *::before, *::after {
                                transition: none!important;
                            }
                            `,
                }}
            />
            <body className={inter.className}>
                <ApolloProvider>{children}</ApolloProvider>
            </body>
        </html>
    );
}
