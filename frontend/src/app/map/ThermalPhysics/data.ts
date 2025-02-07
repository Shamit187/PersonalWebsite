import { NodeData, LinkData } from "../TopicGraph";
const nodes: NodeData[] = [
    { id: "Thermal Physics", group: 1, details: "The study of heat, temperature, and energy transfer." },
    { id: "Thermodynamics", group: 2, details: "Branch of physics that deals with heat, work, and energy." },
    { id: "Statistical Mechanics", group: 2, details: "Describes thermodynamic systems using probability theory." },
    { id: "Heat Transfer", group: 2, details: "Mechanisms of heat flow: conduction, convection, and radiation." },
    { id: "First Law of Thermodynamics", group: 3, details: "Energy cannot be created or destroyed, only transferred or converted." },
    { id: "Second Law of Thermodynamics", group: 3, details: "Entropy of an isolated system always increases." },
    { id: "Third Law of Thermodynamics", group: 3, details: "Entropy approaches a constant value as temperature approaches absolute zero." },
    { id: "Entropy", group: 4, details: "A measure of disorder in a system." },
    { id: "Internal Energy", group: 4, details: "The total energy contained within a system." },
    { id: "Microstates & Macrostates", group: 5, details: "Statistical descriptions of the configurations of a system." },
    { id: "Boltzmann Distribution", group: 5, details: "Probability distribution of states in thermal equilibrium." },
    { id: "Conduction", group: 6, details: "Heat transfer through direct contact." },
    { id: "Convection", group: 6, details: "Heat transfer through fluid movement." },
    { id: "Radiation", group: 6, details: "Heat transfer through electromagnetic waves." }
];

const links: LinkData[] = [
    { source: "Thermal Physics", target: "Thermodynamics", value: 1 },
    { source: "Thermal Physics", target: "Statistical Mechanics", value: 1 },
    { source: "Thermal Physics", target: "Heat Transfer", value: 1 },
    { source: "Thermodynamics", target: "First Law of Thermodynamics", value: 1 },
    { source: "Thermodynamics", target: "Second Law of Thermodynamics", value: 1 },
    { source: "Thermodynamics", target: "Third Law of Thermodynamics", value: 1 },
    { source: "Second Law of Thermodynamics", target: "Entropy", value: 1 },
    { source: "First Law of Thermodynamics", target: "Internal Energy", value: 1 },
    { source: "Statistical Mechanics", target: "Microstates & Macrostates", value: 1 },
    { source: "Statistical Mechanics", target: "Boltzmann Distribution", value: 1 },
    { source: "Heat Transfer", target: "Conduction", value: 1 },
    { source: "Heat Transfer", target: "Convection", value: 1 },
    { source: "Heat Transfer", target: "Radiation", value: 1 }
];

export { nodes, links };
