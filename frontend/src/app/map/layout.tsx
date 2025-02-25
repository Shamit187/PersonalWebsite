import "./map.css";
import MathJaxClientLayout from "./MathJaxClientLayout";
import type { Metadata } from "next";

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
            <MathJaxClientLayout>
            {children}
            </MathJaxClientLayout>
    );
}
