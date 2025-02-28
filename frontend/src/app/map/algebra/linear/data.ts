import { NodeData, LinkData } from "../../TopicGraph";

const nodes: NodeData[] = [
    {
        id: "Linear Algebra",
        group: 1,
        title: "Linear Algebra",
        content: "Linear algebra is the branch of mathematics concerning linear equations, linear functions, and their representations through matrices and vector spaces.",
        image: []
    },
    {
        id: "Vector Spaces",
        group: 2,
        title: "Vector Spaces",
        content: "A vector space is a collection of vectors that can be added together and multiplied by scalars while satisfying certain axioms.",
        image: []
    },
    {
        id: "Matrices",
        group: 2,
        title: "Matrices",
        content: "A matrix is a rectangular array of numbers arranged in rows and columns. It represents linear transformations and systems of equations.",
        image: []
    },
    {
        id: "Eigenvalues and Eigenvectors",
        group: 3,
        title: "Eigenvalues and Eigenvectors",
        content: "Given a square matrix \\(A\\), an eigenvector \\(v\\) and an eigenvalue \\(\\lambda\\) satisfy the equation \\(A v = \\lambda v\\).",
        image: []
    },
    {
        id: "Determinants",
        group: 3,
        title: "Determinants",
        content: "The determinant of a matrix \\(A\\) is a scalar that describes certain properties of the matrix, such as invertibility. It is denoted as \\(\\det(A)\\).",
        image: []
    },
    {
        id: "Linear Transformations",
        group: 2,
        title: "Linear Transformations",
        content: "A linear transformation is a function \\(T: V \\to W\\) that satisfies \\(T(a v + b w) = a T(v) + b T(w)\\) for all scalars \\(a, b\\) and vectors \\(v, w\\).",
        image: []
    },
    {
        id: "Basis and Dimension",
        group: 2,
        title: "Basis and Dimension",
        content: "A basis of a vector space \\(V\\) is a set of linearly independent vectors that span \\(V\\). The number of basis vectors is the dimension of \\(V\\).",
        image: ["https://image.alwaysdumb.com/blog/back.jpg/450"]
    }
];

const links: LinkData[] = [
    { source: "Linear Algebra", target: "Vector Spaces", value: 1 },
    { source: "Linear Algebra", target: "Matrices", value: 1 },
    { source: "Matrices", target: "Eigenvalues and Eigenvectors", value: 1 },
    { source: "Matrices", target: "Determinants", value: 1 },
    { source: "Vector Spaces", target: "Linear Transformations", value: 1 },
    { source: "Vector Spaces", target: "Basis and Dimension", value: 1 }
];

export { nodes, links };