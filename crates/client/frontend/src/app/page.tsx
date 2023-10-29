"use client";

import { NextPage } from "next";
import GamesList from "@/components/game-list";

const UsingUseQuery: NextPage = () => {
    return (
        <>
            <h1>Bramlett&apos;s Totally Awesome Game Launcher</h1>
            <GamesList />
        </>
    );
};

export default UsingUseQuery;
