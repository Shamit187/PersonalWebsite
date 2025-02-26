"use client";

import { MathJax } from "better-react-mathjax";
import * as d3 from "d3";
import { useEffect, useRef, useState } from "react";

interface TopicGraphProps {
    nodeList: NodeData[];
    edgeList: LinkData[];
}

export interface SingleDetail {
    type: "text" | "math" | "title";
    content: string;
}

export interface NodeData extends d3.SimulationNodeDatum {
    id: string;
    group: number;
    title: string;
    content: string;
    image: string;
}

export interface LinkData {
    source: string | NodeData;
    target: string | NodeData;
    value: number;
}

export const TopicGraph = ({ nodeList, edgeList }: TopicGraphProps) => {
    const svgRef = useRef<SVGSVGElement | null>(null);
    // const wrapperRef = useRef<HTMLDivElement | null>(null); // For fullscreen sizing

    const [selectedNode, setSelectedNode] = useState<NodeData | null>(null); // Store hovered node details
    const zoomRef = useRef<d3.ZoomBehavior<SVGSVGElement, unknown> | null>(null);
    const initialTransform = useRef<d3.ZoomTransform | null>(null);

    const [dimensions, setDimensions] = useState({
        width: typeof window !== "undefined" ? (window.innerWidth > 1024 ? window.innerWidth * 0.75 : window.innerWidth) : 1920, // pc has 3/4 th of the width allocated, while mobile has full width
        height: typeof window !== "undefined" ? window.innerHeight : 1080
    });

    useEffect(() => {
        // if (!svgRef.current || !wrapperRef.current) return;
        if (!svgRef.current) return

        const { width, height } = dimensions;
        // const color = d3.scaleSequential(d3.interpolateTurbo).domain([0, 1]);
        const custom_color = [
            "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd", "#8c564b",
            "#e377c2", "#7f7f7f", "#bcbd22", "#17becf", "#a52a2a", "#ff4500",
            "#32cd32", "#ff1493", "#00ced1", "#ff6347", "#8b4513", "#4682b4",
            "#228b22", "#ffd700", "#ff00ff", "#00ff00", "#dc143c", "#00008b",
            "#9932cc", "#ff8c00", "#008000", "#ff69b4", "#800080", "#008b8b"
        ];        
        const color = d3.scaleOrdinal(custom_color);
        const max_color = 30;

        const links: LinkData[] = edgeList.map(d => ({ ...d }));
        const nodes: NodeData[] = nodeList.map(d => ({ ...d }));

        const simulation = d3
            .forceSimulation<NodeData>(nodes)
            .force(
                "link",
                d3
                    .forceLink<NodeData, LinkData>(links)
                    .id(d => (d as NodeData).id)
                    .distance(100)
            )
            .force("charge", d3.forceManyBody().strength(-300))
            .force("center", d3.forceCenter(dimensions.width / 2, dimensions.height / 2));

        const svg = d3
            .select(svgRef.current)
            .attr("width", width)
            .attr("height", height)
            .attr("viewBox", `0 0 ${width} ${height}`)
            .attr("preserveAspectRatio", "xMidYMid meet");

        svg.selectAll("*").remove(); // Clear previous renders

        const graphGroup = svg.append("g");

        // Add zoom capabilities
        const zoom = d3.zoom<SVGSVGElement, unknown>()
            .scaleExtent([0.1, 8])
            .on("zoom", event => {
                graphGroup.attr("transform", event.transform);
            });

        svg.call(zoom);
        if (!initialTransform.current) {
            initialTransform.current = d3.zoomIdentity;
        }

        zoomRef.current = zoom; // Store the zoom instance

        // Links
        const link = graphGroup
            .append("g")
            .attr("stroke", "#999")
            .attr("stroke-opacity", 0.6)
            .selectAll("line")
            .data(links)
            .join("line")
            .attr("stroke-width", d => Math.sqrt(d.value));

        // Nodes
        const node = graphGroup
            .append("g")
            .attr("stroke", "#fff")
            .attr("stroke-width", 1.5)
            .selectAll<SVGCircleElement, NodeData>("circle")
            .data(nodes)
            .join("circle")
            .attr("r", 8)
            .attr("fill", d => color(String(d.group % max_color)))
            .call(
                d3
                    .drag<SVGCircleElement, NodeData>()
                    .on("start", dragstarted)
                    .on("drag", dragged)
                    .on("end", dragended)
            )
            .on("mouseover", (event, d) => handleNodeHover(d))
            .attr("cursor", "pointer");

        // Define the text selection
        const text = graphGroup
            .append("g")
            .selectAll("text")
            .data(nodes)
            .join("text")
            .attr("x", d => (d.x !== undefined ? d.x : 0))
            .attr("y", d => (d.y !== undefined ? d.y : 0))
            .attr("dy", -10)
            .attr("text-anchor", "middle")
            .attr("fill", "#777")
            .attr("font-size", width < 1024 ? "0.5rem" : "0.75rem")
            .attr("opacity", 0) // Hide text initially
            .text(d => d.id);

        function ticked() {
            link
                .attr("x1", d => ((d.source as NodeData).x ?? 0))
                .attr("y1", d => ((d.source as NodeData).y ?? 0))
                .attr("x2", d => ((d.target as NodeData).x ?? 0))
                .attr("y2", d => ((d.target as NodeData).y ?? 0));

            node
                .attr("cx", d => (d.x !== undefined ? d.x : 0))
                .attr("cy", d => (d.y !== undefined ? d.y : 0));

            text
                .attr("x", d => (d.x !== undefined ? d.x : 0))
                .attr("y", d => (d.y !== undefined ? d.y : 0))
                .attr("opacity", 1); // Show text


        }

        function dragstarted(event: d3.D3DragEvent<SVGCircleElement, NodeData, NodeData>) {
            if (!event.active) simulation.alphaTarget(0.3).restart();
            event.subject.fx = event.subject.x;
            event.subject.fy = event.subject.y;
        }

        function dragged(event: d3.D3DragEvent<SVGCircleElement, NodeData, NodeData>) {
            event.subject.fx = event.x;
            event.subject.fy = event.y;
        }

        function dragended(event: d3.D3DragEvent<SVGCircleElement, NodeData, NodeData>) {
            if (!event.active) simulation.alphaTarget(0);
            event.subject.fx = null;
            event.subject.fy = null;
        }

        function handleNodeHover(d: NodeData) {
            setSelectedNode(d);
        }

        simulation.on("tick", ticked);

        // Handle window resizing
        function handleResize() {
            setDimensions({
                width: window.innerWidth,
                height: window.innerHeight
            });
        }

        // add legends
        const legendGroup = svg.append("g")
            .attr("class", "legend")
            .attr("transform", `translate(50, 50)`);
        const uniqueGroups = Array.from(new Set(nodes.map(d => d.group)));
        legendGroup.selectAll("rect")
            .data(uniqueGroups)
            .join("rect")
            .attr("x", 0)
            .attr("y", (d, i) => i * 25)
            .attr("width", 15)
            .attr("height", 15)
            .attr("fill", d => color(String(d % max_color)));
        legendGroup.selectAll("text")
            .data(uniqueGroups)
            .join("text")
            .attr("x", 20)
            .attr("y", (d, i) => i * 25 + 12)
            .attr("fill", "#777")
            .attr("font-size", width < 1024 ? "0.5rem" : "0.75rem")
            .text(d => `Topic Depth Level ${d}`);


        window.addEventListener("resize", handleResize);
        return () => window.removeEventListener("resize", handleResize);
    }, [nodeList, edgeList, dimensions]);


    return (
        <div className="w-screen h-screen flex flex-col md:flex-row">
            <div className="md:w-3/4 w-full h-2/3 md:h-screen overflow-hidden relative">
                <svg ref={svgRef}></svg>
            </div>
            <div className="md:w-1/4 w-full md:h-screen h-1/3 overflow-y-auto px-4 py-10 md:py-10 no-scrollbar bg-neutral-900">
                {selectedNode ? (
                    <div key={selectedNode?.id} className="flex flex-col items-start space-y-4">
                        <h3 className="text-lg font-bold">{selectedNode?.title}</h3>
                        <div className="flex flex-col space-y-2">
                            {selectedNode?.content.split("<br>").map((paragraph, index) => (
                                <div key={index}><MathJax>{paragraph}</MathJax></div>
                            ))}
                        </div>
                        {selectedNode?.image && (
                            <img src={selectedNode?.image} alt={selectedNode?.id} className="rounded-lg" />
                        )}
                    </div>
                ) : (
                    <p>Hover over a node to see details</p>
                )}
            </div>
        </div>
    );

};
