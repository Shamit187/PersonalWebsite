"use client";

import Link from "next/link";

export default function Home() {
    return (
        <div className="flex flex-col space-y-2 p-4">
            <h1>Mind Map Lists</h1>
            <div className="flex flex-col space-y-1 p-4">
                <Link href="/map/Topology">Topology</Link>
                <Link href="/map/Philosophy">Philosophy</Link>
                <Link href="/map/ThermalPhysics">Thermal Physics</Link>
            </div>
        </div>
    );
}
