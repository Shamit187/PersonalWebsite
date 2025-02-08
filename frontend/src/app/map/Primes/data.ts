import { NodeData, LinkData } from "../TopicGraph";

const nodes: NodeData[] = [
    {
        id: "Primes of the form x^2 + ny^2",
        group: 1,
        details: `
            <div class="full-content">
                <h1 class="title-content">Primes of the form x<sup>2</sup> + ny<sup>2</sup></h1>
                <p>Generalizes Euler's two-step strategy (Descent and Reciprocity).</p>
                <p>Uses quadratic forms to determine representability.</p>
                <p>For some values of n, naive descent arguments fail, requiring advanced techniques.</p>
            </div>
        `,
    },
    {
        id: "Primes of the form x^2 + y^2",
        group: 1,
        details: `
            <div class="full-content">
                <h3 class="title-content">Primes of the form x<sup>2</sup> + y<sup>2</sup></h3>
                <p>Euler's proof: If p = x<sup>2</sup> + y<sup>2</sup>, then p ≡ 1 mod 4 follows from congruences.</p>
                <p><b>Descent Step:</b> If p | x<sup>2</sup> + y<sup>2</sup>, then p can be expressed as such.</p>
                <p><b>Reciprocity Step:</b> If p ≡ 1 mod 4, then p divides a sum of squares.</p>
            </div>
        `,
    },
    {
        id: "Infinite Descent",
        group: 1,
        details: `
            <div class="full-content">
                <h3 class="title-content">Infinite Descent</h3>
                <p>If p | x2 + y2, gcd(x, y) = 1, then p can be written as x2 + y2 for some possibly different x, y..</p>
                <p>Leads to an infinite decreasing sequence, which is impossible.</p>
                <h4>Proof:</h4>
                <p>We have N = a<sup>2</sup> + b<sup>2</sup> and gcd(a, b) = 1. Let p be a prime divisor of N.</p>
                <p>We can find the smallest a' and b' such that a' ≡ a mod p and b' ≡ b mod p.</p>
                <p>|a'| \< p/2 and |b'| \< p/2 </p>
                <p>Then a'<sup>2</sup> + b'<sup>2</sup> \< p<sup>2</sup>/2 or N \< p<sup>2</sup> </p>
                <p>q is a prime divisor of N and q < p (Or else N \> p<sup>2</sup>).</p>
                <p>if q = x<sup>2</sup> + y<sup>2</sup> then according to Lemma 1.4, p = c<sup>2</sup> + d<sup>2</sup>.</p>
                <p>if not we can do the same thing with N/q = N and q = p and get a smaller prime again and again.</p>
                <p>But that is impossibl since there are only finitely many primes.</p>
                <p>Therefore, p = c<sup>2</sup> + d<sup>2</sup> is a sum of two squares.</p>
            </div>
        `,
    },
    {
        id: "Goldbach classical identity",
        group: 1,
        details: `
            <div class="full-content">
                <h3 class="title-content">Goldbach classical identity</h3>
                <p>(x<sup>2</sup> + y<sup>2</sup>)(z<sup>2</sup> + w<sup>2</sup>) = (xz ± yw)<sup>2</sup> + (xw ∓ yz)<sup>2</sup></p>
                <p>Key tool in proofs involving sums of squares.</p>
            </div>
        `,
    },
    {
        id: "Lemma 1.4",
        group: 1,
        details: `
            <div class="full-content">
                <h3 class="title-content">Lemma 1.4</h3>
                <p>If N = a<sup>2</sup> + b<sup>2</sup> and q = x<sup>2</sup> + y<sup>2</sup> is a prime divisor of N, then N/q is also a sum of two squares.</p>
                <p>Used in descent arguments for primes of the form x<sup>2</sup> + ny<sup>2</sup>.</p>
            </div>
        `,
    },
    {
        id: "Reciprocity Step",
        group: 1,
        details: `
            <div class="full-content">
                <h3 class="title-content">Reciprocity Step</h3>
                <p>If p ≡ 1 mod 4, then p divides x<sup>2</sup> + y<sup>2</sup> for some x, y.</p>
                <p>Uses Fermat’s Little Theorem: (x<sup>2k</sup> - 1)(x<sup>2k</sup> + 1) ≡ 0 mod p.</p>
                <p>Euler originally proved it using finite differences.</p>
            </div>
        `,
    },
    {
        id: "Fermat's Little Theorem",
        group: 1,
        details: `
            <div class="full-content">
                <h3 class="title-content">Fermat's Little Theorem</h3>
                <p>If p is prime and p ∤ a, then a<sup>(p-1)</sup> ≡ 1 mod p.</p>
                <p>Used in proving the Reciprocity Step.</p>
            </div>
        `,
    }
];

const links: LinkData[] = [
    { source: "Primes of the form x^2 + ny^2", target: "Primes of the form x^2 + y^2", value: 1 },
    { source: "Primes of the form x^2 + y^2", target: "Infinite Descent", value: 1 },
    { source: "Infinite Descent", target: "Lemma 1.4", value: 1 },
    { source: "Lemma 1.4", target: "Goldbach classical identity", value: 1 },
    { source: "Primes of the form x^2 + y^2", target: "Reciprocity Step", value: 1 },
    { source: "Reciprocity Step", target: "Fermat's Little Theorem", value: 1 },
];

export { nodes, links };