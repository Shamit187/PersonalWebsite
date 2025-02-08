import { NodeData, LinkData } from "../TopicGraph";

const nodes: NodeData[] = [
    {
        id: "Primes of the form x^2 + ny^2",
        group: 1,
        details: [
            { type: "title", content: "Primes of the form x^2 + ny^2" },
            { type: "math", content: "Euler's proof: If \\( p = x^2 + n y^2\\), then \\(p \\equiv 1 \\mod 4\\) follows from congruences." },
            { type: "math", content: "Descent Step: If \\( p | x^2 + ny^2\\), then p can be expressed as such." },
            { type: "math", content: "Reciprocity Step: If \\( p \\equiv 1 \\mod 4 \\), then p divides a sum of squares." },
        ],
    }
];

const links: LinkData[] = [
    // { source: "Primes of the form x^2 + ny^2", target: "Primes of the form x^2 + y^2", value: 1 },
    // { source: "Primes of the form x^2 + y^2", target: "Infinite Descent", value: 1 },
    // { source: "Infinite Descent", target: "Lemma 1.4", value: 1 },
    // { source: "Lemma 1.4", target: "Goldbach classical identity", value: 1 },
    // { source: "Primes of the form x^2 + y^2", target: "Reciprocity Step", value: 1 },
    // { source: "Reciprocity Step", target: "Fermat's Little Theorem", value: 1 },
];

export { nodes, links };