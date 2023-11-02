"use client";

import GamesList from "@/components/game-list";
import { Suspense } from "react";
import { ClimbingBoxLoader } from "react-spinners";

export default function Index() {
    return (
        <>
            <h1>Bramlett&apos;s Totally Awesome Game Launcher</h1>
            <Suspense fallback={<ClimbingBoxLoader />}>
                <GamesList />
            </Suspense>
        </>
    );
}
