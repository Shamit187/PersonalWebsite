"use client";

import Link from "next/link";

export default function Home() {
    return (
        <div className="flex flex-col space-y-2 p-4">
            <h1>Mind Map Lists</h1>
            <div className="flex flex-col space-y-1 p-4">
                <Link href="/map/Topology" className="font-extralight">Topology (Demo)</Link>
                <Link href="/map/Philosophy" className="font-extralight">Philosophy (Demo)</Link>
                <Link href="/map/ThermalPhysics" className="font-extralight">Thermal Physics (Demo)</Link>
                <Link href="/map/Primes">Primes of the form x<sup>2</sup> + ny<sup>2</sup></Link>
            </div>
        </div>
    );
}
