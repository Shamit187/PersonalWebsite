import { NodeData, LinkData } from "../TopicGraph";

const nodes: NodeData[] = [
    {
        id: "Topology",
        group: 1,
        details: `
            <div class="full-content">
                <h1 class="title-content">Topology</h1>
                <p class="p-content">The study of properties preserved through <strong>deformations, twistings,</strong> and <em>stretchings</em> of objects.</p>
                <ul class="list-disc ml-5">
                    <li>Studies the nature of space and continuity</li>
                    <li>Fundamental in modern mathematics</li>
                </ul>
            </div>
        `,
    },
    {
        id: "Point-Set Topology",
        group: 2,
        details: `
            <div class="full-content">
                <h3 class="title-content">Point-Set Topology</h3>
                <p class="p-content">Deals with the <span class="text-blue-500">foundational aspects</span> of topology using set theory.</p>
                <blockquote class="border-l-4 border-gray-500 pl-3 italic">
                    "A topological space is a set endowed with a topology."
                </blockquote>
            </div>
        `,
    },
    {
        id: "Algebraic Topology",
        group: 2,
        details: `
            <div class="full-content">
                <h3 class="title-content">Algebraic Topology</h3>
                <p class="p-content">Uses algebraic structures to study topological spaces.</p>
                <code>π_1(X) = { Homotopy classes of loops in X }</code>
            </div>
        `,
    },
    {
        id: "Geometric Topology",
        group: 2,
        details: `
            <div class="full-content">
                <h3 class="title-content">Geometric Topology</h3>
                <p class="p-content">Focuses on the study of <b>manifolds</b> and their embeddings.</p>
                <ul class="list-disc ml-5">
                    <li>Studies low-dimensional manifolds</li>
                    <li>Includes <span class="text-red-500">knot theory</span></li>
                </ul>
            </div>
        `,
    },
    {
        id: "Continuity",
        group: 3,
        details: `
            <div class="full-content">
                <h3 class="title-content">Continuity</h3>
                <p class="p-content">A function <code>f: X → Y</code> is continuous if:</p>
                <ul class="list-disc ml-5">
                    <li>The preimage of every open set is open.</li>
                    <li>Intuitively means "no jumps" in the function.</li>
                </ul>
            </div>
        `,
    },
    {
        id: "Compactness",
        group: 3,
        details: `
            <div class="full-content">
                <h3 class="title-content">Compactness</h3>
                <p class="p-content">A space is compact if every open cover has a finite subcover.</p>
                <p class="text-sm text-gray-500">Examples:</p>
                <ul class="list-disc ml-5">
                    <li>The closed interval <code>[0,1]</code> in <code>ℝ</code></li>
                    <li>The unit sphere <code>S²</code> in <code>ℝ³</code></li>
                </ul>
            </div>
        `,
    },
    {
        id: "Connectedness",
        group: 3,
        details: `
            <div class="full-content">
                <h3 class="title-content">Connectedness</h3>
                <p class="p-content">A space is connected if it cannot be divided into two disjoint non-empty open sets.</p>
                <ul class="list-disc ml-5">
                    <li>Intervals in <code>ℝ</code> are connected.</li>
                    <li>The Cantor set is totally disconnected.</li>
                </ul>
            </div>
        `,
    },
    {
        id: "Homotopy",
        group: 4,
        details: `
            <div class="full-content">
                <h3 class="title-content">Homotopy</h3>
                <p class="p-content">A continuous deformation from one function to another.</p>
                <p class="text-gray-600 text-sm">Mathematically, two functions <code>f,g: X → Y</code> are homotopic if there exists a continuous map:</p>
                <code>H: X × [0,1] → Y</code>
            </div>
        `,
    },
    {
        id: "Fundamental Group",
        group: 4,
        details: `
            <div class="full-content">
                <h3 class="title-content">Fundamental Group</h3>
                <p class="p-content">Captures information about loops in a space.</p>
                <p class="p-content">The fundamental group of a circle <code>S¹</code> is:</p>
                <code>π_1(S¹) ≈ ℤ</code>
            </div>
        `,
    },
    {
        id: "Manifolds",
        group: 4,
        details: `
            <div class="full-content">
                <h3 class="title-content">Manifolds</h3>
                <p class="p-content">A topological space that locally resembles Euclidean space.</p>
                <p class="text-sm text-gray-500">Example:</p>
                <ul class="list-disc ml-5">
                    <li>The surface of a sphere</li>
                    <li>The torus <code>T²</code></li>
                </ul>
            </div>
        `,
    },
    {
        id: "Knot Theory",
        group: 5,
        details: `
            <div class="full-content">
                <h3 class="title-content">Knot Theory</h3>
                <p class="p-content">The study of mathematical knots.</p>
                <p class="p-content">Knot invariants include:</p>
                <ul class="list-disc ml-5">
                    <li>Jones polynomial</li>
                    <li>Alexander polynomial</li>
                </ul>
            </div>
        `,
    },
];

const links: LinkData[] = [
    { source: "Topology", target: "Point-Set Topology", value: 1 },
    { source: "Topology", target: "Algebraic Topology", value: 1 },
    { source: "Topology", target: "Geometric Topology", value: 1 },
    { source: "Point-Set Topology", target: "Continuity", value: 1 },
    { source: "Point-Set Topology", target: "Compactness", value: 1 },
    { source: "Point-Set Topology", target: "Connectedness", value: 1 },
    { source: "Algebraic Topology", target: "Homotopy", value: 1 },
    { source: "Algebraic Topology", target: "Fundamental Group", value: 1 },
    { source: "Geometric Topology", target: "Manifolds", value: 1 },
    { source: "Geometric Topology", target: "Knot Theory", value: 1 },
];

export { nodes, links };
