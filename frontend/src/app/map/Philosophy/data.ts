import { NodeData, LinkData } from "../TopicGraph";
const nodes: NodeData[] = [
    { id: "Philosophy", group: 1, details: "The study of fundamental questions about existence, knowledge, ethics, and reasoning." },
    { id: "Metaphysics", group: 2, details: "The study of reality, existence, and the nature of being." },
    { id: "Epistemology", group: 2, details: "The study of knowledge, belief, and justification." },
    { id: "Ethics", group: 2, details: "The study of morality and principles of right and wrong." },
    { id: "Logic", group: 2, details: "The study of reasoning, arguments, and inference." },
    { id: "Aesthetics", group: 2, details: "The study of beauty, art, and taste." },
    { id: "Ontology", group: 3, details: "A branch of metaphysics that deals with the nature of being and existence." },
    { id: "Free Will & Determinism", group: 3, details: "Explores whether human actions are determined or freely chosen." },
    { id: "Rationalism", group: 4, details: "The belief that knowledge is gained through reason and logic." },
    { id: "Empiricism", group: 4, details: "The belief that knowledge is derived from sensory experience." },
    { id: "Deontology", group: 5, details: "Ethical theory that focuses on rules and duties." },
    { id: "Utilitarianism", group: 5, details: "Ethical theory that focuses on consequences and maximizing happiness." },
    { id: "Virtue Ethics", group: 5, details: "Ethical theory that focuses on character and moral virtues." },
    { id: "Formal Logic", group: 6, details: "A system of reasoning based on formal structures and symbols." },
    { id: "Informal Logic", group: 6, details: "The study of reasoning in natural language and everyday arguments." },
    { id: "Existentialism", group: 7, details: "A philosophical movement emphasizing individual freedom and meaning." },
    { id: "Phenomenology", group: 7, details: "The study of structures of consciousness and experience." },
    { id: "Postmodernism", group: 7, details: "A skeptical approach to narratives, truth, and knowledge claims." }
];

const links: LinkData[] = [
    { source: "Philosophy", target: "Metaphysics", value: 1 },
    { source: "Philosophy", target: "Epistemology", value: 1 },
    { source: "Philosophy", target: "Ethics", value: 1 },
    { source: "Philosophy", target: "Logic", value: 1 },
    { source: "Philosophy", target: "Aesthetics", value: 1 },
    { source: "Metaphysics", target: "Ontology", value: 1 },
    { source: "Metaphysics", target: "Free Will & Determinism", value: 1 },
    { source: "Epistemology", target: "Rationalism", value: 1 },
    { source: "Epistemology", target: "Empiricism", value: 1 },
    { source: "Ethics", target: "Deontology", value: 1 },
    { source: "Ethics", target: "Utilitarianism", value: 1 },
    { source: "Ethics", target: "Virtue Ethics", value: 1 },
    { source: "Logic", target: "Formal Logic", value: 1 },
    { source: "Logic", target: "Informal Logic", value: 1 },
    { source: "Metaphysics", target: "Existentialism", value: 1 },
    { source: "Metaphysics", target: "Phenomenology", value: 1 },
    { source: "Metaphysics", target: "Postmodernism", value: 1 }
];

export { nodes, links };
