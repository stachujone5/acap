"use client";

import { Button } from "@/ui/button";
import { Toggle } from "@/ui/toggle";
import { useState } from "react";
import { dialog } from "@tauri-apps/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { setConfigSavePath } from "@/utils/bindings";
import { CONFIG_QUERY_KEY, useConfig } from "@/utils/useConfig";
import { Skeleton } from "@/ui/skeleton";

const handleSavePathChange = async () => {
	const pathSelectedByUser = await dialog.open({
		directory: true,
	});

	if (typeof pathSelectedByUser !== "string") {
		throw new Error("Invalid files / folders selected!");
	}

	return setConfigSavePath(pathSelectedByUser);
};

const Settings = () => {
	const [pressed, setPressed] = useState(false);
	const [key, setKey] = useState("F10");

	const queryClient = useQueryClient();
	const { data } = useConfig();
	const { mutate } = useMutation({
		mutationFn: handleSavePathChange,
		onSuccess: (newConfig) => queryClient.setQueryData(CONFIG_QUERY_KEY, newConfig),
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
				<Button onClick={() => mutate()} className="w-fit">
					Change save directory
				</Button>
				{data ? (
					<p>{data.save_path}</p>
				) : (
					<Skeleton className="w-1/3">
						<p className="text-transparent">Loading</p>
					</Skeleton>
				)}
			</div>
			{	data ? (
				<p>Recording duration: {data.recording_duration_in_secs} seconds</p>
			) : (
				<Skeleton className="w-1/3">
					<p className="text-transparent">Loading</p>
				</Skeleton>
			)}
		</div>
	);
};

export default Settings;
