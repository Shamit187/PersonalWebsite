"use client";

import { TopicGraph } from "../TopicGraph";
import { nodes, links } from "./data";

export default function Home() {
    return <TopicGraph nodeList={nodes} edgeList={links} />;
}