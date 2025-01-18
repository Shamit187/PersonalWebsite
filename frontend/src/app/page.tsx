import React from "react";
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
};

function distance(x1: number, y1: number, x2: number, y2: number): number {
  return Math.sqrt((x1 - x2) ** 2 + (y1 - y2) ** 2);
}

function isParallel(x1: number, y1: number, x2: number, y2: number, z: number): boolean {
  return (x1 - x2) ** 2 < z ** 2 || (y1 - y2) ** 2 < z ** 2;
}

function generateNonOverlappingPosition(
  chaosDivs: ChaosDiv[],
  content: string,
  link: string,
  id: number,
  minDistance: number,
  parallelThreshold: number,
  styleclass: string,
): ChaosDiv {
  const mean = 75;
  const dev = 10;
  let iteration_dev = dev;
  while (true) {
    const x = Math.max(25, Math.min(90, randomGaussian(mean, iteration_dev))); // Clamp to 25-90
    const y = Math.max(0, Math.min(90, randomGaussian(mean, iteration_dev))); // Clamp to 0-90

    // Check against all existing divs
    const isFarEnough = chaosDivs.every(
      (div) =>
        distance(x, y, div.x, div.y) >= minDistance && // Ensure minimum distance
        !isParallel(x, y, div.x, div.y, parallelThreshold) // Ensure not parallel
    );

    if (isFarEnough) {
      return { id, x, y, content, link, styleclass }; // Return valid position with style
    }

    iteration_dev += 5
  }
}

export default function Home() {
  const contentArray = [
    { text: "About Me", link: "/#" },
    { text: "Blog", link: "/#" },
    { text: "Art Gallary", link: "/#" },
    { text: "Game dev journal", link: "/#" },
    { text: "Email Me", link: "/#" },
    { text: "Dummy", link: "/#" },
    { text: "Dummy", link: "/#" },
    { text: "Dummy", link: "/#" },
    { text: "Dummy", link: "/#" },
    { text: "Dummy", link: "/#" },
    { text: "Dummy", link: "/#" },
    { text: "Dummy", link: "/#" },
    { text: "Dummy", link: "/#" },
    { text: "Dummy", link: "/#" },
    { text: "Dummy", link: "/#" },
    { text: "Dummy", link: "/#" },
    { text: "Dummy", link: "/#" },
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

  const minDistance = 5;
  const parallelThreshold = 0;
  const chaosDivs: ChaosDiv[] = [];

  contentArray.forEach((content, i) => {
    const randomStyleClass = buttonClasses[Math.floor(Math.random() * buttonClasses.length)];
    const newDiv = generateNonOverlappingPosition(
      chaosDivs,
      content.text,
      content.link,
      i,
      minDistance,
      parallelThreshold,
      randomStyleClass
    );
    chaosDivs.push(newDiv);
  });

  return (
    <div className="h-screen w-screen relative overflow-hidden p-10">
      {/* Title */}
      <div className="flex flex-col items-start">
        <div className="title-font">I was too tired</div>
        <div className="base-font">to organize the page</div>
        <div className="base-font">Pick one button yourself...</div>
      </div>
      {/* The Chaos */}
      {chaosDivs.map((div) => (
        <Link
          key={div.id}
          className={`absolute chaos-box ${div.styleclass}`}
          style={{
            top: `${div.y}%`,
            left: `${div.x}%`,
          }}
          href={div.link}
        >
          {div.content}
        </Link>
      ))}
    </div>
  );
}
