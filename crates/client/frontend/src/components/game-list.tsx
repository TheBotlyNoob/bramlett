"use client";

import { Game } from "@/lib/bindings";
import { rspc } from "@/lib/rspc";
import { Button, List } from "antd";
import { ClimbingBoxLoader } from "react-spinners";

export default function GamesList() {
    const { data, isLoading, error } = rspc.useQuery(["games"]);
    const { mutate } = rspc.useMutation("downloadGame");

    const gameStatus = (game: Game) => {
        switch (game.status) {
            case "NotDownloaded":
                return (
                    <Button
                        onClick={() => {
                            mutate(game.info.id);
                        }}
                    >
                        Download
                    </Button>
                );
            case "Downloading":
                return <Button>Cancel</Button>;
            case "Running":
                return <p>Running...</p>;
            case "Stopped":
                return <Button>Start</Button>;
        }
    };

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

            {data && (
                <List
                    bordered
                    dataSource={Object.entries(data)}
                    renderItem={([_, game]) => (
                        <List.Item>
                            <h2>{game.info.name}</h2>
                            <p>{gameStatus(game)}</p>
                        </List.Item>
                    )}
                />
            )}

            {error && JSON.stringify(error)}
        </>
    );
}
