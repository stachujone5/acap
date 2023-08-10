"use client";

import { Button } from "@/ui/button";
import { Toggle } from "@/ui/toggle";
import { useState } from "react";
import { dialog } from "@tauri-apps/api";
import { useQuery } from "@tanstack/react-query";
import { getSaveDir } from "@/utils/bindings";

const Settings = () => {
	const [pressed, setPressed] = useState(false);
	const [key, setKey] = useState("F10");
	const { data } = useQuery({
		queryKey: ["directory"],
		queryFn: getSaveDir,
	});

	const handleKeyDown = (e: React.KeyboardEvent<HTMLButtonElement>) => {
		setKey(e.key);
	};

	return (
		<div className="flex flex-col gap-6">
			<div className="flex items-center gap-4">
				<h3 className="text-2xl font-semibold tracking-tight">Start recording</h3>
				<Toggle
					pressed={pressed}
					onPressedChange={(isPressed) => setPressed(isPressed)}
					variant="outline"
					onKeyDown={handleKeyDown}
				>
					<div className="flex items-center justify-center">{key}</div>
				</Toggle>

				{pressed && <p>Listening for keyboard input</p>}
			</div>

			<div className="flex items-center gap-4">
				<Button
					onClick={() =>
						void dialog
							.open({
								directory: true,
							})
							.then((r) => console.log(r))
					}
					className="w-fit"
				>
					Change save directory
				</Button>
				<p>{data}</p>
			</div>
		</div>
	);
};

export default Settings;
