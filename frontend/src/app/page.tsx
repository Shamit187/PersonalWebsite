"use client";

import React, { useState, useEffect } from "react";
import Link from "next/link";

function randomGaussian(mean: number, stdDev: number): number {
  const u1 = Math.random();
  const u2 = Math.random();
  const z0 = Math.sqrt(-2.0 * Math.log(u1)) * Math.cos(2.0 * Math.PI * u2);
  return z0 * stdDev + mean;
}

type ChaosDiv = {
  id: number;
  x: number;
  y: number;
  content: string;
  link: string;
  styleclass: string;
  rotation: number;
  scale: number;
};

function distance(x1: number, y1: number, x2: number, y2: number): number {
  return Math.sqrt((x1 - x2) ** 2 + (y1 - y2) ** 2);
}

function isParallel(x1: number, y1: number, x2: number, y2: number, z: number): boolean {
  return (x1 - x2) ** 2 < z ** 2 || (y1 - y2) ** 2 < z ** 2;
}

function generateRotation(): number {
  return -30 + Math.random() * 60; // Random value between 0 and 45
}

function generateScale(): number {
  return 0.8 + Math.random() * 0.6; // Random value between 0.9 and 1.1
}

interface WindowSize {
  width: number | undefined;
  height: number | undefined;
}

function useWindowSize(): WindowSize {
  const [windowSize, setWindowSize] = useState<WindowSize>({
    width: undefined,
    height: undefined,
  });

  useEffect(() => {
    function handleResize() {
      setWindowSize({
        width: window.innerWidth,
        height: window.innerHeight,
      });
    }

    window.addEventListener("resize", handleResize);
    handleResize();
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  return windowSize;
}

export default function Home() {
  const { width } = useWindowSize();
  const isMobile = width !== undefined && width <= 768;

  const [chaosDivs, setChaosDivs] = useState<ChaosDiv[]>([]);

  useEffect(() => {
    const contentArray = [
      { text: "About Me", link: "/#" },
      { text: "Blog", link: "/#" },
      { text: "Art Gallery", link: "/#" },
      { text: "Game Dev Journal", link: "/#" },
      { text: "Email Me", link: "/#" },
      { text: "Portfolio", link: "/#" },
      { text: "Resume", link: "/#" },
      { text: "Projects", link: "/#" },
      { text: "Photography", link: "/#" },
      { text: "Contact", link: "/#" },
      { text: "Testimonials", link: "/#" },
      { text: "FAQ", link: "/#" },
      { text: "Team", link: "/#" },
      { text: "Careers", link: "/#" },
      { text: "Community", link: "/#" },
      { text: "Events", link: "/#" },
      { text: "News", link: "/#" },
      { text: "Resources", link: "/#" },
      { text: "Help Center", link: "/#" },
      { text: "Settings", link: "/#" },
    ];
    const buttonClasses = [
      "btn-blue",
      "btn-green",
      "btn-cyan",
      "btn-teal",
      "btn-lime",
      "btn-red",
      "btn-pink",
      "btn-purple",
    ];

    const minDistance = 8;
    const parallelThreshold = 0;
    const generatedDivs: ChaosDiv[] = [];

    function generateNonOverlappingPosition(
      chaosDivs: ChaosDiv[],
      content: string,
      link: string,
      id: number,
      minDistance: number,
      parallelThreshold: number,
      styleclass: string
    ): ChaosDiv {
      const meanX = isMobile ? 50 : 75; // Center x for mobile
      const meanY = 75; // Center y
      const dev = 5;
      let iteration_dev = dev;

      while (true) {
        const x = Math.max(isMobile ? 0 : 25 , Math.min(90, randomGaussian(meanX, iteration_dev)));
        const y = Math.max(isMobile ? 25 : 10, Math.min(90, randomGaussian(meanY, iteration_dev)));
        const rotation = generateRotation();
        const scale = generateScale();

        const isFarEnough = chaosDivs.every(
          (div) =>
            distance(x, y, div.x, div.y) >= minDistance &&
            !isParallel(x, y, div.x, div.y, parallelThreshold)
        );

        if (isFarEnough) {
          return { id, x, y, content, link, styleclass, rotation, scale };
        }

        iteration_dev += 5;
      }
    }

    contentArray.forEach((content, i) => {
      const randomStyleClass = buttonClasses[Math.floor(Math.random() * buttonClasses.length)];
      const newDiv = generateNonOverlappingPosition(
        generatedDivs,
        content.text,
        content.link,
        i,
        minDistance,
        parallelThreshold,
        randomStyleClass
      );
      generatedDivs.push(newDiv);
    });

    setChaosDivs(generatedDivs);
  }, [isMobile]);

  return (
    <div className="h-screen w-screen relative p-10">
      <div className="flex flex-col items-start">
        <div className="title-font">I was too tired</div>
        <div className="base-font pl-0 lg:pl-2">I threw some buttons here... Pick one</div>
      </div>
      {chaosDivs.map((div) => (
        <Link
          key={div.id}
          className="absolute"
          style={{
            top: `${div.y}%`,
            left: `${div.x}%`,
            transform: `rotate(${div.rotation}deg) scale(${div.scale})`,
          }}
          href={div.link}
        >
          <div className="outer-button">
            <div className={`${div.styleclass}`} />
            <div className="button-content">{div.content}</div>
          </div>
        </Link>
      ))}
    </div>
  );
}
