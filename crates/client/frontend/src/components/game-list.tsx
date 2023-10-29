"use client";

import { rspc } from "@/lib/rspc";
import {
    CircleLoader,
    ClimbingBoxLoader,
    MoonLoader,
    PuffLoader,
    RotateLoader
} from "react-spinners";

export default function GamesList() {
    const { data, isLoading, error } = rspc.useQuery(["games"]);

    return (
        <>
            <ClimbingBoxLoader
                loading={isLoading}
                size={50}
                cssOverride={{
                    position: "absolute",
                    top: "50%",
                    left: "50%",
                    transform: "translate(-50%, -50%)"
                }}
            />
            {data &&
                Object.entries(data).map(([key, game]) => (
                    <div key={key}>
                        <h2>{game.info.name}</h2>
                        <p>{JSON.stringify(game.status)}</p>
                    </div>
                ))}
            {error && JSON.stringify(error)}
        </>
    );
}
