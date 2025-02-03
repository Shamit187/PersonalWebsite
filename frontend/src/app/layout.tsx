import type { Metadata } from "next";
import {AR_One_Sans} from "next/font/google"
import "./globals.css";

const arOneSans = AR_One_Sans({
  variable: "--font-ar-one",
  subsets: ["latin"],
})

export const metadata: Metadata = {
  title: "Shamit - Researcher in Computer Systems and AI",
  description: "Shamit Fatin is a researcher specializing in computer systems, AI-driven automation, and software testing. Explore insights on computer architecture, operating systems, LLM-powered automation, and benchmarking.",
  keywords: [
    "Shamit",
    "Shamit Fatin",
    "computer systems research",
    "operating systems",
    "LLM for automation",
    "AI for software testing",
    "Compute Express Link (CXL)",
    "GPU research",
    "systems research",
    "PhD in computer science",
    "computer science blog",
    "web UI automation",
    "AI-driven software testing",
    "benchmarking in computer systems"
  ],
  openGraph: {
    title: "Shamit - Researcher in Computer Systems and AI",
    description: "Explore Shamit's research on computer systems, operating systems, AI-powered automation, and software testing. Read technical blogs, research papers, and insights.",
    url: "https://alwaysdumb.com",
    type: "website",
    images: [
      {
        url: "https://image.alwaysdumb.com/Me.jpg/1920",
        width: 1200,
        height: 630,
        alt: "Shamit - Computer Systems and AI Researcher"
      }
    ]
  },
  twitter: {
    card: "summary_large_image",
    site: "@CuriousShamit",
    title: "Shamit - Researcher in Computer Systems and AI",
    description: "Discover Shamit's work in computer systems, AI, and automation research.",
    images: [
      {
        url: "https://image.alwaysdumb.com/Me.jpg/1920",
        width: 1200,
        height: 630,
        alt: "Shamit - Computer Systems and AI Researcher"
      }
    ]
  }
};



export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={`${arOneSans.variable} antialiased`}
      >
        {children}
      </body>
    </html>
  );
}
