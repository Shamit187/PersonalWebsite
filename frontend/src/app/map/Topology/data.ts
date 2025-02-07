import { NodeData, LinkData } from "../TopicGraph";
const nodes: NodeData[] = [
    { id: "Topology", group: 1, details: "The study of properties preserved through deformations, twistings, and stretchings of objects." },
    { id: "Point-Set Topology", group: 2, details: "Deals with the foundational aspects of topology using set theory." },
    { id: "Algebraic Topology", group: 2, details: "Uses algebraic structures to study topological spaces." },
    { id: "Geometric Topology", group: 2, details: "Focuses on the study of manifolds and their embeddings." },
    { id: "Continuity", group: 3, details: "A function is continuous if the preimage of an open set is open." },
    { id: "Compactness", group: 3, details: "A space is compact if every open cover has a finite subcover." },
    { id: "Connectedness", group: 3, details: "A space is connected if it cannot be divided into two disjoint non-empty open sets." },
    { id: "Homotopy", group: 4, details: "A continuous deformation from one function to another." },
    { id: "Fundamental Group", group: 4, details: "Captures information about loops in a space." },
    { id: "Manifolds", group: 4, details: "A topological space that locally resembles Euclidean space." },
    { id: "Knot Theory", group: 5, details: "The study of mathematical knots." }
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
    { source: "Geometric Topology", target: "Knot Theory", value: 1 }
];
export { nodes, links };
