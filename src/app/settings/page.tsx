"use client";

import { Button } from "@/ui/button";
import { Toggle } from "@/ui/toggle";
import { useRef, useState } from "react";
import { dialog } from "@tauri-apps/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { updateConfigKey } from "@/utils/bindings";
import { CONFIG_QUERY_KEY, useConfig } from "@/utils/useConfig";
import { Skeleton } from "@/ui/skeleton";
import { Input } from "@/ui/input";
import { Label } from "@/ui/label";
import { z } from "zod";

const handleSavePathChange = async () => {
	const pathSelectedByUser = await dialog.open({
		directory: true,
	});

	if (typeof pathSelectedByUser !== "string") {
		throw new Error("Invalid files / folders selected!");
	}

	return updateConfigKey({ SavePath: pathSelectedByUser });
};

const handleRecordingDurationChange = (newRecordingDuration: number) => {
	return updateConfigKey({ RecordingDurationInSecs: newRecordingDuration });
};

const Settings = () => {
	const [pressed, setPressed] = useState(false);
	const [key, setKey] = useState("F10");
	const recordingDurationInputRef = useRef<HTMLInputElement>(null);

	const queryClient = useQueryClient();
	const { data: config } = useConfig();

	const { mutate: changeSavePath } = useMutation({
		mutationFn: handleSavePathChange,
		onSuccess: (newConfig) => queryClient.setQueryData(CONFIG_QUERY_KEY, newConfig),
	});

	const { mutate: changeRecordingDuration, isLoading: isRecordingDurationLoading } = useMutation({
		mutationFn: handleRecordingDurationChange,
		onSuccess: (newConfig) => queryClient.setQueryData(CONFIG_QUERY_KEY, newConfig),
	});

	const handleFormSubmit = (e: React.FormEvent<HTMLFormElement>) => {
		const inputValue = recordingDurationInputRef.current?.value;

		const parsedValue = z.coerce.number().int().min(1).max(600).safeParse(inputValue);

		if (parsedValue.success) {
			changeRecordingDuration(parsedValue.data);
		}

		e.preventDefault();
	};

	const handleKeyDown = (e: React.KeyboardEvent<HTMLButtonElement>) => {
		setKey(e.key);
	};

	return (
		<div className="flex flex-col gap-6">
			<div className="flex items-center gap-4">
				<p className="text-xl font-semibold tracking-tight">Start recording hotkey</p>
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

			{config ? (
				<form className="flex h-10 w-2/3 items-center gap-4" onSubmit={handleFormSubmit}>
					<Label htmlFor="recording_duration">Recording duration in seconds</Label>
					<Input
						ref={recordingDurationInputRef}
						id="recording_duration"
						className="w-20"
						defaultValue={config.recording_duration_in_secs}
					/>
					<Button disabled={isRecordingDurationLoading} type="submit">
						Save
					</Button>
				</form>
			) : (
				<Skeleton className="h-10 w-2/3" />
			)}

			<div className="flex h-10 items-center gap-4">
				<p className="text-xl font-semibold tracking-tight">Save directory</p>

				{config ? (
					<p className="inline-flex h-10 items-center justify-center rounded-md border border-input bg-background px-4 py-2 text-sm font-medium">
						{config.save_path}
					</p>
				) : (
					<Skeleton className="h-10 w-1/3" />
				)}

				<Button disabled={!Boolean(config)} onClick={() => changeSavePath()}>
					Change
				</Button>
			</div>
		</div>
	);
};

export default Settings;
