import { NextPage } from "next";
import GamesList from "@/components/game-list";
import { Suspense } from "react";
import { ClimbingBoxLoader } from "react-spinners";

const Index: NextPage = () => {
    return (
        <>
            <h1>Bramlett&apos;s Totally Awesome Game Launcher</h1>
            <Suspense fallback={<ClimbingBoxLoader />}>
                <GamesList />
            </Suspense>
        </>
    );
};

export default Index;
