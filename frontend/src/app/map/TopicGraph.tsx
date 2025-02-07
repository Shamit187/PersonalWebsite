"use client";

import * as d3 from "d3";
import { useEffect, useRef, useState } from "react";

interface TopicGraphProps {
    nodeList: NodeData[];
    edgeList: LinkData[];
}

export interface NodeData extends d3.SimulationNodeDatum {
    id: string;
    group: number;
    details: string;
}

export interface LinkData {
    source: string | NodeData;
    target: string | NodeData;
    value: number;
}

export const TopicGraph = ({ nodeList, edgeList }: TopicGraphProps) => {
    const svgRef = useRef<SVGSVGElement | null>(null);
    const wrapperRef = useRef<HTMLDivElement | null>(null); // For fullscreen sizing

    const [selectedNode, setSelectedNode] = useState<NodeData | null>(null); // Store hovered node details
    const zoomRef = useRef<d3.ZoomBehavior<SVGSVGElement, unknown> | null>(null);
    const initialTransform = useRef<d3.ZoomTransform | null>(null);

    const [dimensions, setDimensions] = useState({
        width: typeof window !== "undefined" ? window.innerWidth : 1920, // Fallback width
        height: typeof window !== "undefined" ? window.innerHeight : 1080 // Fallback height
    });

    useEffect(() => {
        if (!svgRef.current || !wrapperRef.current) return;

        const { width, height } = dimensions;
        const color = d3.scaleOrdinal(d3.schemeCategory10);

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
            .attr("fill", d => color(d.group.toString()))
            .call(
                d3
                    .drag<SVGCircleElement, NodeData>()
                    .on("start", dragstarted)
                    .on("drag", dragged)
                    .on("end", dragended)
            )
            .on("mouseover", (event, d) => handleNodeHover(d))
            .on("mouseout", () => setSelectedNode(null))
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
            .attr("fill", "#333")
            .attr("font-size", "12px")
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

        window.addEventListener("resize", handleResize);
        return () => window.removeEventListener("resize", handleResize);
    }, [nodeList, edgeList, dimensions]);

    // function __resetZoom() {
    //     if (zoomRef.current && svgRef.current) {
    //         const svg = d3.select(svgRef.current);
    //         svg.transition().duration(750).call(
    //             zoomRef.current.transform,
    //             d3.zoomIdentity.translate(dimensions.width / 2, dimensions.height / 2).scale(1)
    //         );
    //     }
    // }

    return (
        <div className="w-screen h-screen flex flex-col md:flex-row">
            <div ref={wrapperRef} className="md:w-3/4 w-full h-3/4 md:h-screen overflow-hidden relative">
                <svg ref={svgRef}></svg>
            </div>
            <div className="md:w-1/4 w-full md:h-screen h-1/4 overflow-auto p-4">
                {selectedNode ? (
                    <div>
                        <div dangerouslySetInnerHTML={{ __html: selectedNode.details }} />
                    </div>
                ) : (
                    <p>Hover over a node to see details</p>
                )}
                {/* <button
                    onClick={resetZoom}
                    className="mt-4 p-2 bg-blue-500 text-white rounded hover:bg-blue-700 transition"
                >
                    Reset View
                </button> */}
            </div>
        </div>
    );
    
};
