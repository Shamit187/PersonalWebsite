"use client";

import * as d3 from "d3";
import { useEffect, useRef } from "react";

const LineChart = () => {
    const svgRef = useRef<SVGSVGElement | null>(null);

    useEffect(() => {
        if (!svgRef.current) return;

        // Sample data
        const data = [
            { x: 0, y: 10 },
            { x: 1, y: 30 },
            { x: 2, y: 20 },
            { x: 3, y: 40 },
            { x: 4, y: 35 },
            { x: 5, y: 50 }
        ];

        const width = 400;
        const height = 300;
        const margin = { top: 20, right: 20, bottom: 40, left: 50 };

        const svg = d3.select(svgRef.current)
            .attr("width", width)
            .attr("height", height)
            .style("background", "#f9f9f9")
            .style("overflow", "visible");

        // Scales
        const xScale = d3.scaleLinear()
            .domain([0, d3.max(data, d => d.x)!])
            .range([margin.left, width - margin.right]);

        const yScale = d3.scaleLinear()
            .domain([0, d3.max(data, d => d.y)!])
            .range([height - margin.bottom, margin.top]);

        // Line generator
        const line = d3.line<{ x: number; y: number }>()
            .x(d => xScale(d.x))
            .y(d => yScale(d.y))
            .curve(d3.curveMonotoneX);

        // Append path
        svg.selectAll("*").remove(); // Clear previous drawings
        svg.append("path")
            .datum(data)
            .attr("fill", "none")
            .attr("stroke", "steelblue")
            .attr("stroke-width", 2)
            .attr("d", line);

        // Axes
        svg.append("g")
            .attr("transform", `translate(0,${height - margin.bottom})`)
            .call(d3.axisBottom(xScale).ticks(5));

        svg.append("g")
            .attr("transform", `translate(${margin.left},0)`)
            .call(d3.axisLeft(yScale).ticks(5));

    }, []);

    return <svg ref={svgRef}></svg>;
};

export default LineChart;
