import type { Metadata } from "next";
import { Inter } from "next/font/google";
import RspcWrapper from "@/components/rspc-wrapper";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
    title: "Bramlett's Game Launcher",
    description: "A totally awesome game launcher."
};

export default function RootLayout({
    children
}: {
    children: React.ReactNode;
}) {
    return (
        <html lang="en">
            <body className={inter.className}>
                <RspcWrapper>{children}</RspcWrapper>
            </body>
        </html>
    );
}
