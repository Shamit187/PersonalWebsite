import type { Metadata } from "next";
// import { AR_One_Sans } from "next/font/google"
import "./globals.css";

// const arOneSans = AR_One_Sans({
//     variable: "--font-ar-one",
//     subsets: ["latin"],
// })

export const metadata: Metadata = {
    title: "Pomodoro Timer",
    description: "A simple Pomodoro timer to manage your work and break sessions.",
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