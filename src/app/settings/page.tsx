"use client";

import { Toggle } from "@/ui/toggle";
import { useState } from "react";

const Settings = () => {
	const [pressed, setPressed] = useState(false);
	const [key, setKey] = useState("F10");

	const handleKeyDown = (e: React.KeyboardEvent<HTMLButtonElement>) => {
		setKey(e.key);
	};

	return (
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
	);
};

export default Settings;
