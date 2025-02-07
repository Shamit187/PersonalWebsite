import type { Metadata } from "next";
import "./map.css";

export const metadata: Metadata = {
    title: "Topic Map",
    description: "A visual representation of mathematical topics and their relationships.",
};

export default function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <main>{children}</main>
    );
}