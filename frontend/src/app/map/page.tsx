"use client";

import { TopicGraph, NodeData, LinkData } from "./TopicGraph";

export default function Home() {
    const nodes: NodeData[] = [
        { id: "Topic 1", group: 1, details: "Details about Topic 1" },
        { id: "Topic 2", group: 2, details: "Details about Topic 2" },
        { id: "Topic 3", group: 2, details: "Details about Topic 3" },
        { id: "Topic 4", group: 3, details: "Details about Topic 4" },
        { id: "Topic 5", group: 3, details: "Details about Topic 5" }
    ];

    const links: LinkData[] = [
        { source: "Topic 1", target: "Topic 2", value: 1 },
        { source: "Topic 2", target: "Topic 3", value: 1 },
        { source: "Topic 3", target: "Topic 4", value: 1 },
        { source: "Topic 4", target: "Topic 5", value: 1 },
        { source: "Topic 5", target: "Topic 1", value: 1 }
    ];

    return (
        <div>
            <TopicGraph nodeList={nodes} edgeList={links} />
        </div>
    );
}
