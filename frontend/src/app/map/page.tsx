"use client";

import { useState } from "react";
import Link from "next/link";

interface Topic {
  title: string;
  subcategories: { title: string; link: string }[];
}

const topics: Topic[] = [
  {
    title: "Algebra",
    subcategories: [
      { title: "Linear Algebra", link: "/map/algebra/linear" },
      { title: "Abstract Algebra", link: "/map/algebra/abstract" },
    ],
  },
  {
    title: "Geometry",
    subcategories: [
      { title: "Euclidean Geometry", link: "/map/geometry/euclidean" },
      { title: "Non-Euclidean Geometry", link: "/map/geometry/non-euclidean" },
    ],
  },
  {
    title: "Calculus",
    subcategories: [
      { title: "Differential Calculus", link: "/map/calculus/differential" },
      { title: "Integral Calculus", link: "/map/calculus/integral" },
    ],
  },
  {
    title: "Computer Science",
    subcategories: [
      { title: "Data Structures", link: "/map/cs/datastructures" },
      { title: "Algorithms", link: "/map/cs/algorithms" },
      { title: "Operating Systems", link: "/map/cs/os" },
    ],
  },
];

export default function Home() {
  const [expandedTopics, setExpandedTopics] = useState<{ [key: number]: boolean }>({});

  const toggleTopic = (index: number) => {
    setExpandedTopics((prev) => ({
      ...prev,
      [index]: !prev[index],
    }));
  };

  return (
    <div className="h-screen bg-gray-900 text-white transition-colors duration-300 overflow-y-auto">
      {/* Constant Dark-themed Navbar */}
      <nav className="bg-gray-800 text-white p-4 flex justify-between items-center">
        <Link href="/" className="text-lg font-bold">
          Home
        </Link>
      </nav>

      <div className="max-w-5xl mx-auto p-4">
        {/* Hero Section */}
        <section className="flex flex-col md:flex-row items-center bg-gray-800 text-white p-8 rounded-lg shadow-lg">
          <div className="flex-1 mb-4 md:mb-0">
            <h1 className="text-4xl font-bold mb-4">Mind Map Lists</h1>
            <p>
              All of the topics are listed here. Click on a topic to see its subcategories.
            </p>
          </div>
          <div className="flex-1">
            <img
              src="https://image.alwaysdumb.com/map/hero.png/800"
              alt="Mind Map"
              className="rounded-lg"
            />
          </div>
        </section>

        {/* Topics List */}
        <section className="mt-8">
          <h2 className="text-2xl font-semibold mb-4">Topics</h2>
          <div className="space-y-4">
            {topics.map((topic, index) => (
              <div key={index} className="bg-gray-800 p-4 rounded shadow">
                <button
                  onClick={() => toggleTopic(index)}
                  className="w-full text-left text-xl font-semibold"
                >
                  {topic.title}
                </button>
                {expandedTopics[index] && (
                  <ul className="mt-2 ml-4 space-y-2">
                    {topic.subcategories.map((sub, subIndex) => (
                      <li key={subIndex}>
                        <Link
                          href={sub.link}
                          className="text-blue-400 hover:underline"
                        >
                          {sub.title}
                        </Link>
                      </li>
                    ))}
                  </ul>
                )}
              </div>
            ))}
          </div>
        </section>
      </div>
    </div>
  );
}
