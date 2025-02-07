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
    const tooltipRef = useRef<HTMLDivElement | null>(null);
    const wrapperRef = useRef<HTMLDivElement | null>(null); // For fullscreen sizing

    const [dimensions, setDimensions] = useState({
        width: typeof window !== "undefined" ? window.innerWidth : 1920, // Fallback width
        height: typeof window !== "undefined" ? window.innerHeight : 1080 // Fallback height
    });

    useEffect(() => {
        if (!svgRef.current || !tooltipRef.current || !wrapperRef.current) return;

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
                    .id(d => (d as NodeData).id) // Ensure `d.id` is read properly
                    .distance(100)
            )
            .force("charge", d3.forceManyBody().strength(-300))
            .force("center", d3.forceCenter(dimensions.width / 2, dimensions.height / 2));


        const svg = d3
            .select(svgRef.current)
            .attr("width", width)
            .attr("height", height)
            .attr("viewBox", `0 0 ${width} ${height}`) // Makes it responsive
            .attr("preserveAspectRatio", "xMidYMid meet");

        svg.selectAll("*").remove(); // Clear previous renders

        // Zoom & Pan
        const zoom = d3.zoom<SVGSVGElement, unknown>()
            .scaleExtent([0.5, 4]) // Zoom limits (min 50%, max 400%)
            .on("zoom", (event) => graphGroup.attr("transform", event.transform));

        svg.call(zoom);

        // Graph container (for zooming)
        const graphGroup = svg.append("g");

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
            .on("mouseover", (event, d) => showTooltip(event, d))
            .on("mouseout", hideTooltip)
            .attr("cursor", "pointer");

        // Node Labels
        graphGroup
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
            .text(d => d.id);

            function ticked() {
                link
                    .attr("x1", d => ((d.source as NodeData).x !== undefined ? (d.source as NodeData).x! : 0))
                    .attr("y1", d => ((d.source as NodeData).y !== undefined ? (d.source as NodeData).y! : 0))
                    .attr("x2", d => ((d.target as NodeData).x !== undefined ? (d.target as NodeData).x! : 0))
                    .attr("y2", d => ((d.target as NodeData).y !== undefined ? (d.target as NodeData).y! : 0));
                node.attr("cx", d => (d.x !== undefined ? d.x : 0)).attr("cy", d => (d.y !== undefined ? d.y : 0));
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

        function showTooltip(event: React.MouseEvent<SVGCircleElement, MouseEvent>, d: NodeData) {
            d3.select(tooltipRef.current)
                .style("left", `${event.pageX + 10}px`)
                .style("top", `${event.pageY - 20}px`)
                .style("opacity", 1)
                .style("z-index", "1")
                .style("background", "white")
                .style("padding", "1rem")
                .style("border-radius", "4px")
                .style("box-shadow", "0px 0px 10px rgba(0, 0, 0, 0.1)")
                .style("color", "black")
                .html(`<strong>${d.id}</strong><br>${d.details}`);
        }


        function hideTooltip() {
            d3.select(tooltipRef.current).style("opacity", 0);
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

    return (
        <div ref={wrapperRef} style={{ width: "100vw", height: "100vh", overflow: "hidden", position: "relative" }}>
            <svg ref={svgRef}></svg>
            <div
                ref={tooltipRef}
                style={{
                    position: "absolute",
                    background: "white",
                    padding: "8px",
                    borderRadius: "4px",
                    boxShadow: "0px 0px 10px rgba(0, 0, 0, 0.1)",
                    opacity: 0,
                    pointerEvents: "none"
                }}
            ></div>
        </div>
    );
};


